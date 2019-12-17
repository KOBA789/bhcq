use libc;
use nix::{errno::Errno, sys::socket::{self, AddressFamily, SockFlag, SockProtocol, SockType, sockopt, SockAddr}};
use std::ffi::CString;
use std::os::unix::io::FromRawFd;
use std::net;
use std::error::Error as StdError;
use tokio::net::UdpSocket;
use tokio::prelude::*;
use dhcpv4;

#[tokio::main]
async fn main() -> Result<(), Box<dyn StdError>> {
    let sock = bind(CString::new("ens4").unwrap()).unwrap();
    do_loop(sock).await
}

fn bind(ifname: CString) -> Result<UdpSocket, Box<dyn StdError>> {
    let fd = socket::socket(
        AddressFamily::Inet,
        SockType::Datagram,
        SockFlag::empty(),
        Some(SockProtocol::Udp),
    )?;
    let addr: net::SocketAddr = net::SocketAddrV4::new(net::Ipv4Addr::new(0, 0, 0, 0), 67).into();
    socket::bind(fd, &SockAddr::Inet(socket::InetAddr::from_std(&addr)))?;
    socket::setsockopt(fd, sockopt::ReuseAddr, &true)?;
    unsafe {
        let res = libc::setsockopt(
            fd,
            libc::SOL_SOCKET,
            libc::SO_BINDTODEVICE,
            ifname.as_ptr() as *const libc::c_void,
            ifname.as_bytes().len() as u32,
        );
        Errno::result(res).map(drop)
    }?;
    let std_sock = unsafe { net::UdpSocket::from_raw_fd(fd) };
    let sock = UdpSocket::from_std(std_sock)?;
    sock.set_broadcast(true)?;
    Ok(sock)
}

async fn do_loop(mut sock: UdpSocket) -> Result<(), Box<dyn StdError>> {
    let mut buf = vec![0u8; 4096];
    loop {
        let (read, peer) = sock.recv_from(&mut buf).await?;
        let bytes = &buf[0..read];
        //println!("{:#x?}", bytes);
        let p = dhcpv4::DHCPv4::new(bytes).unwrap();
        println!("peer: {}", peer);
        //println!("{:?}", p);
        let mut typ = None;
        for opt in p.options().iter() {
            println!("code: {}, len: {}, payload: {:#04x?}", opt.code(), opt.len(), opt.payload());
            if opt.code() == 53 {
                typ = Some(opt.payload()[0]);
            }
        }
        match typ {
            Some(1) => {
                println!("=> DISCOVER");
                let mut r = dhcpv4::DHCPv4Buf::new(vec![0u8; 300]).unwrap();
                r.set_op_code(dhcpv4::OpCodes::BOOTREPLY);
                r.set_htype(1);
                r.set_hlen(6);
                r.set_hops(0);
                r.set_xid(p.xid());
                r.set_secs(0);
                r.set_flags(p.flags());
                r.set_ciaddr(net::Ipv4Addr::UNSPECIFIED);
                r.set_yiaddr(net::Ipv4Addr::new(192, 168, 44, 2));
                r.set_siaddr(net::Ipv4Addr::UNSPECIFIED);
                r.set_giaddr(p.giaddr());
                r.set_chaddr(p.chaddr());
                let mut options_buf = vec![
                    99, 130, 83, 99,
                    53, 1, 2,
                    1, 4, 255, 255, 255, 0,
                    3, 4, 192, 168, 44, 1,
                    6, 4, 8, 8, 8, 8,
                    51, 4, 0, 0, 0, 30,
                    54, 4, 192, 168, 44, 1,
                    255,
                ];
                options_buf.resize(64, 0);
                let options = dhcpv4::DHCPv4OptionsBuf::new(options_buf).unwrap();
                r.set_options(&options);
                let broadcast = net::SocketAddr::new(net::Ipv4Addr::BROADCAST.into(), 68);
                sock.send_to(r.as_slice(), broadcast).await?;
                println!("<= OFFER");
            },
            Some(3) => {
                println!("=> REQUEST");
                let mut r = dhcpv4::DHCPv4Buf::new(vec![0u8; 300]).unwrap();
                r.set_op_code(dhcpv4::OpCodes::BOOTREPLY);
                r.set_htype(1);
                r.set_hlen(6);
                r.set_hops(0);
                r.set_xid(p.xid());
                r.set_secs(0);
                r.set_flags(p.flags());
                r.set_ciaddr(p.ciaddr());
                r.set_yiaddr(net::Ipv4Addr::new(192, 168, 44, 2));
                r.set_siaddr(net::Ipv4Addr::UNSPECIFIED);
                r.set_giaddr(p.giaddr());
                r.set_chaddr(p.chaddr());
                let mut options_buf = vec![
                    99, 130, 83, 99,
                    53, 1, 5,
                    1, 4, 255, 255, 255, 0,
                    3, 4, 192, 168, 44, 1,
                    6, 4, 8, 8, 8, 8,
                    51, 4, 0, 0, 0, 30,
                    54, 4, 192, 168, 44, 1,
                    255,
                ];
                options_buf.resize(64, 0);
                let options = dhcpv4::DHCPv4OptionsBuf::new(options_buf).unwrap();
                r.set_options(&options);
                let broadcast = net::SocketAddr::new(net::Ipv4Addr::BROADCAST.into(), 68);
                let len = sock.send_to(r.as_slice(), broadcast).await?;
                println!("<= ACK {}", len);
            },
            _ => {
                println!("=> UNKNOWN type: {:?}", typ);
                /* noop */
            }
        }
        println!("========================");
    }
}

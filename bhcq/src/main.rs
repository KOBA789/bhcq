use libc;
use nix::{errno::Errno, sys::socket::{self, AddressFamily, SockFlag, SockProtocol, SockType, sockopt, SockAddr}};
use std::ffi::CString;
use std::os::unix::io::FromRawFd;
use std::net;
use std::error::Error as StdError;
use std::iter::FromIterator;
use std::collections::{HashMap, hash_map::RandomState};
use tokio::net::UdpSocket;
use tokio::prelude::*;
use dhcpv4::{self, message, Message, OpCode};
use dhcpv4::options::{
    message_type::*,
    subnet_mask::*,
    routers::*,
    lease_time::*,
    domain_name_servers::*,
};

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
    let bcast_sock_addr = net::SocketAddr::new(net::Ipv4Addr::BROADCAST.into(), 68);
    loop {
        let (read, peer) = sock.recv_from(&mut buf).await?;
        let bytes = &buf[0..read];
        let m = Message::new(bytes).ok_or("malformed size packet")?;
        let requ_hdr = m.header();
        let options = m.options();
        let options_iter = options.try_iter().ok_or("malformed magic cookie")?;
        let options_map = HashMap::<_, _, RandomState>::from_iter(options_iter.filter_map(Into::into));
        let message_type = options_map.get_message_type().ok_or("no message type")?;
        println!("{:?}", message_type);
        match message_type {
            MessageType::DHCPDISCOVER => {
                println!("=> DISCOVER");
                let mut bldr = message::Builder::new();
                {
                    let mut repl_hdr = bldr.header_mut();
                    repl_hdr.set_op_code(OpCode::BOOTREPLY);
                    repl_hdr.set_xid(requ_hdr.xid());
                    repl_hdr.set_flags(requ_hdr.flags());
                    repl_hdr.set_yiaddr(net::Ipv4Addr::new(192, 168, 44, 2));
                    repl_hdr.set_giaddr(requ_hdr.giaddr());
                    repl_hdr.chaddr().copy_from_slice(requ_hdr.chaddr());
                }
                {
                    let mut opts_bldr = bldr.options_builder();
                    opts_bldr.add_magic_cookie();
                    opts_bldr.add_message_type(MessageType::DHCPOFFER);
                    opts_bldr.add_subnet_mask(net::Ipv4Addr::new(255, 255, 255, 0));
                    opts_bldr.add_routers(&[net::Ipv4Addr::new(192, 168, 44, 1)]);
                    opts_bldr.add_lease_time(30);
                    opts_bldr.add_domain_name_servers(&[
                        net::Ipv4Addr::new(8, 8, 8, 8),
                        net::Ipv4Addr::new(8, 8, 4, 4),
                    ]);
                }
                let packet = bldr.finish();
                println!("{:02x?}", packet);
                sock.send_to(packet, bcast_sock_addr).await?;
                println!("<= OFFER");
            },
            MessageType::DHCPREQUEST => {
                println!("=> REQUEST");
                let mut bldr = message::Builder::new();
                {
                    let mut repl_hdr = bldr.header_mut();
                    repl_hdr.set_op_code(OpCode::BOOTREPLY);
                    repl_hdr.set_xid(requ_hdr.xid());
                    repl_hdr.set_flags(requ_hdr.flags());
                    repl_hdr.set_ciaddr(requ_hdr.ciaddr());
                    repl_hdr.set_yiaddr(net::Ipv4Addr::new(192, 168, 44, 2));
                    repl_hdr.set_giaddr(requ_hdr.giaddr());
                    repl_hdr.chaddr().copy_from_slice(requ_hdr.chaddr());
                }
                {
                    let mut opts_bldr = bldr.options_builder();
                    opts_bldr.add_magic_cookie();
                    opts_bldr.add_message_type(MessageType::DHCPACK);
                    opts_bldr.add_subnet_mask(net::Ipv4Addr::new(255, 255, 255, 0));
                    opts_bldr.add_routers(&[net::Ipv4Addr::new(192, 168, 44, 1)]);
                    opts_bldr.add_lease_time(30);
                    opts_bldr.add_domain_name_servers(&[
                        net::Ipv4Addr::new(8, 8, 8, 8),
                        net::Ipv4Addr::new(8, 8, 4, 4),
                    ]);
                }
                let packet = bldr.finish();
                println!("{:02x?}", packet);
                sock.send_to(packet, bcast_sock_addr).await?;
                println!("<= ACK");
            },
            _ => {

            },
        }
    }
}

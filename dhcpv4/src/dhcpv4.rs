use std::ops::Deref;
use std::borrow::{Borrow, ToOwned};
use std::fmt;
use std::net::Ipv4Addr;
use super::op_code::OpCode;
use super::options::DHCPv4Options;

/*
   0                   1                   2                   3
   0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
   +-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
   |     op (1)    |   htype (1)   |   hlen (1)    |   hops (1)    |
   +---------------+---------------+---------------+---------------+
   |                            xid (4)                            |
   +-------------------------------+-------------------------------+
   |           secs (2)            |           flags (2)           |
   +-------------------------------+-------------------------------+
   |                          ciaddr  (4)                          |
   +---------------------------------------------------------------+
   |                          yiaddr  (4)                          |
   +---------------------------------------------------------------+
   |                          siaddr  (4)                          |
   +---------------------------------------------------------------+
   |                          giaddr  (4)                          |
   +---------------------------------------------------------------+
   |                                                               |
   |                          chaddr  (16)                         |
   |                                                               |
   |                                                               |
   +---------------------------------------------------------------+
   |                                                               |
   |                          sname   (64)                         |
   +---------------------------------------------------------------+
   |                                                               |
   |                          file    (128)                        |
   +---------------------------------------------------------------+
   |                                                               |
   |                          options (variable)                   |
   +---------------------------------------------------------------+
 */

pub const HEADER_SIZE: usize = 236;
pub const MIN_PACKET_SIZE: usize = 300;

pub struct DHCPv4([u8]);
impl DHCPv4 {
    #[inline]
    pub fn new_unchecked<S: AsRef<[u8]> + ?Sized>(s: &S) -> &Self {
        unsafe { &*(s.as_ref() as *const [u8] as *const Self) }
    }

    #[inline]
    pub fn new<S: AsRef<[u8]> + ?Sized>(s: &S) -> Option<&Self> {
        if s.as_ref().len() >= MIN_PACKET_SIZE {
            Some(Self::new_unchecked(s))
        } else {
            None
        }
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    #[inline]
    pub fn op_code(&self) -> OpCode {
        OpCode(self.0[0])
    }

    #[inline]
    pub fn htype(&self) -> u8 {
        self.0[1]
    }

    #[inline]
    pub fn hlen(&self) -> u8 {
        self.0[2]
    }
    
    #[inline]
    pub fn hops(&self) -> u8 {
        self.0[3]
    }

    #[inline]
    pub fn xid(&self) -> u32 {
        u32::from_be_bytes([
            self.0[4],
            self.0[5],
            self.0[6],
            self.0[7],
        ])
    }

    #[inline]
    pub fn secs(&self) -> u16 {
        u16::from_be_bytes([
            self.0[8],
            self.0[9],
        ])
    }

    #[inline]
    pub fn flags(&self) -> u16 {
        u16::from_be_bytes([
            self.0[10],
            self.0[11],
        ])
    }

    #[inline]
    pub fn ciaddr(&self) -> Ipv4Addr {
        Ipv4Addr::new(
            self.0[12],
            self.0[13],
            self.0[14],
            self.0[15],
        )
    }

    #[inline]
    pub fn yiaddr(&self) -> Ipv4Addr {
        Ipv4Addr::new(
            self.0[16],
            self.0[17],
            self.0[18],
            self.0[19],
        )
    }

    #[inline]
    pub fn siaddr(&self) -> Ipv4Addr {
        Ipv4Addr::new(
            self.0[20],
            self.0[21],
            self.0[22],
            self.0[23],
        )
    }

    #[inline]
    pub fn giaddr(&self) -> Ipv4Addr {
        Ipv4Addr::new(
            self.0[24],
            self.0[25],
            self.0[26],
            self.0[27],
        )
    }

    #[inline]
    pub fn chaddr(&self) -> &[u8] {
        &self.0[28..28+16]
    }

    #[inline]
    pub fn sname(&self) -> &[u8] {
        &self.0[44..44+64]
    }

    #[inline]
    pub fn file(&self) -> &[u8] {
        &self.0[108..108+128]
    }

    pub fn options(&self) -> &DHCPv4Options {
        DHCPv4Options::new_unchecked(&self.0[HEADER_SIZE..])
    }
}

impl fmt::Debug for DHCPv4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "DHCPv4 {{ op: {:?}, htype: {:#04x}, hlen: {:#04x}, hops: {:#04x}, xid: {:#010x}, secs: {:#06x}, flags: {:#06x}, ciaddr: {}, yiaddr: {}, siaddr: {}, giaddr: {}, chaddr: {:x?}, sname: {:x?}, file: {:x?}, options: ... }}",
            self.op_code(),
            self.htype(),
            self.hlen(),
            self.hops(),
            self.xid(),
            self.secs(),
            self.flags(),
            self.ciaddr(),
            self.yiaddr(),
            self.siaddr(),
            self.giaddr(),
            self.chaddr(),
            self.sname(),
            self.file(),
        )
    }
}

pub struct DHCPv4Buf(Vec<u8>);
impl DHCPv4Buf {
    #[inline]
    pub fn new_unchecked(buf: Vec<u8>) -> Self {
        Self(buf)
    }

    #[inline]
    pub fn new(buf: Vec<u8>) -> Option<Self> {
        if buf.len() >= MIN_PACKET_SIZE {
            Some(Self::new_unchecked(buf))
        } else {
            None
        }
    }

    #[inline]
    pub fn set_op_code(&mut self, OpCode(op_code): OpCode) {
        self.0[0] = op_code;
    }

    #[inline]
    pub fn set_htype(&mut self, htype: u8) {
        self.0[0] = htype;
    }

    #[inline]
    pub fn set_hlen(&mut self, hlen: u8) {
        self.0[2] = hlen;
    }
    
    #[inline]
    pub fn set_hops(&mut self, hops: u8) {
        self.0[3] = hops;
    }

    #[inline]
    pub fn set_xid(&mut self, xid: u32) {
        self.0[4..4+4].copy_from_slice(&xid.to_be_bytes());
    }

    #[inline]
    pub fn set_secs(&mut self, secs: u16) {
        self.0[8..8+2].copy_from_slice(&secs.to_be_bytes());
    }

    #[inline]
    pub fn set_flags(&mut self, flags: u16) {
        self.0[10..10+2].copy_from_slice(&flags.to_be_bytes());
    }

    #[inline]
    pub fn set_ciaddr(&mut self, ciaddr: Ipv4Addr) {
        self.0[12..12+4].copy_from_slice(&ciaddr.octets());
    }

    #[inline]
    pub fn set_yiaddr(&mut self, yiaddr: Ipv4Addr) {
        self.0[16..16+4].copy_from_slice(&yiaddr.octets());
    }

    #[inline]
    pub fn set_siaddr(&mut self, siaddr: Ipv4Addr) {
        self.0[20..20+4].copy_from_slice(&siaddr.octets());
    }

    #[inline]
    pub fn set_giaddr(&mut self, giaddr: Ipv4Addr) {
        self.0[24..24+4].copy_from_slice(&giaddr.octets());
    }

    #[inline]
    pub fn set_chaddr(&mut self, chaddr: &[u8]) {
        self.0[28..28+16].copy_from_slice(chaddr);
    }

    #[inline]
    pub fn set_sname(&mut self, sname: &[u8]) {
        self.0[44..44+64].copy_from_slice(sname);
    }

    #[inline]
    pub fn set_file(&mut self, file: &[u8]) {
        self.0[108..108+128].copy_from_slice(file);
    }

    pub fn set_options(&mut self, options: &DHCPv4Options) {
        self.0.truncate(HEADER_SIZE);
        self.0.extend_from_slice(options.as_slice());
    }
}

impl ToOwned for DHCPv4 {
    type Owned = DHCPv4Buf;
    fn to_owned(&self) -> Self::Owned {
        DHCPv4Buf::new_unchecked(self.0.to_owned())
    }
}
impl Borrow<DHCPv4> for DHCPv4Buf {
    fn borrow(&self) -> &DHCPv4 {
        DHCPv4::new_unchecked(&self.0)
    }
}
impl Deref for DHCPv4Buf {
    type Target = DHCPv4;
    fn deref(&self) -> &DHCPv4 {
        self.borrow()
    }
}

#[cfg(test)]
mod tests {
    use super::DHCPv4;

    #[test]
    fn test_as_slice() {
        let buf = vec![0; 300];
        let p = DHCPv4::new(&buf).unwrap();
        assert!(p.as_slice() == &buf[..]);
    }
}

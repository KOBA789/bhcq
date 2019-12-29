use std::net::Ipv4Addr;

use super::op_code::OpCode;
use super::options;

pub struct Message<B>(B);

impl<T> Message<T> {
    pub const MIN_SIZE: usize = 300;
}

impl<'a> Message<&'a [u8]> {
    #[inline]
    pub fn new(buf: &'a [u8]) -> Option<Self> {
        if buf.len() >= Self::MIN_SIZE {
            return Some(Self(buf));
        }
        None
    }

    #[inline]
    pub fn as_slice(&self) -> &'a [u8] {
        self.0
    }

    #[inline]
    pub fn header(&self) -> Header<&[u8]> {
        Header::<&_>::new(&self.as_slice()[..Header::<()>::SIZE]).unwrap()
    }

    #[inline]
    pub fn options(&self) -> options::Options<&[u8]> {
        options::Options::new(&self.as_slice()[Header::<()>::SIZE..]).unwrap()
    }
}

pub struct Header<B>(B);

impl<T> Header<T> {
    pub const SIZE: usize = 236;
}

impl<'a> Header<&'a [u8]>
{
    #[inline]
    pub fn new(buf: &'a [u8]) -> Option<Self> {
        if buf.len() == Self::SIZE {
            return Some(Self(buf));
        }
        None
    }

    #[inline]
    pub fn as_slice(&self) -> &'a [u8] {
        self.0
    }

    #[inline]
    pub fn op_code(&self) -> OpCode {
        OpCode(self.as_slice()[0])
    }

    #[inline]
    pub fn htype(&self) -> u8 {
        self.as_slice()[1]
    }

    #[inline]
    pub fn hlen(&self) -> u8 {
        self.as_slice()[2]
    }

    #[inline]
    pub fn hops(&self) -> u8 {
        self.as_slice()[3]
    }

    #[inline]
    pub fn xid(&self) -> u32 {
        u32::from_be_bytes([
            self.as_slice()[4],
            self.as_slice()[5],
            self.as_slice()[6],
            self.as_slice()[7],
        ])
    }

    #[inline]
    pub fn secs(&self) -> u16 {
        u16::from_be_bytes([
            self.as_slice()[8],
            self.as_slice()[9],
        ])
    }

    #[inline]
    pub fn flags(&self) -> u16 {
        u16::from_be_bytes([
            self.as_slice()[10],
            self.as_slice()[11],
        ])
    }

    #[inline]
    pub fn ciaddr(&self) -> Ipv4Addr {
        Ipv4Addr::new(
            self.as_slice()[12],
            self.as_slice()[13],
            self.as_slice()[14],
            self.as_slice()[15],
        )
    }

    #[inline]
    pub fn yiaddr(&self) -> Ipv4Addr {
        Ipv4Addr::new(
            self.as_slice()[16],
            self.as_slice()[17],
            self.as_slice()[18],
            self.as_slice()[19],
        )
    }

    #[inline]
    pub fn siaddr(&self) -> Ipv4Addr {
        Ipv4Addr::new(
            self.as_slice()[20],
            self.as_slice()[21],
            self.as_slice()[22],
            self.as_slice()[23],
        )
    }

    #[inline]
    pub fn giaddr(&self) -> Ipv4Addr {
        Ipv4Addr::new(
            self.as_slice()[24],
            self.as_slice()[25],
            self.as_slice()[26],
            self.as_slice()[27],
        )
    }

    #[inline]
    pub fn chaddr(&self) -> &'a [u8] {
        &self.as_slice()[28..28+16]
    }

    #[inline]
    pub fn sname(&self) -> &'a [u8] {
        &self.as_slice()[44..44+64]
    }

    #[inline]
    pub fn file(&self) -> &'a [u8] {
        &self.as_slice()[108..108+128]
    }
}

impl<'a> Header<&'a mut [u8]> {
    #[inline]
    pub fn new(buf: &'a mut [u8]) -> Option<Self> {
        if buf.len() == Self::SIZE {
            return Some(Self(buf));
        }
        None
    }

    #[inline]
    pub fn as_mut_slice(&mut self) -> &mut [u8] {
        self.0
    }

    #[inline]
    pub fn set_op_code(&mut self, OpCode(op_code): OpCode) {
        self.as_mut_slice()[0] = op_code;
    }

    #[inline]
    pub fn set_htype(&mut self, htype: u8) {
        self.as_mut_slice()[1] = htype;
    }

    #[inline]
    pub fn set_hlen(&mut self, hlen: u8) {
        self.as_mut_slice()[2] = hlen;
    }

    #[inline]
    pub fn set_hops(&mut self, hops: u8) {
        self.as_mut_slice()[3] = hops;
    }

    #[inline]
    pub fn set_xid(&mut self, xid: u32) {
        self.as_mut_slice()[4..4+4].copy_from_slice(&xid.to_be_bytes());
    }

    #[inline]
    pub fn set_secs(&mut self, secs: u16) {
        self.as_mut_slice()[8..8+2].copy_from_slice(&secs.to_be_bytes());
    }

    #[inline]
    pub fn set_flags(&mut self, flags: u16) {
        self.as_mut_slice()[10..10+2].copy_from_slice(&flags.to_be_bytes());
    }

    #[inline]
    pub fn set_ciaddr(&mut self, ciaddr: Ipv4Addr) {
        self.as_mut_slice()[12..12+4].copy_from_slice(&ciaddr.octets());
    }

    #[inline]
    pub fn set_yiaddr(&mut self, yiaddr: Ipv4Addr) {
        self.as_mut_slice()[16..16+4].copy_from_slice(&yiaddr.octets());
    }

    #[inline]
    pub fn set_siaddr(&mut self, siaddr: Ipv4Addr) {
        self.as_mut_slice()[20..20+4].copy_from_slice(&siaddr.octets());
    }

    #[inline]
    pub fn set_giaddr(&mut self, giaddr: Ipv4Addr) {
        self.as_mut_slice()[24..24+4].copy_from_slice(&giaddr.octets());
    }

    #[inline]
    pub fn chaddr(&mut self) -> &mut [u8] {
        &mut self.as_mut_slice()[28..28+16]
    }

    #[inline]
    pub fn sname(&mut self) -> &mut [u8] {
        &mut self.as_mut_slice()[44..44+64]
    }

    #[inline]
    pub fn file(&mut self) -> &mut [u8] {
        &mut self.as_mut_slice()[108..108+128]
    }

    pub fn reset_to_default(&mut self) {
        self.set_op_code(OpCode::BOOTREQUEST);
        self.set_htype(1); // FIXME: considering ethernet only
        self.set_hlen(6); // FIXME: considering ethernet only
        self.set_hops(0);
        self.set_xid(0);
        self.set_ciaddr(Ipv4Addr::UNSPECIFIED);
        self.set_yiaddr(Ipv4Addr::UNSPECIFIED);
        self.set_siaddr(Ipv4Addr::UNSPECIFIED);
        self.set_giaddr(Ipv4Addr::UNSPECIFIED);
        self.chaddr().copy_from_slice(&[0u8; 16]);
        self.sname().copy_from_slice(&[0u8; 64]);
        self.file().copy_from_slice(&[0u8; 128]);
    }
}

pub struct Builder {
    buf: Vec<u8>,
}

impl Builder {
    pub fn new() -> Self {
        let buf = vec![0u8; Header::<()>::SIZE];
        let mut bldr = Self { buf };
        bldr.reset();
        bldr
    }

    pub fn header_mut(&mut self) -> Header<&mut [u8]> {
        Header::<&mut _>::new(&mut self.buf[..Header::<()>::SIZE]).unwrap()
    }

    pub fn options_builder(&mut self) -> options::Builder {
        options::Builder(AppendOnly(&mut self.buf))
    }

    fn fill_to_min_size(&mut self) {
        if self.buf.len() < Message::<()>::MIN_SIZE {
            self.buf.resize(Message::<()>::MIN_SIZE, 0);
        }
    }

    pub fn finish_owned(mut self) -> Vec<u8> {
        self.fill_to_min_size();
        self.buf
    }

    pub fn finish(&mut self) -> &[u8] {
        self.fill_to_min_size();
        &self.buf[..]
    }

    pub fn reset(&mut self) {
        self.buf.truncate(Header::<()>::SIZE);
        self.header_mut().reset_to_default();
    }
}

pub struct AppendOnly<'a>(&'a mut Vec<u8>);
impl<'a> AppendOnly<'a> {
    #[inline]
    pub fn append(&mut self, buf: &[u8]) {
        self.0.extend(buf);
    }
}

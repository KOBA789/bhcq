use std::ops::Deref;
use std::borrow::{Borrow, ToOwned};

pub const HEADER_SIZE: usize = 2;
pub const MIN_SIZE: usize = 2;

pub struct DHCPv4Option([u8]);
impl DHCPv4Option {
    #[inline]
    pub fn new_unchecked<S: AsRef<[u8]> + ?Sized>(s: &S) -> &Self {
        unsafe { &*(s.as_ref() as *const [u8] as *const Self) }
    }

    #[inline]
    pub fn new<S: AsRef<[u8]> + ?Sized>(s: &S) -> Option<&Self> {
        if s.as_ref().len() >= MIN_SIZE {
            Some(Self::new_unchecked(s))
        } else {
            None
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }

    pub fn code(&self) -> u8 {
        self.0[0]
    }

    pub fn len(&self) -> u8 {
        self.0[1]
    }

    pub fn payload(&self) -> &[u8] {
        &self.0[HEADER_SIZE..self.total_size()]
    }

    pub fn total_size(&self) -> usize {
        let len = self.len() as usize;
        HEADER_SIZE + len
    }
}

pub struct DHCPv4OptionBuf(Vec<u8>);
impl DHCPv4OptionBuf {
    #[inline]
    pub fn new_unchecked(buf: Vec<u8>) -> Self {
        Self(buf)
    }

    #[inline]
    pub fn new(buf: Vec<u8>) -> Option<Self> {
        if buf.len() >= MIN_SIZE {
            Some(Self::new_unchecked(buf))
        } else {
            None
        }
    }
}

impl ToOwned for DHCPv4Option {
    type Owned = DHCPv4OptionBuf;
    fn to_owned(&self) -> Self::Owned {
        DHCPv4OptionBuf::new_unchecked(self.0.to_owned())
    }
}
impl Borrow<DHCPv4Option> for DHCPv4OptionBuf {
    fn borrow(&self) -> &DHCPv4Option {
        DHCPv4Option::new_unchecked(&self.0)
    }
}
impl Deref for DHCPv4OptionBuf {
    type Target = DHCPv4Option;
    fn deref(&self) -> &DHCPv4Option {
        self.borrow()
    }
}

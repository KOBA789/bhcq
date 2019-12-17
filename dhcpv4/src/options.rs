use std::ops::Deref;
use std::borrow::{Borrow, ToOwned};
use std::iter::Iterator;
use std::cmp;
use super::option::DHCPv4Option;

pub const MAGIC_COOKIE_SIZE: usize = 4;
pub const MIN_SIZE: usize = 64;

pub struct DHCPv4Options([u8]);
impl DHCPv4Options {
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

    #[inline]
    pub fn iter(&self) -> Iter {
        Iter { buf: &self.as_slice()[MAGIC_COOKIE_SIZE..] }
    }
}

pub struct DHCPv4OptionsBuf(Vec<u8>);
impl DHCPv4OptionsBuf {
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

impl ToOwned for DHCPv4Options {
    type Owned = DHCPv4OptionsBuf;
    fn to_owned(&self) -> Self::Owned {
        DHCPv4OptionsBuf::new_unchecked(self.0.to_owned())
    }
}
impl Borrow<DHCPv4Options> for DHCPv4OptionsBuf {
    fn borrow(&self) -> &DHCPv4Options {
        DHCPv4Options::new_unchecked(&self.0)
    }
}
impl Deref for DHCPv4OptionsBuf {
    type Target = DHCPv4Options;
    fn deref(&self) -> &DHCPv4Options {
        self.borrow()
    }
}

pub struct Iter<'a> {
    buf: &'a [u8],
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a DHCPv4Option;
    fn next(&mut self) -> Option<Self::Item> {
        while self.buf.len() > 0 {
            if self.buf[0] == 0 {
                self.buf = &self.buf[1..];
                continue;
            }
            if self.buf[0] == 255 {
                return None;
            }
            if let Some(ret) = DHCPv4Option::new(self.buf) {
                let start = cmp::min(ret.total_size(), self.buf.len());
                self.buf = &self.buf[start..];
                return Some(ret);
            }
        }
        None
    }
}

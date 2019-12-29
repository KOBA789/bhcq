use std::net::Ipv4Addr;
use std::convert::TryInto;
use std::borrow::Borrow;
use super::super::option::Code;
use super::{Builder, OptionMap};

pub trait AddIpsExt {
    fn add_ips<I, A>(&mut self, code: Code, ips: I)
    where
        A: Borrow<Ipv4Addr>,
        I: IntoIterator<Item = A>,
        I::IntoIter: ExactSizeIterator,
    ;
}

impl<'a> AddIpsExt for Builder<'a> {
    fn add_ips<I, A>(&mut self, code: Code, ips: I)
    where
        A: Borrow<Ipv4Addr>,
        I: IntoIterator<Item = A>,
        I::IntoIter: ExactSizeIterator,
    {
        let Code(code) = code;
        let iter = ips.into_iter();
        self.append(&[code, (iter.len() * 4) as u8]);
        for ip in iter {
            self.append(&ip.borrow().octets());
        }
    }
}

pub trait GetIpsExt: OptionMap {
    fn get_ips(&self, code: Code) -> Option<IpsIter> {
        let value = self.get_option(code)?;
        let bytes = value.value()?;
        if bytes.len() % 4 == 0 {
            return Some(IpsIter(bytes));
        }
        None
    }
}

impl<T: OptionMap> GetIpsExt for T {}

pub struct IpsIter<'a>(&'a [u8]);
impl<'a> Iterator for IpsIter<'a> {
    type Item = Ipv4Addr;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let octets: [u8; 4] = self.0.get(..4)?.try_into().unwrap();
        self.0 = &self.0[4..];
        Some(Ipv4Addr::from(octets))
    }
}

impl<'a> ExactSizeIterator for IpsIter<'a> {
    fn len(&self) -> usize {
        self.0.len() / 4
    }
}

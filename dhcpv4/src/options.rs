use std::ops::{Deref, DerefMut};
use std::convert::TryInto;
use std::hash::BuildHasher;
use std::collections::{HashMap, BTreeMap};

use super::option::{self, Code};
use super::message::AppendOnly;

pub mod ip;
pub mod ips;
pub mod subnet_mask;
pub mod routers;
pub mod domain_name_servers;
pub mod bytes;
pub mod domain_name;
pub mod host_name;
pub mod message_type;
pub mod lease_time;

pub struct Options<B>(B);

impl<T> Options<T> {
    pub const MIN_SIZE: usize = 64;
    pub const MAGIC_COOKIE: [u8; 4] = [99, 130, 83, 99];
    pub const MAGIC_COOKIE_SIZE: usize = Self::MAGIC_COOKIE.len();
}

impl<'a> Options<&'a [u8]> {
    #[inline]
    pub fn new(buf: &'a [u8]) -> Option<Self> {
        if buf.len() >= Self::MIN_SIZE {
            return Some(Self(buf));
        }
        None
    }

    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        self.0.as_ref()
    }

    #[inline]
    pub fn magic_cookie(&self) -> Option<[u8; 4]> {
        self.as_slice()
            .get(..Self::MAGIC_COOKIE_SIZE)
            .and_then(|s| s.try_into().ok())
    }

    #[inline]
    pub fn is_magic_cookie_valid(&self) -> bool {
        self.magic_cookie() == Some(Self::MAGIC_COOKIE)
    }

    #[inline]
    pub fn try_iter(&self) -> Option<Iter> {
        if !self.is_magic_cookie_valid() {
            return None;
        }
        self.as_slice()
            .get(Self::MAGIC_COOKIE_SIZE..)
            .map(Iter)
    }
}

pub struct Iter<'a>(&'a [u8]);

impl<'a> Iterator for Iter<'a> {
    type Item = option::Option<&'a [u8]>;

    fn next(&mut self) -> Option<Self::Item> {
        option::Option::read(self.0).map(|(opt, rest)| {
            self.0 = rest;
            opt
        })
    }
}

pub struct Builder<'a>(pub AppendOnly<'a>);

impl<'a> Builder<'a> {
    pub fn add_magic_cookie(&mut self) {
        self.append(&Options::<()>::MAGIC_COOKIE[..]);
    }
}

impl<'a> Extend<(&'a Code, &'a option::Value<&'a [u8]>)> for Builder<'a> {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (&'a Code, &'a option::Value<&'a [u8]>)>,
    {
        for (&Code(code), value) in iter {
            self.append(&[code]);
            self.append(value.as_slice());
        }
    }
}

impl<'a> Deref for Builder<'a> {
    type Target = AppendOnly<'a>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl<'a> DerefMut for Builder<'a> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

pub trait OptionMap {
    fn get_option(&self, code: Code) -> Option<&option::Value<&[u8]>>;
}

impl<'a, S> OptionMap for HashMap<Code, option::Value<&'a [u8]>, S>
where
    S: BuildHasher,
{
    fn get_option(&self, code: Code) -> Option<&option::Value<&[u8]>> {
        self.get(&code)
    }
}

impl<'a> OptionMap for BTreeMap<Code, option::Value<&'a [u8]>> {
    fn get_option(&self, code: Code) -> Option<&option::Value<&[u8]>> {
        self.get(&code)
    }
}

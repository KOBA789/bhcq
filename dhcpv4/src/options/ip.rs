use std::net::Ipv4Addr;
use std::convert::TryInto;
use super::super::option::Code;
use super::{Builder, OptionMap};

pub trait AddIpExt {
    fn add_ip(&mut self, code: Code, ip: Ipv4Addr);
}

impl<'a> AddIpExt for Builder<'a> {
    fn add_ip(&mut self, code: Code, ip: Ipv4Addr) {
        let Code(code) = code;
        self.append(&[code, 4]);
        self.append(&ip.octets());
    }
}

pub trait GetIpExt: OptionMap {
    fn get_ip(&self, code: Code) -> Option<Ipv4Addr> {
        let value = self.get_option(code)?;
        let bytes = value.value()?;
        let octets: [u8; 4] = bytes.try_into().ok()?;
        Some(Ipv4Addr::from(octets))
    }
}

impl<T: OptionMap> GetIpExt for T {}

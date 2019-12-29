use std::convert::TryInto;
use super::super::option::Code;
use super::{Builder, OptionMap};

pub trait AddLeaseTimeExt {
    fn add_lease_time(&mut self, time_in_secs: u32);
}

impl<'a> AddLeaseTimeExt for Builder<'a> {
    fn add_lease_time(&mut self, time_in_secs: u32) {
        let Code(code) = Code::IP_ADDRESS_LEASE_TIME;
        self.append(&[code, 4]);
        self.append(&time_in_secs.to_be_bytes());
    }
}

pub trait GetLeaseTimeExt: OptionMap {
    fn get_lease_time(&mut self) -> Option<u32> {
        let value = self.get_option(Code::IP_ADDRESS_LEASE_TIME)?;
        let bytes = value.value()?;
        Some(u32::from_be_bytes(bytes.try_into().ok()?))
    }
}

impl<T: OptionMap> GetLeaseTimeExt for T {}

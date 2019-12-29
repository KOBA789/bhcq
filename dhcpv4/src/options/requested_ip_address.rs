use std::net::Ipv4Addr;
use super::super::option::Code;
use super::ip::{AddIpExt, GetIpExt};

pub trait AddRequestedIpAddressExt: AddIpExt {
    fn add_requested_ip_address(&mut self, addr: Ipv4Addr);
}

impl<T: AddIpExt> AddRequestedIpAddressExt for T {
    fn add_requested_ip_address(&mut self, addr: Ipv4Addr) {
        self.add_ip(Code::REQUESTED_IP_ADDRESS, addr);
    }
}

pub trait GetRequestedIpAddressExt: GetIpExt {
    fn get_requested_ip_address(&self) -> Option<Ipv4Addr> {
        self.get_ip(Code::REQUESTED_IP_ADDRESS)
    }
}

impl<T: GetIpExt> GetRequestedIpAddressExt for T {}

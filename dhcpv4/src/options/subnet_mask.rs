use std::net::Ipv4Addr;
use super::super::option::Code;
use super::ip::{AddIpExt, GetIpExt};

pub trait AddSubnetMaskExt: AddIpExt {
    fn add_subnet_mask(&mut self, subnet_mask: Ipv4Addr);
}

impl<T: AddIpExt> AddSubnetMaskExt for T {
    fn add_subnet_mask(&mut self, subnet_mask: Ipv4Addr) {
        self.add_ip(Code::SUBNET_MASK, subnet_mask);
    }
}

pub trait GetSubnetMaskExt: GetIpExt {
    fn get_subnet_mask(&self) -> Option<Ipv4Addr> {
        self.get_ip(Code::SUBNET_MASK)
    }
}

impl<T: GetIpExt> GetSubnetMaskExt for T {}

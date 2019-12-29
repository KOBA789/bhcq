use std::net::Ipv4Addr;
use super::super::option::Code;
use super::ip::{AddIpExt, GetIpExt};

pub trait AddServerIdentifierExt: AddIpExt {
    fn add_server_identifier(&mut self, addr: Ipv4Addr);
}

impl<T: AddIpExt> AddServerIdentifierExt for T {
    fn add_server_identifier(&mut self, addr: Ipv4Addr) {
        self.add_ip(Code::SERVER_IDENTIFIER, addr);
    }
}

pub trait GetServerIdentifierExt: GetIpExt {
    fn get_server_identifier(&self) -> Option<Ipv4Addr> {
        self.get_ip(Code::SERVER_IDENTIFIER)
    }
}

impl<T: GetIpExt> GetServerIdentifierExt for T {}

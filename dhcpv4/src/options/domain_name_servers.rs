use std::net::Ipv4Addr;
use std::borrow::Borrow;
use super::super::option::Code;
use super::ips::{AddIpsExt, GetIpsExt, IpsIter};

pub trait AddDomainNameServersExt: AddIpsExt {
    fn add_domain_name_servers<I, A>(&mut self, servers: I)
    where
        A: Borrow<Ipv4Addr>,
        I: IntoIterator<Item = A>,
        I::IntoIter: ExactSizeIterator,
    {
        self.add_ips(Code::DOMAIN_NAME_SERVER, servers);
    }
}

impl<T: AddIpsExt> AddDomainNameServersExt for T {}

pub trait GetDomainNameExt: GetIpsExt {
    fn get_domain_name_servers(&self) -> Option<IpsIter> {
        self.get_ips(Code::DOMAIN_NAME_SERVER)
    }
}

impl<T: GetIpsExt> GetDomainNameExt for T {}

use std::net::Ipv4Addr;
use std::borrow::Borrow;
use super::super::option::Code;
use super::ips::{AddIpsExt, GetIpsExt, IpsIter};

pub trait AddRoutersExt: AddIpsExt {
    fn add_routers<I, A>(&mut self, routers: I)
    where
        A: Borrow<Ipv4Addr>,
        I: IntoIterator<Item = A>,
        I::IntoIter: ExactSizeIterator,
    {
        self.add_ips(Code::ROUTER, routers);
    }
}

impl<T: AddIpsExt> AddRoutersExt for T {}

pub trait GetRoutersExt: GetIpsExt {
    fn get_routers(&self) -> Option<IpsIter> {
        self.get_ips(Code::ROUTER)
    }
}

impl<T: GetIpsExt> GetRoutersExt for T {}

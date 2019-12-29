use super::super::option::Code;
use super::bytes::{AddBytesExt, GetBytesExt};

pub trait AddDomainNameExt: AddBytesExt {
    fn add_domain_name(&mut self, domain_name: &[u8]) {
        self.add_bytes(Code::DOMAIN_NAME, domain_name);
    }
}

impl<T: AddBytesExt> AddDomainNameExt for T {}

pub trait GetDomainNameExt: GetBytesExt {
    fn get_domain_name(&self) -> Option<&[u8]> {
        self.get_bytes(Code::DOMAIN_NAME)
    }
}

impl<T: GetBytesExt> GetDomainNameExt for T {}

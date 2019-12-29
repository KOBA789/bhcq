use super::super::option::Code;
use super::bytes::{AddBytesExt, GetBytesExt};

pub trait AddHostNameExt: AddBytesExt {
    fn add_host_name(&mut self, host_name: &[u8]) {
        self.add_bytes(Code::HOST_NAME, host_name);
    }
}

impl<T: AddBytesExt> AddHostNameExt for T {}

pub trait GetHostNameExt: GetBytesExt {
    fn get_host_name(&self) -> Option<&[u8]>;
}

impl<T: GetBytesExt> GetHostNameExt for T {
    fn get_host_name(&self) -> Option<&[u8]> {
        self.get_bytes(Code::HOST_NAME)
    }
}

use super::super::option::Code;
use super::{Builder, OptionMap};

pub trait AddBytesExt {
    fn add_bytes(&mut self, code: Code, bytes: &[u8]);
}

impl<'a> AddBytesExt for Builder<'a> {
    fn add_bytes(&mut self, code: Code, bytes: &[u8]) {
        let Code(code) = code;
        self.append(&[code, bytes.len() as u8]);
        self.append(bytes);
    }
}

pub trait GetBytesExt: OptionMap {
    fn get_bytes(&self, code: Code) -> Option<&[u8]> {
        let value = self.get_option(code)?;
        value.value()
    }
}

impl<T: OptionMap> GetBytesExt for T {}

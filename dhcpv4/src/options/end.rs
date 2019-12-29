use super::super::option::Code;
use super::{Builder, OptionMap};

pub trait AddEndExt {
    fn add_end(&mut self);
}

impl<'a> AddEndExt for Builder<'a> {
    fn add_end(&mut self) {
        let Code(code) = Code::END;
        self.append(&[code]);
    }
}

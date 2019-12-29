use std::option::Option as StdOption;

pub mod code;
pub use code::Code;

pub struct Option<B>(B);

impl<'a> Option<&'a [u8]>
{
    #[inline]
    pub fn read(buf: &'a [u8]) -> StdOption<(Option<&'a [u8]>, &'a [u8])> {
        if buf.len() >= 1 {
            let opt = Option(buf);
            match opt.code() {
                Code::PAD | Code::END => {
                    let (bytes, rest) = buf.split_at(1);
                    return Some((Option(bytes), rest));
                },
                _ => {
                    return opt.value().map(|value| {
                        let (bytes, rest) = buf.split_at(value.len() as usize + 2);
                        (Option(bytes), rest)
                    });
                },
            }
        }
        None
    }

    #[inline]
    pub fn as_slice(&self) -> &'a [u8] {
        self.0.as_ref()
    }

    #[inline]
    pub fn code(&self) -> Code {
        Code(self.as_slice()[0])
    }

    #[inline]
    pub fn value(&self) -> StdOption<Value<&'a [u8]>> {
        Value::new(&self.as_slice()[1..])
    }
}

pub struct Value<B>(B);

impl<'a> Value<&'a [u8]> {
    #[inline]
    pub fn new(buf: &'a [u8]) -> StdOption<Value<&'a [u8]>> {
        if buf.len() >= 1 {
            return Some(Value(buf));
        }
        None
    }

    #[inline]
    pub fn as_slice(&self) -> &'a [u8] {
        self.0.as_ref()
    }

    #[inline]
    pub fn len(&self) -> u8 {
        self.as_slice()[0]
    }

    #[inline]
    pub fn value(&self) -> StdOption<&'a [u8]> {
        let value_bytes = &self.as_slice()[1..];
        if value_bytes.len() >= self.len() as usize {
            return Some(value_bytes);
        }
        None
    }
}

impl<'a> Into<(Code, StdOption<Value<&'a [u8]>>)> for Option<&'a [u8]> {
    fn into(self) -> (Code, StdOption<Value<&'a [u8]>>) {
        let code = self.code();
        let value = self.value();
        (code, value)
    }
}

impl<'a> Into<StdOption<(Code, Value<&'a [u8]>)>> for Option<&'a [u8]> {
    fn into(self) -> StdOption<(Code, Value<&'a [u8]>)> {
        let (k, v) = self.into();
        v.map(|v| (k, v))
    }
}

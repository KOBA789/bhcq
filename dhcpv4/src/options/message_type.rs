use super::super::option::Code;
use super::OptionMap;
use super::Builder;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageType(pub u8);
impl MessageType {
    pub const DHCPDISCOVER: MessageType = MessageType(1);
    pub const DHCPOFFER: MessageType = MessageType(2);
    pub const DHCPREQUEST: MessageType = MessageType(3);
    pub const DHCPDECLINE: MessageType = MessageType(4);
    pub const DHCPACK: MessageType = MessageType(5);
    pub const DHCPNAK: MessageType = MessageType(6);
    pub const DHCPRELEASE: MessageType = MessageType(7);
    pub const DHCPINFORM: MessageType = MessageType(8);
}

pub trait AddMessageTypeExt {
    fn add_message_type(&mut self, typ: MessageType);
}

impl<'a> AddMessageTypeExt for Builder<'a> {
    fn add_message_type(&mut self, typ: MessageType) {
        let MessageType(typ) = typ;
        let Code(code) = Code::DHCP_MESSAGE_TYPE;
        self.append(&[code, 1, typ]);
    }
}

pub trait GetMessageTypeExt: OptionMap {
    fn get_message_type(&self) -> Option<MessageType> {
        let value = self.get_option(Code::DHCP_MESSAGE_TYPE)?;
        let bytes = value.value()?;
        if bytes.len() == 1 {
            return Some(MessageType(bytes[0]));
        }
        None
    }
}

impl<T: OptionMap> GetMessageTypeExt for T {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpCode(pub u8);

#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod OpCodes {
    use super::OpCode;

    pub const BOOTREQUEST: OpCode = OpCode(1);
    pub const BOOTREPLY: OpCode = OpCode(2);
}

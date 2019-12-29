#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct OpCode(pub u8);

impl OpCode {
    pub const BOOTREQUEST: OpCode = OpCode(1);
    pub const BOOTREPLY: OpCode = OpCode(2);
}

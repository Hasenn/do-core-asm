use std::str::FromStr;
#[derive(PartialEq, Debug)]
pub enum Instr {
    RegRegOp(OpCode, Register, Register),
    RegNumOp(OpCode, Register, u8),
}
#[derive(Debug)]
pub enum OpCodeParseErr {
    InvalidOpCode(String)
}

#[derive(PartialEq, Debug)]
pub enum OpCode {
    LD,
    ST,
    ADD,
    XOR
}

impl FromStr for OpCode {
    type Err = OpCodeParseErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LD" => Ok(Self::LD),
            "ST" => Ok(Self::ST),
            "ADD"=> Ok(Self::ADD),
            "XOR"=> Ok(Self::XOR),
            _=> Err(Self::Err::InvalidOpCode(
                    String::from_str(s).unwrap()
                ))
        }
    }
}
#[derive(PartialEq, Debug)]
pub enum Register {
    GeneralPurpose(u8),
    RIP,
    RFLAGS
}


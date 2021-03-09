use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub enum Instr {
    /// An instruction that uses only registers, such as:
    /// ```asm
    /// LD R0 R1
    /// ```
    RegRegOp(OpCode, Register, Register),
    /// An instruction that uses a register and an unsigned integer
    /// ```asm
    /// LSH R0 8
    /// ```
    /// the integer must be encodable into 4 bits
    /// otherwise this instruction will fail to assemble
    RegNumOp(OpCode, Register, u8),
}

#[derive(Debug)]
pub enum Error {
    InvalidOpCode(String)
}

#[derive(PartialEq, Debug)]
pub enum Register {
    GeneralPurpose(u8),
    RIP,
    RFLAGS
}

#[derive(PartialEq, Debug)]
pub enum OpCode {
    LD,
    ST,
    ADD,
    XOR
}

impl FromStr for OpCode {
    type Err = Error;
    // the parser calls this with uppercased strings
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opcode_from_string_decodes_uppercase() {
        assert_eq!(OpCode::from_str("LD").unwrap(),  OpCode::LD);
        assert_eq!(OpCode::from_str("ST").unwrap(),  OpCode::ST);
        assert_eq!(OpCode::from_str("ADD").unwrap(), OpCode::ADD);
        assert_eq!(OpCode::from_str("XOR").unwrap(), OpCode::XOR);
    }
}
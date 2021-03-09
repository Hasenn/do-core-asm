use std::str::FromStr;
use strum_macros::EnumString;

#[derive(PartialEq, Debug)]
pub enum Instr {
    /// An instruction that uses only registers, such as:
    /// ```asm
    /// LD R0 R1
    /// ```
    RegRegOp(Op, Register, Register),
    /// An instruction that uses a register and an unsigned integer
    /// ```asm
    /// LSH R0 8
    /// ```
    /// the integer must be encodable into 4 bits
    /// otherwise this instruction will fail to assemble
    RegNumOp(Op, Register, u8),
}

#[derive(Debug)]
pub enum Error {
    InvalidOp(String)
}

#[derive(PartialEq, Debug)]
pub enum Register {
    GeneralPurpose(u8),
    RIP,
    RFLAGS
}

#[derive(PartialEq, Debug, EnumString)]
// To avoid this duplication of do-core's code
// we'll need 
pub enum Op {
    LD,
    ST,
    ADD,
    XOR
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn op_from_string_decodes_uppercase() {
        assert_eq!(Op::from_str("LD").unwrap(),  Op::LD);
        assert_eq!(Op::from_str("ST").unwrap(),  Op::ST);
        assert_eq!(Op::from_str("ADD").unwrap(), Op::ADD);
        assert_eq!(Op::from_str("XOR").unwrap(), Op::XOR);
    }
}
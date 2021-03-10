use std::fmt::Debug;
use std::convert::Infallible;
use crate::instruction::{
    Instr,
    Register,
    Register::*,
    Op,
};
#[derive(Debug, PartialEq)]
pub enum Error {
    ArgOutOfBounds(String),
    SpecialRegisterError,
    NumArgumentError
}

trait ToByte {
    type Err : Debug;
    fn to_byte(&self) -> Result<u8, Self::Err>;
}

impl ToByte for Register {
    type Err = Error;
    fn to_byte(&self) -> Result<u8, Self::Err> {
        match self {
            GeneralPurpose(v) => Ok(*v),
            RIP => Err(Error::SpecialRegisterError),
            RFLAGS => Err(Error::SpecialRegisterError)
        }
    }
}
impl ToByte for Op {
    type Err = Infallible;
    fn to_byte(&self) -> Result<u8, Self::Err> {
        Ok(*self as u8)
    }
}

pub trait Encodable {
    type Err : Debug;
    fn encode(&self) -> Result<u16, Self::Err>;
}

impl Encodable for Instr {
    type Err = Error;
    fn encode(&self) -> Result<u16, Self::Err> {
        match self {
            Instr::RegRegOp(op, r0, r1) => join(
                op.to_byte().unwrap(),
                r0.to_byte()?,
                r1.to_byte()?
            ),
            Instr::RegNumOp(_,_,_) => Err(Error::NumArgumentError)
            
        }
    }
}

fn join(a: u8, upper_b: u8, lower_b : u8) -> Result<u16, Error> {
    if upper_b > 0xF {
        return Err(Error::ArgOutOfBounds("First operand out of bounds".to_owned()))
    }
    if lower_b > 0xF {
        return Err(Error::ArgOutOfBounds("Second operand out of bounds".to_owned()))
    }
    Ok(
        ((a as u16) << 8)
        | ((upper_b as u16) << 4)
        | (lower_b as u16)
    )
}

#[cfg(test)]
mod encode {
    use super::*;
    #[test]
    fn registers() {
        assert_eq!(
            GeneralPurpose(3).to_byte(),
            Ok(0x3)
        )
    }
    #[test]
    fn operation() {
        assert_eq!(
            Op::LD.to_byte(),
            Ok(0x00)
        )
    }
    #[test]
    fn instructions() {
        assert_eq!(
            Instr::RegRegOp(Op::LD,GeneralPurpose(0), GeneralPurpose(1)).encode(),
            Ok(0x0001)
        );
        assert_eq!(
            Instr::RegRegOp(Op::ADD,GeneralPurpose(3), GeneralPurpose(4)).encode(),
            Ok(0x0234)
        );
        assert_eq!(
            Instr::RegRegOp(Op::ST,GeneralPurpose(0), GeneralPurpose(1)).encode(),
            Ok(0x0101)
        );
        assert_eq!(
            Instr::RegRegOp(Op::XOR,GeneralPurpose(3), GeneralPurpose(4)).encode(),
            Ok(0x0334)
        );
    }
}

#[cfg(test)]
mod join {
    use super::*;
    #[test]
    fn valid_arguments_0x1234() {
        assert_eq!(
            join(0x12,0x3,0x4).unwrap(),
            0x1234
        )
    }
    #[test]
    fn valid_arguments_0x4321() {
        assert_eq!(
            join(0x43,0x2,0x1).unwrap(),
            0x4321
        )
    }

    #[test]
    fn invalid_arguments_op0_too_big() {
        assert!(
            join(0x12, 0x12, 0x1).is_err()
        )
    }
    #[test]
    fn invalid_arguments_op1_too_big() {
        assert!(
            join(0x12, 0x1, 0x10).is_err()
        )
    }
    
}
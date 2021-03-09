#[macro_use] extern crate lalrpop_util;
#[macro_use] extern crate strum_macros;

pub mod instruction;
lalrpop_mod!(pub grammar); // synthesized by LALRPOP

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hex_nums_parse_correctly() {
        assert_eq!(grammar::NumParser::new().parse("0xFf").unwrap(), 0xff);
        assert_eq!(grammar::NumParser::new().parse("0xff").unwrap(), 0xff);
    }

    #[test]
    fn bin_nums_parse_correctly() {
        assert_eq!(grammar::NumParser::new().parse("0b101010").unwrap(), 0b101010);
        assert!(grammar::NumParser::new().parse("0b101 010").is_err());
    }

    #[test]
    fn decimal_nums_parse_correctly() {
        assert_eq!(grammar::NumParser::new().parse("23").unwrap(), 23);
    }
    #[test]
    fn add_instructions_parse_correctly() {
        let instr = grammar::InstructionParser::new().parse("ADD R0 R1").unwrap();
        assert_eq!(instr, instruction::Instr::RegRegOp(
            instruction::Op::ADD,
            instruction::Register::GeneralPurpose(0),
            instruction::Register::GeneralPurpose(1)
        ));
    }
    #[test]
    fn ops_allow_free_capitalization() {
        let instr = grammar::InstructionParser::new().parse("Add R0 R1").unwrap();
        assert_eq!(instr, instruction::Instr::RegRegOp(
            instruction::Op::ADD,
            instruction::Register::GeneralPurpose(0),
            instruction::Register::GeneralPurpose(1)
        ));
        let instr = grammar::InstructionParser::new().parse("add R0 R1").unwrap();
        assert_eq!(instr, instruction::Instr::RegRegOp(
            instruction::Op::ADD,
            instruction::Register::GeneralPurpose(0),
            instruction::Register::GeneralPurpose(1)
        ));
        let instr = grammar::InstructionParser::new().parse("ADD R0 R1").unwrap();
        assert_eq!(instr, instruction::Instr::RegRegOp(
            instruction::Op::ADD,
            instruction::Register::GeneralPurpose(0),
            instruction::Register::GeneralPurpose(1)
        ));
    }
    #[test]
    fn xor_instructions_parse_correctly() {
        let instr = grammar::InstructionParser::new().parse("XOR R0 R1").unwrap();
        assert_eq!(instr, instruction::Instr::RegRegOp(
            instruction::Op::XOR,
            instruction::Register::GeneralPurpose(0),
            instruction::Register::GeneralPurpose(1)
        ));
    }

    #[test]
    fn simple_files_parse_without_error() {
        assert!(grammar::FileParser::new()
            .parse("Add r0 r1 ; some comment
            Ld  r2 r1


            LD r1 r2

            ST r2 r1").is_ok());
    }
}
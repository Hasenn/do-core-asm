#[macro_use] extern crate lalrpop_util;
pub mod instruction;
lalrpop_mod!(pub assembler); // synthesized by LALRPOP


#[test]
fn hex_nums_parse_correctly() {
    assert_eq!(assembler::NumParser::new().parse("0xFf").unwrap(), 0xff);
    assert_eq!(assembler::NumParser::new().parse("0xff").unwrap(), 0xff);
}

#[test]
fn bin_nums_parse_correctly() {
    assert_eq!(assembler::NumParser::new().parse("0b101010").unwrap(), 0b101010);
    assert!(assembler::NumParser::new().parse("0b101 010").is_err());
}

#[test]
fn decimal_nums_parse_correctly() {
    assert_eq!(assembler::NumParser::new().parse("23").unwrap(), 23);
}
#[test]
fn instructions_parse_correctly() {
    let instr = assembler::InstructionParser::new().parse("ADD R0 R1").unwrap();
    assert_eq!(instr, instruction::Instr::RegRegOp(
        instruction::OpCode::ADD,
        instruction::Register::GeneralPurpose(0),
        instruction::Register::GeneralPurpose(1)
    ))
}

fn main() {

}
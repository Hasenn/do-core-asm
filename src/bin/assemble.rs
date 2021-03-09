extern crate assembler;

use assembler::grammar::FileParser;

fn main() {
    print!("{:?}",FileParser::new().parse(
        "Ld r0 r1 ; comments
        St r0 r1 ; comments
        Ld r0 r1 ; comments
        Ld r0 r1 ; comments

        Ld r0 r1 ; comments
        // c style comments

        Ld r0 r1 ; comments
        /* moar
        c style
        comments */
        
        "
    ));
}
mod tests {
    use super::*;
    
    #[test]
    fn comments_are_ignored() {
        // todo consider using this https://github.com/mitsuhiko/insta
        // and not this mess of a test
        assert_eq!(
            FileParser::new().parse(
                "Ld r0 r1 ; comments
                St r0 r1 ; comments
                Ld r0 r1 ; comments
                Ld r0 r1 ; comments
        
                Ld r0 r1 ; comments
                // c style comments
        
                Ld r0 r1 ; comments
                /* moar
                c style
                comments */
                
                "),
                Ok(vec![
                    assembler::instruction::Instr::RegRegOp(assembler::instruction::Op::LD, assembler::instruction::Register::GeneralPurpose(0), assembler::instruction::Register::GeneralPurpose(1)),
                    assembler::instruction::Instr::RegRegOp(assembler::instruction::Op::ST, assembler::instruction::Register::GeneralPurpose(0), assembler::instruction::Register::GeneralPurpose(1)),
                    assembler::instruction::Instr::RegRegOp(assembler::instruction::Op::LD, assembler::instruction::Register::GeneralPurpose(0), assembler::instruction::Register::GeneralPurpose(1)),
                    assembler::instruction::Instr::RegRegOp(assembler::instruction::Op::LD, assembler::instruction::Register::GeneralPurpose(0), assembler::instruction::Register::GeneralPurpose(1)),
                    assembler::instruction::Instr::RegRegOp(assembler::instruction::Op::LD, assembler::instruction::Register::GeneralPurpose(0), assembler::instruction::Register::GeneralPurpose(1)),
                    assembler::instruction::Instr::RegRegOp(assembler::instruction::Op::LD, assembler::instruction::Register::GeneralPurpose(0), assembler::instruction::Register::GeneralPurpose(1))
                    ])
        )
    }
}

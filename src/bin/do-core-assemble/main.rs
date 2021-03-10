extern crate assembler;
#[macro_use] extern crate structopt;
use assembler::grammar::FileParser;
use assembler::instruction;
use assembler::encode::Encodable;
use assembler::instruction::Instr;

use std::fs::File;
use std::io::prelude::*;
use std::io::Read;
use std::io::{BufWriter, Write};

use std::path::{
    PathBuf
};

use structopt::StructOpt;
mod args;

const INITIAL_ALLOC_SIZE : usize = 16384;


fn main() -> Result<(), std::io::Error> {
    let opt = args::Opt::from_args();
    match opt.output {
        Some(output_path) => assemble_file(
            opt.input,
            output_path
            ),
        None => {
            let mut output = opt.input.clone();
            output.set_extension("out");
            assemble_file(
            opt.input,
            output
            )
        }   
    }
}


fn assemble_file(input_path : PathBuf, output_path : PathBuf) -> Result<(), std::io::Error> {
    let mut input_file = File::open(input_path)?;

    let mut input_string = String::with_capacity(INITIAL_ALLOC_SIZE);
    input_file.read_to_string(&mut input_string)?;

    let instructions : Vec<Instr> = FileParser::new().parse(&input_string).expect("Error while parsing file");

    let output_file = File::create(output_path).expect("couldn't create output file");
    let mut output_writer = BufWriter::new(output_file);

    for instruction in instructions {
        output_writer.write(&instruction.encode().unwrap().to_be_bytes()).expect("Error while encoding");
    }

    

    Ok(())
    
}





mod tests {
    use super::*;
    
    #[test]
    fn comments_are_ignored() {
        // todo consider using this https://github.com/mitsuhiko/insta
        // and not this mess of a test
        // or more use statements
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

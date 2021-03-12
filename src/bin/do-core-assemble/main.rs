extern crate assembler;
extern crate structopt;

use assembler::grammar::FileParser;
use assembler::encode::Encodable;
use assembler::instruction::Instr;

use std::fs::File;
use std::io::Read;
use std::io::{BufWriter, Write};

use std::path::{
    PathBuf
};

use structopt::StructOpt;
mod args;

// Constants

const INITIAL_ALLOC_SIZE : usize = 16384;


fn main() -> Result<(), std::io::Error> {
    let opt = args::Opt::from_args();
    run(opt)
}

pub fn run(opt: args::Opt) -> Result<(), std::io::Error>{
    match opt.output {
        Some(output_path) => assemble_file(
            opt.input,
            output_path
            ),
        None => {
            // set out path to in path with `out` extension
            let mut output = opt.input.clone();
            output.set_extension("out");
            assemble_file(
            opt.input,
            output
            )
        }   
    }
}


pub fn assemble_file(input_path : PathBuf, output_path : PathBuf) -> Result<(), std::io::Error> {
    // Read entire input file into a string
    let mut input_file = File::open(input_path)?;
    let mut input_string = String::with_capacity(INITIAL_ALLOC_SIZE);
    input_file.read_to_string(&mut input_string)?;

    // Parse input into a list of instructions
    let instructions : Vec<Instr> = FileParser::new().parse(&input_string)
        .expect("Error while parsing file");

    // open output file for buffered write
    let output_file = File::create(output_path)
        .expect("couldn't create output file");
    let mut output_writer = BufWriter::new(output_file);

    // encode and write all instructions to output
    for instruction in instructions {
        #[cfg(debug_assertions)]
        print!("{:#06x} ", &instruction.encode().unwrap());

        output_writer.write(
            &instruction.encode().unwrap().to_ne_bytes()
        ).expect("Error while encoding");
    }

    #[cfg(debug_assertions)]
    print!("\n");

    Ok(())
    
}




#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;
    use assembler::instruction::{
        Instr,
        Op::*,
        Register,


    };
    
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
                // c style line comments
        
                Ld r0 r1 ; comments
                
                "),
                Ok(vec![
                    Instr::RegRegOp(LD, Register::GeneralPurpose(0), Register::GeneralPurpose(1)),
                    Instr::RegRegOp(ST, Register::GeneralPurpose(0), Register::GeneralPurpose(1)),
                    Instr::RegRegOp(LD, Register::GeneralPurpose(0), Register::GeneralPurpose(1)),
                    Instr::RegRegOp(LD, Register::GeneralPurpose(0), Register::GeneralPurpose(1)),
                    Instr::RegRegOp(LD, Register::GeneralPurpose(0), Register::GeneralPurpose(1)),
                    Instr::RegRegOp(LD, Register::GeneralPurpose(0), Register::GeneralPurpose(1))
                    ])
        )
    }
}

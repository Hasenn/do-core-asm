extern crate assembler;

use assembler::grammar::FileParser;

fn main() {
    print!("{:?}",FileParser::new().parse(
        "Ld r0 r1 ; comments"
    ));
}
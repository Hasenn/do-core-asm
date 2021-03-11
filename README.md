# do-core-asm

This is an assembler that can parse a simple assembly language
for the do-core fantasy architecture.

## Installation

```sh
git clone https://github.com/Hasenn/do-core-asm.git
cargo install --path do-core-asm
# remove the repo if you need to
rm -rf do-core-asm
```
You can then use the binary `do-core-assemble`
```sh
do-core-assemble -i myfile.asm
```
To see the result you can use the `od -x` command
```
do-core-assemble -i example.asm && od -x example.out
```

As we use `main` as our git main branch, and cargo install expects `master` untill [this pull request](https://github.com/rust-lang/cargo/pull/9133) becomes part of a rust-lang release, we can't use `cargo install --git`.



## The language

As it is easier to explain by example, here is an example of valid do-core-asm code :

```asm

Ld r0 r1 ; comments :-)

add r0 r1
xor r1 r2
XOR R1 RIP ; capitalization is only important for special registers
add R1 R2  ; like RIP and RFLAGS

INC R1 3   ; this is not supported by do-core yet but might be
INC R1 1   ; integer litterals are only valid as second arguments 
INC R1 0b1010 ; they can be in binary with the 0b prefix
INC R1 0xF ; or in hex with the 0x prefix

LD    R1    R2 ; whitespace doesn't matter    
```

### Instructions

Instructions are of the form :

```asm
OPERATION ARG1 ARG2
```

With exactly two arguments. The first argument is always a register, The second argument can either be a 4-bits-representable unsigned integer, or a Register.

### Comments

C-style line comments :
```c
// comment
```
as well as asm style comments
```asm
; comment
```
are supported

### Registers

General purpose registers start with an 'r' followed by a number, like `r0` or `R1`

Special registers are `RIP` and `RFLAGS` and **must** be uppercase. they are not supported as of yet in instructions of the do-core architecture and will most likely only be usable at certain places in specific instructions.

## Command line


`assemble` accepts paths for in `-f` and out `-o` files

If no out path is given, it will default to the input path with an `out` extension

The output will be a binary file of back to back `u16` instructions. As of now they are native-endian encoded, but this could be parametrized (**TO BE SPECIFIED**)

For convenience we can also define modes where it outputs a readable stream like `0103 0204 0403` instead of a binary stream, possibly with prefixes for clarity `0x0103 0x0204 0x0403`. (**TO BE IMPLEMENTED**)

## API

The lib crate will expose a sensible api to parse do-core-asm into do-core bytecode.





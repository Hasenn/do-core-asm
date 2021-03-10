# do-core-asm

This is an assembler that can parse a simple assembly language
for the do-core fantasy architecture.

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

C-style comments :
```c
// comment
/* com
ment
*/
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

The output will be a binary file of back to back `u16` instructions. As of now it will be in big-endian order, but this could be parametrized (**TO BE SPECIFIED**)

For convenience we can also define modes where it outputs a readable stream like `0103 0204 0403` instead of a binary stream, possibly with prefixes for clarity `0x0103 0x0204 0x0403`.

## API

The lib crate will expose a sensible api to parse do-core-asm into do-core bytecode.





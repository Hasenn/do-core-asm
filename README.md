# do-core-asm

This is an assembler language that can parse a simple assembly language
for the do-core fantasy architecture.

## The language

As it is easier to explain by example, here is an example of valid do-core-asm code :

```asm

Ld r0 r1 ; comments :-)

add r0 r1
xor r1 r2
XOR R1 RIP ; capitalization is only important for special registers
add R1 R2  ; like RIP and RFLAGS



LD    R1    R2 ; whitespace doesn't matter    
```

### Instructions

Instructions are of the form :

```asm
OPERATION ARG1 ARG2
```

With exactly two arguments. The first argument is always a register, The second argument can either be a 4-bits-representable unsigned integer, or a Register.

### Comments

A Comment starts with `;` and ends on a newline

As of now only comments after an instructions are supported but this will change in the future.

### Registers

General purpose registers start with an 'r' followed by a number, like `r0` or `R1`

Special registers are `RIP` and `RFLAGS` and **must** be uppercase. thay are not supported as of yet in instructions of the do-core architecture and will most likely only be usable at certain places of certain instructions.

## Command line

As of now the `assemble` binary does nothing, but here is a specification :

`assemble` will accept paths for in `-f` and out `-o` files

If no out path is given, it will default to a sensible value or output to stdout (**TO BE SPECIFIED**)

If no in path is given, it will read from `stdin` ending on an END OF FILE character (`ctrl-D` in the terminal (IIRC))

The output will be a binary stream of `u16` instructions.

For convenience we can also define modes where it outputs a readable stream like `0103 0204 0403` instead of a binary stream, possibly with prefixes for clarity `0x0103 0x0204 0x0403`.

## API

The lib crate will expose a sensible api to parse do-core-asm into do-core bytecode





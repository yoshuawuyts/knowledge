# compilers

## Anatomy of a compiler
- __front-end:__ parse text into AST, generate HIR
- __middle-end:__ MIR, opt/analysis
- __back-end:__ literally just LLVM

## MIR
Mid-level Intermediate Representation - mid-level refers to the `back-end`
stage of the compiler. MIR is the desugaring of complex code into simpler code.
By having simpler code, optimizers will only have to deal with a part of the
language, becoming more efficient at their job. Tl;Dr: desugar complex code
into simpler code which is easier to optimize.
- [rust-rfcs/1211](https://github.com/rust-lang/rfcs/blob/master/text/1211-mir.md)

## See Also
- http://prog21.dadgum.com/30.html
- [the super tiny compiler](https://github.com/thejameskyle/the-super-tiny-compiler)

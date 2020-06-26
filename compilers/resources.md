# compiler resources

This is an overview of useful resources on what goes happens inside
compilers. Usually people learn from [the "Dragon
Book"](https://en.wikipedia.org/wiki/Compilers:_Principles,_Techniques,_and_Tools)
in school, but that doesn't reflect how modern compilers are built and not
everyone has access to that book.

These are some resources that I've found helpful as I've been learning about
compilers.

## Introduction to compilers

The [super tiny
compiler](https://github.com/jamiebuilds/the-super-tiny-compiler) is a great
introduction to compilers and shows really well how to go from "plain text
files" to "working program". It covers the basic steps of lexing, parsing,
and writing.

- https://github.com/jamiebuilds/the-super-tiny-compiler

## Modern Compiler Architecture

Modern compilers aren't run just once to produce an artficant: we expect them
to run continually: compiling and checking the program on every keystroke to
provide features such as autocomplete and inline warnings. And we want all of
this to feel instantaneous as well.

In order to enable this we need to design the compiler from the ground up to
support this. Instead of having the compiler be a set sequence of stages, the
compiler uses a "query" system for compilation. The talk on [modern compiler
construction](https://www.youtube.com/watch?v=wSdV1M7n4gQ) goes into detail
on this.

Once you understand the basics of query-based compilation, reading up on how
Rust's compiler works is a great resource. Rust's compiler book covers
[Rust's query architecture](https://rustc-dev-guide.rust-lang.org/query.html)
in great detail. And the core of the compiler is available as a
[library](https://github.com/salsa-rs/salsa) with an [introduction
video](https://rustc-dev-guide.rust-lang.org/query.html) available on YouTube.

- Talk on modern compiler architecture https://www.youtube.com/watch?v=wSdV1M7n4gQ
- Rustc's query engine (salsa) https://github.com/salsa-rs/salsa
- Rustc's query architecture https://rustc-dev-guide.rust-lang.org/query.html
- Introduction to salsa https://rustc-dev-guide.rust-lang.org/query.html

## Parsing

Parsing is a key step in compilers: it's where you convert a text file into
structured objects, or error if it doesn't work. Matklad, the compiler lead
for the rust-analyzer IDE project, wrote an excellent post on [how to write a
Pratt
parser](https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html).
This is an efficient parser that's straightforward to write, and has lots of
useful properties.

- Building a Pratt parser https://matklad.github.io/2020/04/13/simple-but-powerful-pratt-parsing.html

## Errors

Most programs don't start off correctly -- authoring programs is a dialogue
between the programmer and the compiler. And having the compiler provide
meaningful error messages and suggestions on how to fix things is *crucial*.

The talk I'd like to link to [hasn't been given
yet](https://twitter.com/rustconf/status/1276564883237007360). So instead
check out ["The shape of errors to
come"](https://blog.rust-lang.org/2016/08/10/Shape-of-errors-to-come.html) by
Jonathan Turner.

The library used for authoring errors in the Rust compiler is
[rust-lang/annotate-snippets-rs](https://github.com/rust-lang/annotate-snippets-rs)
as the library to author error messages, and is well worth checking out.

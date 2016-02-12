# crates

## How to create a crate.
export the number seven with an implicit return of the number `7`.
```rust
pub fn seven () -> ur32 {
  7
}
```

### crates
Packages you import from other places and share between projects.

### modules
Local files that together create a thing, for example an application or crate.

`mod` is the keyword to define a local module. For example:
```rust
// ./src/lib.rs
mod english {
  mod greetings {
  }

  mod farewells {
  }
}
```

### use
`use` is used after importing a crate to have a shorthand of a method on that
crate available.
```rust
// expose 'thread'
use std::thread;
```
```rust
// expose 'foo' and expose 'seven'
mod foo;
use foo::seven;
```
```rust
// expose 'foo' and expose 'svn'
mod foo;
use foo::seven as svn;
```
```rust
// expose 'foo' and expose 'svn' and expose 'thn'
mod foo;
use foo::{seven as svn, eight as thn};
```

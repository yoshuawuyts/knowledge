# rust

## Managing versions
Manage multiple versions using multirust.
```sh
$ rustup update <channel>   # get latest versions for {stable,beta,nightly}
```
- [multirust](https://github.com/brson/multirust)
- [rustup](https://www.rustup.rs/)

## Deny flags
```rust
#![deny(
    missing_docs,
    missing_debug_implementations,
    missing_copy_implementations,
    trivial_casts,
    unused_import_braces,
    unused_allocation
)]
```

## Closures
```rust
let square = |x| x * x;     // Fn
let mut square = || x *= x; // FnMut
let square = move || x * x; // FnOnce
```
- __Fn__ can close over external variables in a read-only fashion aka "borrow".
- __FnMut__ can close over external values mutably, aka "mutable borrow".
- __FnOnce__ takes ownership of external variables referenced. Aka "owned".

## See Also
- [rust vs java](https://llogiq.github.io/2016/02/28/java-rust.html)
- [rust's built in traits](https://llogiq.github.io/2015/07/30/traits.html)
- [wrapper types in rust](http://manishearth.github.io/blog/2015/05/27/wrapper-types-in-rust-choosing-your-guarantees/)

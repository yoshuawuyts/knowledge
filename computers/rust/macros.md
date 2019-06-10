# macros
Code generation in Rust.

## Setup
### Dependencies
- `syn`
- `quote`

### Cargo.toml
```toml
[lib]
proc-macro = true
```

## Required Macro Value with Syn
```rust
let rt = syn::parse_macro_input!(attr as syn::Expr);
```

## Optional Macro Value with Syn
```rust
let rt = if attr.is_empty() {
  syn::parse_str("string replacement").unwrap()
} else {
  syn::parse_macro_input!(attr as syn::Expr)
};
```

## Get last item from macro_rules
- Build a weird catching macro statement for up to, say, 24 arguments.
- Move the last argument to the first position
- Call to a new macro of type `($head:expr, $($tail:expr),+)`

```rust
macro_rules! span {
    ($a:expr) => span_inner!($a);
    ($a:expr, $b:expr) => span_inner!($b, $a);
    ($a:expr, $b:expr, $c:expr) => span_inner!($c, $a, $b);
    ($a:expr, $b:expr, $c:expr, $d:expr) => span_inner!($d, $a, $b, $c);
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr) => span_inner!($e, $a, $b, $d);
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => span_inner!($f, $a, $b, $d, $e);
}

macro_rules! span_inner {
    ($head:expr, $($tail:expr),+) => (
        println!($($arg)*);
        $y
        println!("done");
    }
}
```

## References
- https://doc.rust-lang.org/book/first-edition/procedural-macros.html

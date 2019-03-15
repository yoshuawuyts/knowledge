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

## References
- https://doc.rust-lang.org/book/first-edition/procedural-macros.html

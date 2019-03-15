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
let rt = match syn::parse::<syn::Expr>(attr) {
  Ok(data) => data,
  Err(_) => syn::parse_str("replacement_string").unwrap(),
};
```

## References
- https://doc.rust-lang.org/book/first-edition/procedural-macros.html

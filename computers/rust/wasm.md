# WASM

## Cargo.toml
```toml
[lib]
crate-type = ["cdylib"]

[dependencies]
failure = "0.1.2"
wasm-bindgen = "0.2.23"
futures-preview = "0.3.0-alpha.7"

[dependencies.web-sys]
version = "0.3.0"
features = [
  "Document",
  "EventTarget",
]

[dev-dependencies]
wasm-bindgen-test = "0.2.23"
```

## Start function
```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn foo() {}

#[wasm_bindgen(start)]
pub fn foo2(x: u32) {}

#[wasm_bindgen(start)]
pub fn foo3<T>() {}
```

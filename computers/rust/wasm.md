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

## `ArrayBuffer` to `Vec<u8>`
```rust
use js_sys::{ArrayBuffer, Uint8Array};

let buf = ArrayBuffer::new(128);

let slice = Uint8Array::new(&buf);
let dest: Vec<u8> = vec![0; slice.length() as usize];
slice.copy_to(&mut dest);
```

- https://docs.rs/js-sys/0.3.25/js_sys/struct.ArrayBuffer.html
- https://github.com/dakom/awsm/blob/master/src/data/js_sys.rs
- https://rustwasm.github.io/wasm-bindgen/api/js_sys/struct.Uint8Array.html#method.view

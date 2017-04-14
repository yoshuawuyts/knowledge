# linking
Link C to Rust

## Hello world
src/main.rs
```rust
#[link(name="hello", kind="static")]
extern {
  fn hello_hello () -> i32;
}

fn main () {
  unsafe {
    hello_hello();
  };
}
```

build.rs
```rust
extern crate gcc;

fn main() {
  gcc::Config::new()
    .file("src/hello.c")
    .include("src")
    .compile("libhello.a");
}
```

Cargo.toml
```toml
[package]
name = "hello-world"
links = "libhello"
build = "build.rs"
version = "1.0.0"

[build-dependencies]
gcc = "0.3"
```

src/hello.c
```c
#include <stdio.h>

int hello_hello () {
  printf("Hello World\n");
  return 0;
}
```

## See Also
- https://doc.rust-lang.org/book/ffi.html
- https://users.rust-lang.org/t/linking-with-custom-c-library/637
- https://doc.rust-lang.org/reference.html#items-and-attributes
- https://doc.rust-lang.org/nomicon/
- http://doc.crates.io/build-script.html

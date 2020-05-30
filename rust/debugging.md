# debugging

## Debug blocks
```rust
if cfg!(debug_assertions) {
  // debug code goes here
}
```

## Add Debugging for specific fields
```rust
use std::fmt::{self, Debug};

impl<T> fmt::Debug for Blocking<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Blocking")
            .field("handle", &self.0)
            .finish()
    }
}
```

## Forward Debug to Display
```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
```

## Implement Display
```rust
use std::fmt::{self, Display};
impl Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

## Implement Debug
```rust
use std::fmt::{self, Debug};
impl Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
```

## Enable debug info for release builds
```sh
$ RUSTFLAGS=-g cargo build --release
```

```toml
[profile.release]
debug = true
```
- https://stackoverflow.com/questions/38803760/how-to-get-a-release-build-with-debugging-information-when-using-cargo

## See Also
- [rust-xxdb](https://michaelwoerister.github.io/2015/03/27/rust-xxdb.html)

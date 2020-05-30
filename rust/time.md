# time
Rust manages and formats time similar to `strftime(2)` in C - this is neat
because it means we can reason about time very similarly.

## Usage
```rust
extern crate time;

// New time
let time = time::now();

// New UTC time
let utc_time = time::now_utc();
```

## See Also
- https://doc.rust-lang.org/time/time/index.html
- https://doc.rust-lang.org/time/time/fn.strftime.html
- http://man7.org/linux/man-pages/man3/strftime.3.html

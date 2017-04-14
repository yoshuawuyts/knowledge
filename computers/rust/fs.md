# fs
Rust has pretty good filesystem support. It has sync methods of reading files,
but also readers that use cursors to read out files one chunk at the time.

## read a file
### reader
```rust
use std::io::BufReader;
use std::fs::File;
use std::io::Read;

pub fn read (location: &str) {
  let f = File::open(location).unwrap();
  let mut rs = BufReader::new(f);

  let mut file = String::new();
  rs.read_to_string(&mut file).unwrap();

  println!("{}", file);
}
```

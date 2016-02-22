# cli
Reading and writing to the command line is something pretty common for
programs. It forms the glue that binds Unix together.

## echo
- `println!`
- `print!`
- `fmt!`

## stdin
### read each line into a string
```rs
use std::io;

fn main() {
  let mut input = String::new();
  match io::stdin().read_line(&mut input) {
    Ok(n) => {
      println!("{} bytes read", n);
      println!("{}", input);
    }
    Err(err) => println!("error: {}", err),
  }
}
```

### read single line from stdin to string
```rust
fn main () {
  let mut line = String::new();
  let stdin = io::stdin();
  stdin.lock().read_line(&mut line).unwrap();
}
```

## stdout
### print single line to stdout no newline
```rust
fn main () {
  print!("minutes: ");
  io::stdout().flush().unwrap();
}
```

## options parsing
- [getopts](https://github.com/rust-lang-nursery/getopts)

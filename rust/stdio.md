# stdio
Reading and writing to the command line is something pretty common for
programs. It forms the glue that binds Unix together.

## echo
- `println!`
- `print!`
- `fmt!`

## Stdin
```rs
use std::io;

fn main() {
  // create an empty string buffer
  let mut input = String::new();

  // read each line into the string buffer
  match io::stdin().read_line(&mut input) {
    Ok(n) => {
      println!("{} bytes read", n);
      println!("{}", input);
    }
    Err(err) => println!("error: {}", err),
  }
}
```

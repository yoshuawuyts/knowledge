# CLI
Reading and writing to the command line is something pretty common for
programs. It forms the glue that binds Unix together.

## Echo
- `println!`
- `print!`
- `fmt!`

## Stdin
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

## Stdout
### print single line to stdout no newline
```rust
fn main () {
  print!("minutes: ");
  io::stdout().flush().unwrap();
}
```

## Options parsing
```rust
extern crate getopts;

use getopts::Options;
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let program = args[0].clone();

  let mut options = Options::new();
  options.optflag("h", "help", "Output usage information");

  let matches = match options.parse(&args[1..]) {
    Ok(m) => { m }
    Err(f) => { panic!(f.to_string()) }
  };

  // help command
  if matches.opt_present("h") {
    print_usage(&program, options);
    return;
  }
}

// print CLI usage
fn print_usage(program: &str, opts: Options) {
  let brief = format!("Usage: {} <file> [options]", program);
  print!("{}", opts.usage(&brief));
}
```
- [getopts](https://github.com/rust-lang-nursery/getopts)

## Subshell
Spawn a new command; returns the status code:
```rust
use std::process::Command;

fn main() {
  let output = Command::new("ls")
    .arg("-la")
    .output()
    .expect("failed to execute process");

  println!("status: {}", output.status);
  println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
  println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
}
```
- https://doc.rust-lang.org/nightly/std/process/struct.Command.html

# strings
Handling strings is interesting in Rust, and given that there are three or so
different types of strings, strings warrant their own entry:

## Read file to string:
```rust
let f = match File::open(&input[0]) {
  Ok(v) => v,
  Err(_) => return println!("File {} does not exist", &input[0]),
};
let mut rs = BufReader::new(f);
let mut file = String::new();
rs.read_to_string(&mut file).unwrap();
```

## Compile a string into an artifact
```rust
let secret_key = include_str!("secret-key.ascii");
```
- [std::include_str!](https://doc.rust-lang.org/std/macro.include_str!.html)

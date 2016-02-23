# fp

## try!
The `try!` macro is used in a function to either get a result, or instantly
return an error. You cannot use `try!` in `main()` because it cannot return a
value. Instead errors must be handled manually there.

```rust
fn write_to_file_using_try() -> Result<(), io::Error> {
  let mut file = try!(File::create("my_best_friends.txt"));
  try!(file.write_all(b"This is a list of my best friends."));
  println!("I wrote to the file");
  Ok(())
}
```

```rust
extern crate csv;

fn main() {
  let mut rdr = match csv::Reader::from_file("data.csv") {
    Ok(file) => file,
    Err(e) => println!("Put nicer error handling here"),
  };

  for record in rdr.decode() {
    let rec: Vec<String> = match record {
      Ok(rec) => rec,
      Err(e) => println!("Put nicer error handling here"),
    };
    println!("{}", rec[0]);
  }
}
```

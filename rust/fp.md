# fp

## try!
The `try!` macro is used in a function to either get a result, or instantly
return an error. You cannot use `try!` in `main()` because it cannot return a
value. Instead errors must be handled manually there.

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

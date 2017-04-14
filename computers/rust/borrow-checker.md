# borrow-checker
One of the most common questions asked by beginners about Rust is “How do I
satisfy the borrow checker?”. The borrow checker is probably one of the
steepest parts of Rust’s learning curve, and it is understandable that
beginners have some trouble applying the concepts in a real world situation.

## Borrow checker rules
- You can only have one mutable reference to a variable at a time
- You can have as many immutable references to a variable as you want
- You can not mix mutable and immutable references to the same variable

## Tips on improving usage of borrow checker
- Use blocks to scope variables
- Use temporary variables to prevent them from going out of scope. E.g. `let
  name = something(); let name = name.foo();`
- For circular references `Rc` and `RefCell` can be useful, they operate at
  runtime though, so be careful
- Use a third module to solve circular links; like you'd do with normalizing
  SQL

## See Also
- https://m-decoster.github.io//2017/01/16/fighting-borrowchk/

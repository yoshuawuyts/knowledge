# primitives

## strings
When using strings you can either use `str`, which are static and cannot be
grown, or the more dynamic `String` type, which can be appended, sliced and
increased because it's allocated on the heap.
`utf8` encoded
- `&str` - stack allocated fixed-size string slice. Used as fn arguments.
- `String` - heap allocated growable string

### new string
```rust
let hello = String::from("Hello, world!");   // String from literal
```

### string literal to string collection
Gathered from IRC:
```rust
"foo".into()   // is a short way of getting a String
```

### string to uppercase
```rust
&str.to_uppercase();   // String slice to uppercase
```
- [Struct std::string::String](https://doc.rust-lang.org/std/string/struct.String.html)

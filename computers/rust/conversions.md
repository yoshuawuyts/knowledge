# Conversions

## `std::str::FromStr`
Create any type from a `str` with error handling through `str`'s `.parse()`
method.
```rust
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
struct Point(i32)

impl FromStr for Point {
  type Err = ParseIntError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let coord = s.parse::<i32>()?;
    Ok(Point(coord))
  }
}

let p = Point::from_str("1")?;
assert_eq!(p, Point(1))

// Or using `str.parse()`
let p: Point = "1".parse()?;
assert_eq!(p, Point(1))

// `str.parse()` with turbo fish syntax
let p = "1".parse::<Point>()?;
assert_eq!(p, Point(1))
```

## `std::string::ToString`
Never implement this, implement `Display` instead.

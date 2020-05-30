# Numbers

## Print a number in binary notation
Prints with max 18 leading zeros:
```rust
let num = 0b0000000000101100u16;
println!("num: {:#018b}", num);
```
- https://stackoverflow.com/questions/44690439/how-do-i-print-an-integer-in-binary-with-leading-zeros#44690529

## Max Values
```rust
 u8 |           255
u16 |        65.535
u32 | 4.294.967.295

 i8 |           127
i16 |        32.767
i32 | 2.147.483.647
````

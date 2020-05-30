# serde

## Conditional compilation
```rust
#[derive(Debug, Clone, Serialize)]
pub struct Manifest<'s, 'i, 'r> {
  name: &'s str,
  #[serde(skip_serializing_if = "Option::is_none")]
  short_name: Option<&'s str>,
  #[serde(skip_serializing_if = "Option::is_none")]
  start_url: Option<&'s str>,
}
```

### Links
- https://serde.rs/field-attrs.html

## Rename value
```rust
#[derive(Debug, Clone, Serialize)]
pub struct Manifest<'s, 'i, 'r> {
  #[serde(rename = "display")]
  display_mode: Option<DisplayMode>,
}
```

### Links
- https://serde.rs/field-attrs.html

## Deserialize Reqwest to Struct
```rust
#[derive(Debug, Clone, Deserialize)]
struct PrivateKey {
  key: String,
}

fn make_req() -> String {
  let url = "https://foo.com/key"
  let key: PrivateKey = reqwest::get(&url)?.json()?;
  Ok(key.key)
}
```

Implemented as:
```rust
/// Try to deserialize the response body as JSON using `serde`.
#[inline]
pub fn json<T: DeserializeOwned>(&mut self) -> Json<T> {
    let body = mem::replace(&mut self.body, Decoder::empty());

    Json {
        concat: body.concat2(),
        _marker: PhantomData,
    }
}

/// A JSON object.
pub struct Json<T> {
    concat: Concat2<Decoder>,
    _marker: PhantomData<T>,
}
```
- https://github.com/seanmonstar/reqwest/blob/478ef9bf158b74de7302c4a9046fda80e97a4731/src/async_impl/decoder.rs
- https://github.com/seanmonstar/reqwest/blob/478ef9bf158b74de7302c4a9046fda80e97a4731/src/async_impl/response.rs#L113

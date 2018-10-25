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

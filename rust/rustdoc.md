# rustdoc

## Set Images
```rust
#![doc(html_root_url = "https://api.rocket.rs/v0.4")]
#![doc(html_favicon_url = "https://rocket.rs/v0.4/images/favicon.ico")]
#![doc(html_logo_url = "https://rocket.rs/v0.4/images/logo-boxed.png")]
```

## Inline documentation
Prevent something from showing up as a re-export.
```rust
#[doc(inline)]
pub use foo;
```

## References
- https://doc.rust-lang.org/rustdoc/the-doc-attribute.html

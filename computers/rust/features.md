# Rust Features

## Cargo.toml
1. Features can't be named the same as one of their dependencies.
2. The list in the `[features]` tag lists what their dependencies are.
3. Tag each dependency with `optional = true`.

```toml
[features]
default = ["middleware-logger"]
middleware-logger = []

[dependencies]
log = { version = "0.4.7", optional = true }
```

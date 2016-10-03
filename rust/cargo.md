# cargo
Cargo is the rust package manager. It has a bunch of default settings, but
those are silly. Using the `path` field in `lib` and `bin` we can override the
siliest ones.

## Example
```toml
[package]
name = "swagger_to_md"
version = "1.0.0"
description = "Transform swagger to markdown"
keywords = ["swagger","markdown","transform","md","compile","html"]

[lib]
name = "swagger_to_md"
path = "main.rs"

[[bin]]
name = "swagger-to-md"
path = "bin/cli.rs"

[dependencies]
```

## Lib
Libs are libraries. All it needs to do is expose a bunch of functions.

## Bin
`bin` can have multiple multiple exports. Each command must reference a path
and have an export name. Each file must expose a `main` function.
- [configuring a target](http://doc.crates.io/manifest.html#configuring-a-target)

## Pinning versions
To pin versions in `Cargo.toml` the `cargo-edit` command must be exposed to add
`cargo add`. This pulls the latest version of a dep from crates.io and stores
it in `Cargo.toml`.
- [cargo-edit](https://github.com/killercup/cargo-edit)

## Dont't touch the network
```sh
$ cargo run --frozen
```

## See Also
- [cargo manifest format](http://doc.crates.io/manifest.html)

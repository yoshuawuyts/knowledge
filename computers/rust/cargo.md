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

## Install latest cargo version
```sh
$ rustup override set nightly
```

## Use last known good cargo version
```sh
$ rustup override set $(<rustc-version)
```

## Incremental compilation
```sh
$ CARGO_INCREMENTAL=1 cargo <command>
$ rustc -Zincremental=<path> <other arguments>
```

## Check for correctness
```sh
$ cargo install cargo-check  # install cargo-check
$ cargo check                # run cargo-check
```
- https://github.com/rsolomo/cargo-check

## Publishing scripts
```sh
$ cargo login <token>           # login
$ cargo package                 # bundle into crates.io ready package
$ cargo publish                 # publish a package
$ cargo yank --vers <version>   # remove a version
```

## Docs
- https://docs.rs/

## Add GitHub Org Team to Maintainers
```sh
$ cargo owner --add github:<organization>:<team>
```

## Workspace
```toml
[workspace]
members = [
  "futures",
  "futures-core",
  "futures-channel",
  "futures-executor",
  "futures-io",
  "futures-sink",
  "futures-util",
  "futures-test",
]
```

## Patch dependencies
You can override dependencies in Cargo.toml by setting the `[patch]` section:
```toml
[patch.crates-io]
bar = { path = 'my/local/bar' }
```
- https://doc.rust-lang.org/cargo/reference/manifest.html#the-patch-section


## Git Dependencies
```toml
[dependencies]
rand = { git = "https://github.com/rust-lang-nursery/rand", branch = "next" }

[dev-dependencies.basic-cookies]
git = "https://github.com/yoshuawuyts/basic-cookies"
branch = "fix-cookie-token"
```

## Deprecating Crates
- use [`std::compile_error`](https://doc.rust-lang.org/std/macro.compile_error.html)
  to mention the crate is now deprecated.
- publish new major version

## See Also
- [cargo manifest format](http://doc.crates.io/manifest.html)

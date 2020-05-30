# benches

## Common Commands
```sh
$ cargo bench -- --test                   # verify benches once
$ cargo bench -- --save-baseline master   # set a baseline
$ cargo bench -- --baseline master        # compare against baseline
```

## Run Criterion in a pipeline
```sh
#!/usr/bin/env bash

if [ "${TRAVIS_PULL_REQUEST_BRANCH:-$TRAVIS_BRANCH}" != "master" ] && [ "$TRAVIS_RUST_VERSION" == "nightly" ]; then
    REMOTE_URL="$(git config --get remote.origin.url)";
    cd ${TRAVIS_BUILD_DIR}/.. && \
    git clone ${REMOTE_URL} "${TRAVIS_REPO_SLUG}-bench" && \
    cd  "${TRAVIS_REPO_SLUG}-bench" && \
    # Bench master
    git checkout master && \
    cargo bench && \
    # Bench pull request
    git checkout ${TRAVIS_COMMIT} && \
    cargo bench;
fi
```

## Cargo.toml
```toml
[lib]
bench = false

[dev-dependencies]
criterion = "0.2"

[[bench]]
name = "bench"
harness = false
```

## Iterate over range
```rust
  c.bench_function_over_inputs(
    "from_elem",
    |b, i| {
      b.iter(|| flat_tree::depth(*i));
    },
    10..12,
  );
```

## References
- https://github.com/japaric/criterion.rs/blob/aa392720fd3ad68a10fffab0d0f856083187cc01/book/src/faq.md

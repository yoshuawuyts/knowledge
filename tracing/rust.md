# tracing in Rust

## Passing a span between functions

From "distributed tracing in practice, p23":

> [Go] or other languages have a user-managed process context object
> available. In languages such as Java or C#, thread-local storage would offer
> similar functions.

For Rust we would use `task-local storage` that is implemented on top of
`thread-local storage` to propagate context. The overhead of calling into
this for tracing is minimal [citation of Dapper required]. This mechanism is
referred to in the book as a "scope manager".

## tokio-rs/tracing
- https://github.com/davidbarsky/tracing-tree/
- https://gist.github.com/davidbarsky/d98781f3a9090bb12c6136ab6c631c4f

## References

- https://github.com/rust-lang/rust/issues/73522
- https://blog.yoshuawuyts.com/async-log/

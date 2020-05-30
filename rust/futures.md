# futures
M:N mapping of tasks to threads in Rust.

## `Waker` & `LocalWaker`
The "waker" argument to a future is a callback that's used to signal back to the
scheduler that this future is now ready to be polled again.

- `Waker` is `send + sync`
- `LocalWaker` is `!send + !sync`
- `LocalWaker.into_waker()` converts a `LocalWaker` into a thread-safe `Waker`.

## See Also
- https://github.com/cramertj/wg-net/blob/master/async-book/src/TOC.md
- https://tokio-rs.github.io/tokio-core/tokio_core/net/struct.UdpSocket.html

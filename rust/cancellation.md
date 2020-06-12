# Cooperative Cancellation

## Stop Token

1. Create a stop source.
2. From the source, get a stop token.
3. Use the token to wrap a future or stream.

```rust
use stop_token::StopToken;

async fn main() {
    let stop_source = StopSource::new();
    let stop_token = stop_source.stop_token();

    // The `work` stream will end early: as soon as `stop_token` is cancelled. 
    let mut work = stop_token.stop_stream(work);
    while let Some(event) = work.next().await {
        process_event(event).await
    }
}
```

- https://github.com/async-rs/stop-token

## StopStream proposal

1. Create a stop source.
2. Create a stop token.
3. Use the method `recv_cancel` on Stream or Future that takes a token.

### Usage

```rust
use stop_token::StopToken;

async fn main() {
    let src = StopSource::new();
    let token = src.token();

    // The `work` stream will end early: as soon as `token` is StopSource. 
    for ev.await in work.stop_on(token) {
        process_event(event).await
    }
}
```


### Implemenation
```rust
// async_std::sync
pub struct StopSource;
impl StopSource {
    pub fn new() -> Self;
    pub fn token(&self) -> StopToken;
}
pub struct StopToken;
impl Clone for StopToken {};

// async_std::stream
pub struct StopStream<T>;
impl<T> Stream for StopStream<T> { type Item = T; }
impl Stream { fn stop_on(&StopToken) -> StopStream; }

// async_std::future
pub struct StopFuture<T>;
impl<T> Future for StopFuture<T> { type Output = Result<T, StopError>; }
impl Future { fn stop_on(&StopToken) -> StopFuture; }
```

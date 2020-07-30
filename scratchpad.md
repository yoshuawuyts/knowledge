# tide changelog

This release includes 35 PRs merged over the course of the last month. Most
notably we've streamlined how errors are propagated inside middleware,
introduced a new `ResponseBuilder` type, `State` must now be `Clone`, and we
are introducing an extensible API for `Server::listen`.

## ResponseBuilder

Returning responses from endpoints is often different from operating on
response in middleware. In order to make it easier for people to author
responses we're introducing `tide::ResponseBuilder` in this patch!

You can create a new response builder by calling `Response::builder` and
passing it a status code. This enables writing some really concise endpoints:

```rust
app.at("/").get(|_| async {
    let res = Response::builder(203)
        .body(json!({ "hello": "cats!" }))
        .header("X-Nori", "me-ow")
        .header("X-Chashu", "meewwww");
    Ok(res)
})
```

This sets Tide up really nicely for the future too; once we have async
closures, and a resolution for Ok-wrapping (fingers crossed) this will be
even more concise. We're excited for developments in the language!

## Server listen

Tide [now supports](https://github.com/http-rs/tide/pull/610) extensions for
`App::listen`. This patch introduces a new `Listener` trait that is
implemented for `std` types such as `TcpStream`, `SocketAddr` and
`UnixStream`. But can also be implemented by users of Tide to provide custom
transports.

In particular, what this enables us to do is to start trialing TLS support in
external crates. We'll soon have
[`tide-rustls`](https://github.com/jbr/tide-rustls) available as an external
crate that will enable building TLS-terminating Tide servers:

```rust
let mut app = tide::new();
let listener = TlsListener::build()
    .addrs("localhost:4433")
    .cert(cert)
    .key(key);
app.listen(listener).await?;
```

In addition we're shipping `tide::listener::ConcurrentListener`, a convenient
constructor to have a single server respond to incoming requests from
multiple transports. For example, some applications may want to listen on
both IPv4 and IPv6. With `ConcurrentListener` that's possible:

```rust 
use std::net::{Ipv4Addr, Ipv6Addr};
use tide::listener;

let mut app = tide::new();
let mut listener = listener::ConcurrentListener::new();
listener.add((Ipv4Addr::new(127, 0, 0, 1), 8000));
listener.add((Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1), 8000));
app.listen(listener).await?;
```

## State must be Clone

One problem we had for a while was that when manually nesting or
parallelizing applications, `State` would be wrapped in an `Arc` multiple
times. In this patch we're solving that by providing people with more control
around how `State` is shared by requiring `State` to implement `Clone`.

In most existing applications `State` can be made `Clone` by manually
wrapping it in an `Arc::new`. But it's also possible to wrap individual
fields in an `Arc` and deriving `Clone for the whole struct, as we did in one
of our examples:

```rust
// Example state before this patch.
struct State {
    users: RwLock<Vec<User>>,
}

// Example state after this patch.
#[derive(Clone)]
struct State {
    users: Arc<RwLock<Vec<User>>>,
}
```

There is no *right* answer how to structure `State`; but we wanted to enable
people to factor it in the way that makes most sense for their applications.

## Using of async-trait

We've migrated all of our traits to use
[`async-trait`](https://docs.rs/async-trait/0.1.36/async_trait/). This should
make it easier to author `Middleware` implementations. For convenience Tide
re-exports as `tide::utils::async_trait`.

## Changes in middleware error handling

Before this patch, calling `next().await?` in middleware would return a
`Result<Response>`. The idea was that the `Err` variant could freely be
thrown up the middleware stack, and transformed into a `Response` at the top
of the stack. However in practice this didn't turn out great: middleware such
as CORS needed to manipulate the `Err` path to ensure the right headers were
set. And we didn't provide an interface for this.

So instead this patch changes the way we handle errors in middleware. We
still enable `?` to be used inside middleware, but between each middleware we
convert `Result<Response, tide::Error>` into a `Response`, and if an error
occurred, we populate the newly introduced `Response::error` field.

This means that middleware can always assume there is a valid `Response`
coming through, and no longer needs to check both `Ok` and `Err` branch
returned by `next().await`. An example:

```rust
/// Before this patch: need to check both branches.
async fn my_middleware<State>(req: Request<State>, next: Next) -> Result<Response> {
    println!("before");
    match next().await {
        Err(err) => {
            println!("status code {}", err.status());
            Err(err)
        }
        Ok(res) => {
            println!("status code {}", res.status());
            Ok(res)
        }
    }
}

/// With this patch: there's only a single branch to operate on.
async fn my_middleware<State>(req: Request<State>, next: Next) -> Result<Response> {
    println!("before");
    let res = next().await;
    println!("status code {}", res.status());
    Ok(res)
}
```

_Note: neither of these examples will quite compile until we have async
closures, but it serves to illustrate the point._

## Added

- Add a doc example for `Request::body_form` #631
- Add a doc example for `Request::query` #630
- Add an upload example #619
- Add extensible entrypoint for `Server::listen` #610
- Add `From<Body> for Response` #584
- Add `ResponseBuilder` #580
- Add `Response::error` #570
- Add CORS headers to error responses #546

## Changed

- Use `async_trait` to simplify async signatures #639
- Also include port and host in the log string #634
- Don't panic on missing path param #615
- Return a result from `sse::Sender::send` #598 
- Relax the lifetime requirements of `Server::at` #597
- In middleware `Next::run` now returns `Response` instead of `Result<Response>` #570
- Rename `tide::middleware` to `tide::utils` #567
- Require `State` to be `Clone` #644

## Fixed

- Make `ServeDir` return 404 if file does not exists #637
- Remove `#[must_use]` for `Response::set_status()` #612
- Do not await the spawned task in `Server::listen` #606
- Allow route based function middlewares #600
- Fix CORS middleware to retain cookies #599
- Enable SSE senders to break out of loops #598
- Remove extra unwraps from `insert_header` calls #590 #588 #583
- Don't crash the server when there's a listen error #587
- Add CORS headers to error responses #546

## Internal

- Remove `executable` mode from lib.rs #635
- Update to async-sse 4.0.0 #632
- Comment cleanup fixes #622
- Add clippy to ci #618
- Restore .github docs #616
- Use route-recognizer 0.2 #607
- Introduce an extension trait for testing servers #601
- Update the readme with the latest versions #594
- Fix tempfile usage in tests #592
- Fix CI #589
- Remove unnecessary `move`s from route handlers #581

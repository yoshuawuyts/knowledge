# tide changelog

This release introduces first-class support sessions, fixes a 
long-standing bug with our default middleware, clarifies our stability
guarantees, and renamed the API to register middleware through.

# Sessions

We're excited to announce initial support for sessions in Tide. This feature
enables Tide applications to associate multiple separate requests as
belonging to the same origin. Which is a pre-requisite to build common
web-app features such as user accounts, multi-request transactions, and
[channels](https://blog.yoshuawuyts.com/tide-channels/).

Tide sessions are generic over backend stores and signing strategy. It builds
on the newly released [`async-session`
2.0.0](https://docs.rs/async-session/2.0.0) library, which is a set of common
traits that types that make up a session. But beyond that, much of it is
implementation specific.

Tide ships with a `memory` and `cookie` store by default. However we have
also published several convenient session store implementations for common
databases, providing a nice selection to choose from:

- Memory Session (shipped with Tide)
- Cookie Session (shipped with Tide)
- [async-sqlx-session](https://docs.rs/async-sqlx-session) (SQLite only for now; we hope to support more)
- [async-redis-session](https://docs.rs/async-redis-session)
- [async-mongodb-session](https://docs.rs/async-mongodb-session)

Using "Redis" as the backing session store for Tide is as easy as writing 3
lines and including a dependency in your Cargo.toml:

```rust
use async_redis_session::RedisSessionStore;
use tide::sessions::SessionMiddleware;
use tide::{Redirect, Request};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    // Create a Redis-backed session store and use it in the app
    let store = RedisSessionStore::new("redis://127.0.0.1:6379")?;
    let secret = std::env::var("SESSION_SECRET").unwrap();
    app.with(SessionMiddleware::new(store, secret.as_bytes()));

    app.at("/").get(|mut req: Request<()>| async move {
        // Store a counter in the session; increment it by one on each visit
        let session = req.session_mut();
        let visits: usize = session.get("visits").unwrap_or_default();
        session.insert("visits", visits + 1).unwrap();

        // Render a page that shows the number of requests made in the session
        let visits: usize = req.session().get("visits").unwrap();
        Ok(format!("you have visited this website {} times", visits))
    });

    // Close the current session
    app.at("/reset").get(|mut req: Request<()>| async move {
        req.session_mut().destroy();
        Ok(Redirect::new("/"))
    });

    // Start the server
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}
```

It's still early for Tide sessions. But we're incredibly excited for how
convenient it already is, and excited for the possibilities this will enable!

## Renaming Server::middleware to Server::with

This patch renames `Server::middleware` to `Server::with` in order to
streamline much of the middleware APIs.

```rust
// After this patch
let mut app = tide::new();
app.with(MyMiddleware::new());
app.at("/").get(|_| async move { Ok("hello chashu") });
app.listen("localhost:8080").await?;

// Before this patch
let mut app = tide::new();
app.middleware(MyMiddleware::new());
app.at("/").get(|_| async move { Ok("hello chashu") });
app.listen("localhost:8080").await?;
```

A small change, but ever so convenient.

## No more duplicate log messages

Ever since we introduced application nesting we've had issues with default
middleware running twice. This patch fixes that for our logging middleware in
two ways:

1. We've introduced a `logger` Cargo feature to disable the default logging middleware #661
2. We now track calls to the logger inside the state map to ensure
it's only called once per app #662

There may be room to optimize this further in the future, and perhaps extend
the same mechanisms to work for more built-in middleware. But for now this
patch resolves the most common issues people were reporting.

## Clarification on stability

Tide has been deployed to production in many places: independent authors
taking control of their publishing pipelines, software professionals building
internal tooling, and enterprises running it in key parts of their
infrastructure.

In past Tide releases shipped with a warning that actively recommended
against using it in any critical path. However we've chosen to no longer
include that warning starting this release. Much of the work at the protocol
layer and below has completed, and we've received positive reports on how it
performs.

For the foreseable future Tide will remain on the `0.x.x` semver range. While
we're confident in our foundations, we want to keep iterating on the API.
Once we find that work has slowed down we may decide when to release a 1.0.0
release.

## Added

- Added `From<StatusCode> for Response` #650
- Added a feature-flag to disable the default logger middleware #661
- Added `impl Into<Request> for http_types::Request` #670

## Changes

- Rename `Server::middleware` to `Server::with` #666
- Relax Sync bound on Fut required on sse::upgrade #647
- Consistency improvements for examples #651
- Bumped version number in docs #652
- Add enable logging in README example #657
- Remove "experimental" warning #667

## Fixes

- Guarantee the log middleware is only run once per request #662

## Internal

- Enable clippy for tests #655
- Reorder deps in cargo.toml #658

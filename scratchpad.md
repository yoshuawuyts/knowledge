# tide changelog

This release introduces first-class support sessions, fixes a few
long-standing bugs with our default middleware, and

# Sessions

We're excited to announce initial support for sessions in Tide. This feature
enables Tide applications to associate multiple separate requests as
belonging to the same origin. Which is a pre-requisite to build common
web-app features such as user accounts, multi-request transactions, and
[channels](https://blog.yoshuawuyts.com/tide-channels/).

This feature builds on the newly released [`async-session`
2.0.0](https://docs.rs/async-session/2.0.0) library, an

```rust
use async_redis_session::RedisSessionStore;
use tide::sessions::SessionMiddleware;
use tide::{Redirect, Request};

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();

    // Create a Redis-backed session store and use it in the app
    let store = RedisSessionStore::new("redis://127.0.0.1:6379")?;
    let secret = std::env::var("TIDE_SECRET").unwrap();
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


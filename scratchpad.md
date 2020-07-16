# tide changelog

This release includes 35 PRs merged over the course of the last month. Most
notably we've streamlined how errors are propagated inside middleware,
introduced a new `ResponseBuilder` type, and are introducing an extensible
API for `Server::listen` that will enable TLS support in the (near) future.

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
- Update http-types to 2.2.1 #611
- Use route-recognizer 0.2 #607
- Introduce an extension trait for testing servers #601
- Update the readme with the latest versions #594
- Fix tempfile usage in tests #592
- Fix CI #589
- Remove unnecessary `move`s from route handlers #581

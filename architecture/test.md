# test

## What to test
The goal of any test is to verify expectations. On the one hand you propose
what you _expect_ to happen, and on the other hand you compare those
expectations to the actual result. There's little debate about what testing
does.

What is a point of debate, however, is which expectations to test. For any
given program the only thing that ever needs to be tested is _behavior_.

Treat programs as black boxes where things go in and other things come out.
Test every feasible variation of this, and 100% coverage should be the result.
Stubs and mocks are against this model, as they usually bind into the internals
of the program, making refactoring fragile and cumbersome.

- When a test fails, internals should be modified, never the test.
- When new behavior is added, old tests are never touched.
- When refactoring, tests are never touched.
- Tests should not be stateful.

## Testing outbound services
Sometimes a part of your application must be tested that communicates with an
intricate external service (e.g. a third party api such as S3). There are three
approaches that can be taken:

- hit a service endpoint (usually somewhere on the internet)
- stub out the request at the code level, replacing the code with other code.
- catch the request at the network level and respond with a predetermined
  response.

Hitting actual endpoints does not work offline and leads to monolithic
architectures. It has by far the slowest feedback loop, and does not accurately
reflect whether or not expectations are being met (e.g. too many requests,
timeouts, etc.)

Stubbing code out causes tests to be bound to an _implementation_, which is
brittle and won't survive refactors. This de-incentivizes writing tests because
they give no guarantees.

Stubbing requests out causes tests to be to _behavior_, which does survive
refactors. By stubbing requests you set expectations of the outgoing request,
and reply with a certain response. It tests a range of expectations, and
verifies your application complies with those expectations.

# Closures

## for<'a> lifetimes

> for<'a> introduces a new, unbound lifetime – it means "any lifetime". It's
> useful for closures because the lifetimes of their arguments usually don't
> need to be bound to the lifetime of the closure itself.

- https://twitter.com/GolDDranks/status/1173246939820740608
- https://stackoverflow.com/questions/32194367/expected-bound-lifetime-parameter-found-concrete-lifetime

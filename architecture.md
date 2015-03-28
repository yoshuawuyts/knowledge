# architecture
Some ideas on how to build modular applications from day 0. No monoliths, ever.
The naming of these modules is not important (I made most of them up). It's
about how the different layers interact with each other that matters.

```text
api-* ....... server-side endpoint logic
client-* .... client-side components
domain-* .... server-side domain logic
interface-* . client-side endpoint logic
server-* .... server-side components
store-* ..... client-side domain logic
view-* ...... client-side view
x-* ......... client-side view component
```

## Servers
This list goes from high-level to low-level, starting at the level users
interact with the program.
- __api__: Endpoints. These are the glue code that bind all other parts
    together. Should be kept as small as possible. This is the only thing
    clients should ever talk to.
- __domain__: Core models. These represent the data structures that we want to
    expose to the client. `api` talks directly to these. This is the ubiquitous
    language of our application, and should __always__ be updated (even if the 
    database cannot be migrated).
- __server__: Common functionality. These are the functions that are used by
    the `domain` and `api` modules. Think: database interfaces, middleware, event
    emitting logic.

## Clients
This list goes from high-level to low-level, starting at the level users
interact with the program.
- __view__: The views that are rendered on the screen. These are usually
    closely mapped to the urls used. Should be mostly a grouping of `x`
    components. `view`s are usually directly called from a router.
- __x__: View components that are consumed by views. `x` components should do
    one thing, and one thing well. They're also allowed to consume other `x`
    components to create higher level components. __Example__: _`x-timeline-item` is
    consumed by `x-timeline` to create a timeline. Data flows from `x-timeline`
    into `x-timeline-item`. That way `x-timeline-item` can be a stateless
    (idempotent) wrapper around data which makes it easy to reason about._
- __store__: Core models. These represent the data structures that we want to
    use in the client. `x` components talk directly to these. This is the ubiquitous
    language of our client, and should __always__ be updated (even if the 
    api cannot be migrated).
- __interface__: Endpoint consumers. These map directly to the endpoints we're
    consuming. These can be any protocol (webRTC, websockets, SSE, http) and
    multiple of these can be consumed per store. By having this layer in, the
    internal data is split off from how we're using them internally.
- __client__: Common functionality. These functions are used by the `interface` and
    `store` modules. Think: api interfaces, middleware, local persistance
    layers, socket clients, emitting logic.

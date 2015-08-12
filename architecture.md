# architecture
Some ideas on how to build modular applications from day 0. No monoliths, ever.
The naming of these modules is not important (I made most of them up). It's
about how the different layers interact with each other that matters.

## server-client model

__server__
```text
app-api ........ endpoint logic
app-core ....... context functions
app-domains .... domain logic
app-main ....... server entry point
app-services ... business logic
app-stores ..... database communication
service-* ...... context unaware handler
```

__client__
```
client-actions . business logic
client-core .... context functions
client-stores .. database communication
client-views ... context aware views
component-* .... context unaware views
```

## Circuit breakers
Provide stability and prevent cascading failures in distributed systems.

#### States
- closed (all is well)
- open (it's not ok, don't contact me. Sets a timeout, and tries to recover)
- half-open (allow next request to talk to the service, and see if it passes)

Pieces (for example: levee)
- timeout
- max tries before timeout
- reset timeout

All requests flow through the circuit breaker to check if things work. Lives in
service invocation library.

- track the request
- see where it flowed through
- see what services passed
- see which services failed

That way you can see which parts of your client are slowing down your
application. A useful client to do this in a distributed service is with `hystrix`.

## Self tuning systems
Leverage algorithms & data structures to make systems tune themselves.
Interface with a central configuration and derive from there. Or better, split
up engine from interface & have the engines part be non-configurable. Kinda.

- [self-tuning-systems](https://00f.net/2015/06/01/self-tuning-systems/)

## Client side architecture
- [reactive-mvc-and-the-virtual-dom](http://futurice.com/blog/reactive-mvc-and-the-virtual-dom)
- [combining react flux & web components](http://futurice.com/blog/combining-react-flux-and-web-components)
- [simpler UI reasoning with unidrectional dataflow and immutable data](http://omniscientjs.github.io/guides/01-simpler-ui-reasoning-with-unidirectional/)
- [react-transit](https://github.com/RickWong/react-transmit/blob/master/DOCS.md)

## Entity component system
High performance games sometimes make use of ECS.
- composition over inheritance
- complexity is stored away in the `system`
- nodes just pull in a bunch of properties onto themselves
- unity is an example ECS

- entity: general purpose object, usually just holds an id
- component: raw data for one aspect of the world, and how it interacts with
the world. Labels the entity as having a particular aspect. Usually uses Objects
or Arrays.
- system: performs global actions on entities that have a component of the same
aspect as the system.

- [Entity_component_system](https://en.wikipedia.org/wiki/Entity_component_system)

## See Also
- [service disoriented architecture](http://bravenewgeek.com/service-disoriented-architecture/)
- [engineering blogs](https://github.com/kilimchoi/engineering-blogs)
- [simple made easy](http://www.infoq.com/presentations/Simple-Made-Easy)
- [reusability trap](http://250bpm.com/blog:49)
- [finish your projects](http://250bpm.com/blog:50)
- [server-style-guide](https://github.com/jonathanong/server-style-guide)

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

__resources__
- [reactive-mvc-and-the-virtual-dom](http://futurice.com/blog/reactive-mvc-and-the-virtual-dom)
- [combining react flux & web components](http://futurice.com/blog/combining-react-flux-and-web-components)
- [simpler UI reasoning with unidrectional dataflow and immutable data](http://omniscientjs.github.io/guides/01-simpler-ui-reasoning-with-unidirectional/)
- [react-transit](https://github.com/RickWong/react-transmit/blob/master/DOCS.md)

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

## Domain Driven Design (DDD)
Domain Driven Design (DDD) is a style of coding where you separate your business
logic (domains) from implementation details (api / database). As with
everything in engineering it has tradeoffs:
- __speed__: DDD takes longer than monolithic approaches
- __flexibility__: DDD is more flexible than monolithic approaches

### when should you do DDD?
Ask yourself the following question:
> Will this project be maintained in the future?

If the answer is yes, DDD might be a good fit. If not, DDD is not for you.

### domain vs persistance
- __domain model__: real-life problems and solutions, models __behavior__.
- __persistance model__: what and how data is stored, models __storage structure__.

Start with the domain first, as it caters to the problem that's being solved.
The persistance model is an implementation detail to the domain model.

A domain is persistance agnostic. The persistance model, however, needs to be
aware of the domain in order to transform the data.

### slicing the lasagna
The service-oriented equivalent of spaghetti code is lasagna code: a single
request leads you through multiple (too many) abstraction layers.
[ tbi ]

### domain model methods
The domain model should contain all logic to interact with the model; exposing
complete actions on top of them. That way services can be thin abstractions on
top of the models, giving maximum reusability.

### resources
- [domain model is not persistance model](http://blog.sapiensworks.com/post/2012/04/07/Just-Stop-It!-The-Domain-Model-Is-Not-The-Persistence-Model.aspx/)
- [ddd persistance model and domain model](http://stackoverflow.com/questions/14024912/ddd-persistence-model-and-domain-model)
- [anemic domain model](http://www.martinfowler.com/bliki/AnemicDomainModel.html)

## Microservice Monolith
If you write shell scripts your framework is `unix / sh`. If you write Node
applications your framework is JS. Crossing framework boundries is a cause of
complexity because the interfaces do not fully align. Containing logic within a
single framework saves cost in the long run.

But before we can put the knowledge above into practice, we need to take a step
back and look at architecture. There are generally 2 types of architecture:
- __monolithic__: single codebase, all logic lives within here.
- __modular__: multiple code bases, concerns (and logic) are separated. Buzz
  word: microservice.

Monoliths are beautiful in itself: by deploying a single codebase everything
works. There's a lot more simplicity to the project, but can backfire
horrificly if wrongly designed.

Microservices are much more lenient towards bad design: if a service doesn't
work well you can (in theory) replace it by another, better designed service.
Mistakes don't leak.

Most projects are somewhere in between the two. For example a node project with
an external database, or a service that handles multiple concerns. A
microservice monolith is both:
- __single framework__: no shell-outs or parent processes: all of the logic is
  contained within a single VM. Example: Node + LevelDB.
- __modular architecture__: using an internal event system and separated
  concerns the application logic is decoupled. Every service should be able to
  be run standalone.

By building a microservice monolith you gain the best of both worlds: the
ease of deployment and initial speed of a monolith + the loose coupling and
maintainability of a modular architecutre.

__resources__
- [microservice envy](http://martinfowler.com/bliki/MicroservicePremium.html)

## See Also
- [service disoriented architecture](http://bravenewgeek.com/service-disoriented-architecture/)
- [engineering blogs](https://github.com/kilimchoi/engineering-blogs)
- [simple made easy](http://www.infoq.com/presentations/Simple-Made-Easy)
- [reusability trap](http://250bpm.com/blog:49)
- [finish your projects](http://250bpm.com/blog:50)
- [server-style-guide](https://github.com/jonathanong/server-style-guide)

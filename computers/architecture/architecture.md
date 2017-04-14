# architecture
Some ideas on how to build modular applications from day 0. No monoliths, ever.
The naming of these modules is not important (I made most of them up). It's
about how the different layers interact with each other that matters.

## server model
The top level directory should convey as much information about a project as
possible. Roughly an application should always be split up as such:
```txt
delivery mechanism -> boundries -> message bus -> entities
```
### delivery mechanism
A way of getting messages into the application. Generally some HTTP server but
could be anything really. It's a plugin to the application. Should take care of
abstracting out all application details so only pure data flows out to the
boundries.

### boundries
The application specific business logic. Cannot be abstracted into a framework.
Implements the domain model and exposes it to the outside world over the
delivery mechanism. Has no notion of details that are hosted in the delivery
mechanism. Only operates over data.

### message bus
Ties the `boundries` to the `entities` without hard linking them. Generally
just a function that makes sure the right `entity` is called.

### entities
Non application specific business logic. Generic services that do a thing.

The application root should be able to b

- database should be a plugin
- delivery mechanism should be a plugin

__resources__
- [reactive-mvc-and-the-virtual-dom](http://futurice.com/blog/reactive-mvc-and-the-virtual-dom)
- [combining react flux & webcomponents](http://futurice.com/blog/combining-react-flux-and-web-components)
- [simpler UI reasoning with unidirectional data flow and immutable data](http://omniscientjs.github.io/guides/01-simpler-ui-reasoning-with-unidirectional/)
- [react-transit](https://github.com/RickWong/react-transmit/blob/master/DOCS.md)

## Circuit breakers
Provide stability and prevent cascading failures in distributed systems.

#### States
- closed (all is well)
- open (it's not OK, don't contact me. Sets a timeout, and tries to recover)
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
everything in engineering it has trade-offs:
- __speed__: DDD takes longer than monolithic approaches
- __flexibility__: DDD is more flexible than monolithic approaches

### when should you do DDD?
Ask yourself the following question:
> Will this project be maintained in the future?

If the answer is yes, DDD might be a good fit. If not, DDD is not for you.

### domain vs persistence
- __domain model__: real-life problems and solutions, models __behavior__.
- __persistence model__: what and how data is stored, models __storage structure__.

Start with the domain first, as it caters to the problem that's being solved.
The persistence model is an implementation detail to the domain model.

A domain is persistence agnostic. The persistence model, however, needs to be
aware of the domain in order to transform the data.

### slicing the lasagna
The service-oriented equivalent of spaghetti code is lasagna code: a single
request leads you through multiple (too many) abstraction layers.
[ tbi ]

### domain model methods
The domain model should contain all logic to interact with the model; exposing
complete actions on top of them. That way services can be thin abstractions on
top of the models, giving maximum re-usability.

### resources
- [domain model is not persistence model](http://blog.sapiensworks.com/post/2012/04/07/Just-Stop-It!-The-Domain-Model-Is-Not-The-Persistence-Model.aspx/)
- [DDD persistence model and domain model](http://stackoverflow.com/questions/14024912/ddd-persistence-model-and-domain-model)
- [anemic domain model](http://www.martinfowler.com/bliki/AnemicDomainModel.html)

## Microservice Monolith
If you write shell scripts your framework is `unix / sh`. If you write Node
applications your framework is JS. Crossing framework boundaries is a cause of
complexity because the interfaces do not fully align. Containing logic within a
single framework saves cost in the long run.

But before we can put the knowledge above into practice, we need to take a step
back and look at architecture. There are generally 2 types of architecture:
- __monolithic__: single codebase, all logic lives within here.
- __modular__: multiple code bases, concerns (and logic) are separated. Buzz
  word: microservice.

Monoliths are beautiful in itself: by deploying a single codebase everything
works. There's a lot more simplicity to the project, but can backfire
horrifically if wrongly designed.

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
maintainability of a modular architecture.

__resources__
- [microservice envy](http://martinfowler.com/bliki/MicroservicePremium.html)

## Sessions
Sessions provide a way to perform actions without authenticating every request.
It's the network alternative to gpg agents.

## Continuous Deployment
Also known as `Continuous Integration` (CI), sorta mostly. The goal is to be
able to deploy applications fast with very low friction. It's also more secure
because credentials don't need to be loaded onto individual machines preventing
theft (to some extent, there's no perfect security).

The flow would be as follows
- people develop code on feature branches
- feature branches are pushed and trigger a `build`
- if a feature branch `build` passes, the code gets manually merged into
  `development`
- whenever a merge happens on `development` trigger a `build`
- in the scripts if branch is `development`, trigger `a docker build`
- each succesful `docker build` triggers a `deploy`

There's a clear infrastructure requirement here:
- a git host like GitHub that can host code, keep branches and call webhooks
- a testing platform like BuildKite or Travis that can execute arbitrary code
  triggered by a webhook and report pass / fail
- a building platform like BuildKite or Docker Cloud that can create, tag and
  upload Docker builds and deploy them to infrastructure
- a deployment target like Google Cloud or AWS that can be used as a target for
  the deploys

## See Also
- [service disoriented architecture](http://bravenewgeek.com/service-disoriented-architecture/)
- [engineering blogs](https://github.com/kilimchoi/engineering-blogs)
- [simple made easy](http://www.infoq.com/presentations/Simple-Made-Easy)
- [reusability trap](http://250bpm.com/blog:49)
- [finish your projects](http://250bpm.com/blog:50)
- [server-style-guide](https://github.com/jonathanong/server-style-guide)
- [serverless architecutre](http://martinfowler.com/articles/serverless.html)
- [architecture, the lost years](https://www.youtube.com/watch?v=WpkDN78P884)

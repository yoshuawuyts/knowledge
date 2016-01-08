# patterns
Scalability patterns, or how to write software that works regardless of
algorithms, data structures, abstractions and languages.

Examples here are mainly extracted from [the zeromq
guide](http://zguide.zeromq.org/page:all), which you should read. The guide
also excellently describes why we need patterns and sockets, rather than
centralized brokers:

> So small to medium application developers are trapped. Either they avoid
> network programming and make monolithic applications that do not scale. Or
> they jump into network programming and make brittle, complex applications
> that are hard to maintain.

## terminology
- __intermediaries:__ deals with data on either side of a socket. More
  specifically known as: `proxies`, `queues`, `forwarders`, `device` and
  `brokers`.

## core patterns
- [REQRES](#request-reply) - Connect a set of clients to a set of services.
  Remote procedure call and task distribution pattern.
- [PUBSUB](#publish-subscribe) - Connect a set of publishers to a set of
  subscribers. Data distribution pattern.
- [PUSHPULL](#pipeline) - connect nodes in a fan-out/fan-in pattern that can
  have multiple steps and loops. Parallel task distribution and collection
  pattern.

## request-reply
One client talks to one server, synchronously.
```txt
    ┌──────────┐
    │  Client  │
    ├──────────┤
    │   REQ    │
    └──┬────▲──┘
       │    │
 Hello │    │ World
       │    │
    ┌──▼────┴──┐
    │   REP    │
    ├──────────┤
    │  Server  │
    └──────────┘
```

## publish-subscribe
### extended publish-subscribe
`PUBSUB` with forwarding makes a multi-publish multi-subscriber model less
fragile. The forwarder should only forward connections without keepin state to
stay robust.
```txt
 ┌──────────┐   ┌──────────┐   ┌──────────┐
 │   PUB    │   │   PUB    │   │   PUB    │
 └──────────┘   └──────────┘   └──────────┘
       │              │              │
       └──────────────┼──────────────┘
                ┌─────▼────┐
                │   XSUB   │
                ├──────────┤
                │   Code   │
                ├──────────┤
                │   XPUB   │
                └──────────┘
                      │
       ┌──────────────┼──────────────┐
       │              │              │
 ┌─────▼────┐   ┌─────▼────┐   ┌─────▼────┐
 │   SUB    │   │   SUB    │   │   SUB    │
 └──────────┘   └──────────┘   └──────────┘
```

## pipeline
A single task is broken into multiple tasks that are spread out over a number
of workers, and aggregated in a sink to form a final result.
```txt
                ┌──────────┐
                │Ventilator│
                ├──────────┤
                │   Push   │
                └──────────┘
                    Tasks
       ┌──────────────┼──────────────┐
       ▼              ▼              ▼
 ┌──────────┐   ┌──────────┐   ┌──────────┐
 │   PULL   │   │   PULL   │   │   PULL   │
 ├──────────┤   ├──────────┤   ├──────────┤
 │  Worker  │   │  Worker  │   │  Worker  │
 ├──────────┤   ├──────────┤   ├──────────┤
 │   PUSH   │   │   PUSH   │   │   PUSH   │
 └──────────┘   └──────────┘   └──────────┘
       │              │               │
       └──────────────┼───────────────┘
                   Results
                      ▼
                ┌──────────┐
                │   PULL   │
                ├──────────┤
                │   Sink   │
                └──────────┘
```

## See Also
- [zguide by zeromq](http://zguide.zeromq.org/page:all) - downright brilliant

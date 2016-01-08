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

## core patterns
- [REQRES](#request-reply) - Connect a set of clients to a set of services.
  Remote procedure call and task distribution pattern.
- [PUBSUB](#publish-subscribe) - Connect a set of publishers to a set of
  subscribers. Data distribution pattern.
- [PUSHPULL](#pipeline) - connect nodes in a fan-out/fan-in pattern that can
  have multiple steps and loops. Parallel task distribution and collection
  pattern.

## request-reply
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

## pipeline
[ tbi ]

## See Also
- [zguide by zeromq](http://zguide.zeromq.org/page:all) - downright brilliant

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
- __processes as living cells:__ a process should be resilient against outside
errors, but immediately self-destruct on internal errors

## relationships
It's important to remember that in any kind of networking everything can be
expressed as one of 3 kinds of networks:
- one to one
- many to one
- one to many

## interrupt signals
When `SIGTERM` is sent, remember to close connections, stop listening for new
connections and exit gracefully. By default programs straight up die; which is
not something you want to happen.

## core patterns
- [PAIR](#pair) connect two sockets exclusively. For connecting two threads in
  process, not to be confused with "normal" pairs of sockets.
- [REQRES](#request-reply) - Connect a set of clients to a set of services.
  Remote procedure call and task distribution pattern.
- [PUBSUB](#publish-subscribe) - Connect a set of publishers to a set of
  subscribers. Data distribution pattern.
- [PIPELINE](#pipeline) (alias: `PUSHPULL`) - connect nodes in a fan-out/fan-in
  pattern that can have multiple steps and loops. Parallel task distribution
  and collection pattern.

## request-reply
- __simple:__ base request reply pattern
- __extended:__ with a load balancer to evenly distribute work
- __lazy pirate:__ reliable reqres from the client side
- __simple pirate:__ reliable reqres with load balancing
- __paranoid pirate:__ reliable reqres with heartbeating
- __majordomo:__ service-oriented reliable queueing
- __titanic:__ disk-based/connected reliable queueing
- __binary star:__ primary-backup server failover
- __freelance:__ brokerless reliable reqres

### simple
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

### extended request-reply
This works both for networks of machines and threads within a machine. The
router takes requests of some form, synchronously distributes work over the
workers and then returns the responses.

When networked this would be done using `TCP`. When threading this should be
done `in process`.
```txt
 ┌──────────┐   ┌──────────┐   ┌──────────┐
 │  Client  │   │  Client  │   │  Client  │
 ├──────────┤   ├──────────┤   ├──────────┤
 │   REQ    │   │   REQ    │   │   REQ    │
 └──────────┘   └──────────┘   └──────────┘
       │              │              │
       └──────────────┼──────────────┘
                ┌─────▼────┐
                │  ROUTER  │
                ├──────────┤
                │  Broker  │
                ├──────────┤
                │  DEALER  │
                └──────────┘
                      │
       ┌──────────────┼──────────────┐
       │              │              │
 ┌─────▼────┐   ┌─────▼────┐   ┌─────▼────┐
 │   REP    │   │   REP    │   │   REP    │
 ├──────────┤   ├──────────┤   ├──────────┤
 │Service A │   │Service B │   │Service C │
 └──────────┘   └──────────┘   └──────────┘
```

## publish-subscribe
- __suicidal snail:__ handle too slow subscribers
- __black box:__ design high speed subscribers
- __espresso:__ monitor pub-sub network
- __clone:__ shared key-value store
- __reactor:__ simplify complex server
- __binary star:__ add failover to server

### simple
Simple pub sub is single publisher, multi subscriber. It is fragile; instead
use:
### extended publish-subscribe
`PUBSUB` with forwarding makes a multi-publish multi-subscriber model less
fragile. The forwarder should only forward connections without keeping state to
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

### synchronized pub sub
Usually you only want to start publishing data once you know there's
subscribers. Combining `REQREP` with `PUBSUB` this can be done. Once all
subscribers (magic number) are connected, start publishing data.
```txt
 ┌───────────┐
 │ Publisher │
 ├─────┬─────┤
 │ REP │ PUB │
 └▲──┬─┴──┬──┘
  1  │    │
  │  2    3
 ┌┴──▼─┬──▼──┐
 │ REQ │ SUB │
 ├─────┴─────┤
 │Subscriber │
 └───────────┘
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
       │              │              │
 ┌─────▼────┐   ┌─────▼────┐   ┌─────▼────┐
 │   PULL   │   │   PULL   │   │   PULL   │
 ├──────────┤   ├──────────┤   ├──────────┤
 │  Worker  │   │  Worker  │   │  Worker  │
 ├──────────┤   ├──────────┤   ├──────────┤
 │   PUSH   │   │   PUSH   │   │   PUSH   │
 └──────────┘   └──────────┘   └──────────┘
       │              │               │
       └──────────────┼───────────────┘
                   Results
                ┌─────▼────┐
                │   PULL   │
                ├──────────┤
                │   Sink   │
                └──────────┘
```
### parallel pipeline with kill signaling
Signal workers to stop when batch is done processing.
```txt
                ┌──────────┐
                │Ventilator│
                ├──────────┤
                │   PUSH   │
                └──────────┘
                    Tasks
    ┌──────────────┬──┴───────────┐
    │      ─ ─ ─ ─ ┼ ─ ─ ┬ ─ ─ ─ ─│─ ─ ─ ─ ─ ─
    │     │        │              │     │     │
 ┌──▼──┬──▼──┐  ┌──▼──┬──▼──┐  ┌──▼──┬──▼──┐
 │PULL │ SUB │  │PULL │ SUB │  │PULL │ SUB │  │
 ├─────┴─────┤  ├─────┴─────┤  ├─────┴─────┤
 │  Worker   │  │  Worker   │  │  Worker   │  │
 ├───────────┤  ├───────────┤  ├───────────┤
 │   PUSH    │  │   PUSH    │  │   PUSH    │  │
 └───────────┘  └───────────┘  └───────────┘
       │              │              │        │
       └──────────────┼──────────────┘
                   Results                    │
                ┌─────▼────┐
                │   PULL   │                  │
                ├──────────┤
                │   Sink   │─ ─ KILL signal ─ ┘
                └──────────┘
```

## See Also
- [zguide by zeromq](http://zguide.zeromq.org/page:all) - downright brilliant

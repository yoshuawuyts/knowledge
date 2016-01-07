# patterns
Scalability patterns, or how to write software that works regardless of
algorithms, data structures, abstractions and languages.

## request-reply
In process only, lockstep message communication. If doing it inter process / in
network any type of failure will cause either client or server to freeze.
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

## See Also
- [zguide by zeromq](http://zguide.zeromq.org/page:all) - downright brilliant

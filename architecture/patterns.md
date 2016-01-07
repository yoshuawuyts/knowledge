# patterns
Scalability patterns, or how to write software that works regardless of
algorithms, data structures, abstractions and languages.

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

## See Also
- [zguide by zeromq](http://zguide.zeromq.org/page:all) - downright brilliant

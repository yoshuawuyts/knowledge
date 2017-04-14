# events
Aka event sourcing; it's about message queues and getting stuff from A to B
without dropping any.

## Layout
A typical event sourcing architecture looks like this:
```txt
           Writer app                         Reader app
  ┌───────────────────────────┐      ┌───────────────────────────┐
  │Application with interface │      │Application with interface │
  └─────────────┬─────────────┘      └───────────────────────────┘
                ▼                                  ▲
  ┌───────────────────────────┐                    │
  │        Event queue        │                    │
  └─────────────┬─────────────┘                    │
                ▼                                  │
  ┌───────────────────────────┐      ┌───────────────────────────┐
  │        Event store        │      │     Application state     │
  └───────────────────────────┘      └───────────────────────────┘
                │                                  ▲
                │                                  │
                │       ┌───────────────────────────┐
                └──────▶│       Event handler       │
                        └───────────────────────────┘
```

## See Also
- http://www.confluent.io/blog/event-sourcing-cqrs-stream-processing-apache-kafka-whats-connection

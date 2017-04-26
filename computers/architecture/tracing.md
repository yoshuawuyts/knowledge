# tracing
Any one system is usually constructed of multiple layers. Usually there's at
least a client, and a server, but in complex applications there can be many
more layers.

Tracing does not replace all other tools. Using a distributed tracer can help
focus a performance investigation so that other tools can be applied locally.

## Overview
Tracing exists to solve two problems in complex systems:
- __fault discovery:__ figure out if a part of the system broke down
- __performance:__ understand how the system behaves and find points to improve

In order for a tracing system to be successful it must have certain goals in
mind:
- __ubiquitous deployment:__ it must be able to run everywhere
- __continuous monitoring:__ monitoring should always be turned on

Which in turn can be translated to system requirements:
- Should have negligable performance impact on systems
- Programmers shouldn't need to go out of their way to enable tracing
- Data should be available for analysis within a minute after it's been
  generated

Key components to instrument are:
- control flow libraries (e.g. `epoll(7)` wrappers)
- threading libraries
- RPC libraries

## Sampling
Tracing every request in a high throughput system is expensive. That's why
having a 
For high throughput systems y

## OpenTracing
Open tracing is a spec for cross-vendor tracing. It specifies terminology, but
does not specify any protocols.

## See Also
- http://jaeger.readthedocs.io/en/latest/
- https://eng.uber.com/distributed-tracing/
- https://github.com/d3/d3-hexbin
- https://github.com/uber/deck.gl
- http://opentracing.io/documentation/
- https://research.google.com/pubs/pub36356.html
- https://github.com/opentracing/opentracing-javascript
- http://opentracing.io/documentation/pages/spec.html
- https://github.com/opentracing/specification/blob/master/specification.md
- https://github.com/opentracing/specification/blob/master/semantic_conventions.md

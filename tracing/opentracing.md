# OpenTracing

## Terminology

- https://opentracing.io/specification/

## Architectural Overview

Tracing is roughly made up of 3 parts:

1. A tracing provider that combines information from various sources.
2. Trace IDs, which match individual "spans" with a "trace".
2. Individual traces, which measure the time it took between two points.

The idea is that each application sends individual spans to the tracing
provider, where each span is annotated with a "trace ID". The trace provider
then combines all the spans it's been sent into a cohesive story -- and
that's what tracing is.

## Creating Trace IDs

This is usually some kind of UUID -- UUID V4 is common. This is then put into
a [trace context](https://w3c.github.io/trace-context) which is sent between
HTTP requests. Each individual span that is sent to the trace provider is
annotated with the ID from the trace context. The trace provider combines the
various spans for each ID into a full trace.

## API reference

It seems Jaeger and Zipkin use the same HTTP API. So does Honeycomb. It'd be
curious to figure out how to bridge these. We could possibly also convert
this into a chrome devtools hook?

- https://opentracing-javascript.surge.sh/
- https://www.elastic.co/guide/en/apm/agent/nodejs/3.x/opentracing.html
- https://lib.rs/crates/libhoney-rust
- https://lib.rs/crates/tracing-honeycomb

## References

- https://www.elastic.co/guide/en/apm/agent/nodejs/3.x/opentracing.html
- https://www.elastic.co/guide/en/apm/get-started/current/opentracing.html
- https://github.com/open-telemetry/opentelemetry-js
- https://w3c.github.io/trace-context/

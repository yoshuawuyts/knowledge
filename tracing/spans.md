# Spans

## Synthetic Spans

Sometimes spans need to be created from external events. E.g. a new deploy is
rolled out; a new release is announced, a national holiday occurs. For these
cases it can be useful to inject events that originate from outside the
system. They can help contextualize events.

## Spans

_note: the "service name" can be found in Rust by using `env!("CARGO_PKG_NAME")`._

## Span Links

Sometimes spans can trigger events that are loosely related. E.g. several
HTTP requests reuse the same websocket. Or some batch job does batch
processing for spans. This is where links come into play.

```txt
trace.link.span_id        The span ID you wish to link to
trace.link.trace_id       The trace ID you wish to link to
meta.span_type: "link"    Mark this as being a "link"
trace.parent_id           The parent ID of the span
trace.trace_id            The trace ID of the span
```

- https://docs.honeycomb.io/getting-data-in/tracing/send-trace-data/#links

## Span Events

A "span event" is a timestamped log (event) without a duration.

```txt
name                          The name of the span event
meta.span_type: "span_event"  Mark this trace as a "span_event"
trace.parent_id               The span ID this span event will be attached to
trace.trace_id                The ID of the trace this span event belongs to
Timestamp                     The timestamp at which the span event occurred
service_name (optional)       The name of the service in which the span event occurred
```

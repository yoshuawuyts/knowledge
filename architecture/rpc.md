# Remote Procedure Call (RPC)
In real-world systems, systems must often delegate tasks to other systems. This
is RPC - a system calls another system over the network, and something gets
executed in the process.

## Layout
RPC is by design REQUEST-RESPONSE. A request comes in with an optional payload,
a reply is given with an optional payload. It usually operates over TCP with a
specific encoding schema, but that isn't necesarily the case.

Google's `grpc` package operates over HTTP/2 with binary encoding enabled.
Since HTTP/2 is TLS enabled this means that inter-process communication is
encrypted by default.

## Status Codes
As HTTP is used, you can either use the HTTP status codes - or the `grpc`
defined ones - https://godoc.org/google.golang.org/grpc/codes. The `grpc` codes
have been optimized for `rpc`, though they're less well known. There's an
implicit assumption that the `grpc` package will be used for that specific
purpose.

## See Also
- https://godoc.org/google.golang.org/grpc/codes
- https://twitter.com/FrozenFire/status/736115884536795136

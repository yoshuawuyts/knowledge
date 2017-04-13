# rate limiting
You gotta make sure your system doesn't go down when placed under strain. There
are a few techniques to this

## Rate limiters
Place restraints on API data (e.g. pull). Doesn't work for realtime things
though (e.g. push). There are 2 types of rate limiters:

### Request rate limiter
Limit each user to N requests a second. Run the same value of N for both
development and production to prevent odd surprises.

### Concurrent request limiter
Instead of “You can use our API 1000 times a second”, this rate limiter says
“You can only have 20 API requests in progress at the same time”. This is
useful to limit resource-intensive APIs that can be rather slow.

## Load shedders
A load shedder makes its decisions based on the whole state of the system
rather than the user who is making the request. Load shedders help you deal
with emergencies, since they keep the core part of your business working while
the rest is on fire. There are 2 types of load shedders:

### Fleet usage load shedder
Say you're a bank. Divide your traffic up in two types: critical (create
transactions) and non-critical (list transactions). Then reserve a percentage
of the infrastructure for critical traffic (for example 20%), and start
rejecting non-critical traffic with a 503 if that budget is threatened. This
can prevent complete outages.

### Worker utilization load shedder
If a service is backed by worker processes you can start shedding requests
there. This is a last line of defence, and is mostly damage control - it helps
services go down slower and come back up faster. Requests can be categorized
as:

1. Critical methods
2. POSTs
3. GETs
4. Test mode traffic

This can only be achieved by tracking workers at all times. If a box is too
busy it should start shedding workers slowly. It's important to shed workers
gradually, and not in big chunks to prevent huge spikes and weirdness.

## Architecture
### Token buckets
Create a counter per user that increments in a set interval, and decrements
every time the user makes a request. If the counter hits 0, reject the request.

### Good practices
- make sure that if the rate limiter backend goes down, it doesn't take down
  the API with it
- Show clear, actionable errors if a request is rejected for rate limiting
  reasons. e.g. 429 (too many requests) or 503 (service unavailable).
- Build escape valves so limiters can be disabled.
- Set up alerts and metrics to understand how often they trigger.
- Do dry runs on limiters to figure out how much traffic they would block.

### Build order
- Build request rate limiters first
- Introduce the other types of limiters & shedders as you scale

## See Also
- https://stripe.com/blog/rate-limiters
- https://gist.github.com/ptarjan/e38f45f2dfe601419ca3af937fff574d
- https://en.wikipedia.org/wiki/Token_bucket
- https://medium.com/figma-design/an-alternative-approach-to-rate-limiting-f8a06cf7c94c

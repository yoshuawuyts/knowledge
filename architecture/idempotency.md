# idempotency
Idempotency is when repeating the multiple times gives the same result. It can
be visualized as:
```txt
A + A = A
```

## Stages of error
Connections can error out at 3 different points:
- Initial connection fails
- Connection fails midway through work
- Work succeeds, but connection fails before server can reply

## Networking
- `GET`, `PUT` and `DELETE` are the idempotent HTTP verbs. Use them all
  generously.
- `POST` is the verb to mutate a resource, use this for actions that should
  strictly happen once.
- Have clients retry operations, using exponential backoff + jitter to prevent
  stampeding herd.

## Idempotency keys
To make sure unique requests only happen once (e.g. charging someone money)
"idempotency keys" should be used. It's a unique UUID that's created on the
client, which makes it safe to retry. It's up to the server to respect these
keys and make sure they only happen once. Using an ACID compliant database
helps with this b/c of atomic updates.

The best way to do this is by including a header in a request that has the
value. Use this with the `POST` verb.

```sh
$ curl https://api.mydomain.com/charges \
   -X POST \
   -H "Idempotency-Key: AGJ6FJMkGQIpHUTX" \
   -d amount=2000
```

## See Also
- https://stripe.com/blog/idempotency

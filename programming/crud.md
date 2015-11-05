# crud

CRUD stands for Create, Read, Update, Delete.

## Read
Read is different in that it causes no side effects, making it purely
functional. It usually means that it won't trigger any events, as mutation
observers aren't interested in it.

## Update
Update can also be the creation of a new thing + the destruction of an old
thing, and being aware of that. Update is an illusion I guess?

## Delete
Delete is not necessarily needed; like with git you can keep an index of the
current HEAD + backwards pointers on new entries and never having to delete
anything. Welcome to weird town.

## status codes
Carve out a subset of available status codes, preferably place them behind a
layer of abstraction for consistency.
```txt
200 OK - Everything worked as expected.
400 Bad Request - Often missing a required parameter.
401 Unauthorized - No valid API key provided.
402 Request Failed - Parameters were valid but request failed.
404 Not Found - The requested item doesn't exist.
500, 502, 503, 504 Server errors - something went wrong on the server's end.
```
- [stripe/docs/api#errors](https://stripe.com/docs/api#errors)

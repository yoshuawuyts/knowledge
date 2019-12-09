# http

- `PUT` - use for automated idempotent actions that have no custom user data.
    Think: adding an existing user to an application, starring an object, etc.
    If it already exists, well yeah let it be.

- `POST` - use for custom data that requires user interaction. Can vary between
    requests so if it's already been processed warn.

- `PATCH` - only method of updating. Use if for specific identifiers.

- `GET` - derp

- `DELETE` - moot

## PUT vs POST vs PATCH
The difference between `PUT` and `POST` is that the former is guaranteed to be
idempotent. To match the idempotent nature of the request, `PUT` should only be
used for static requests (e.g. clicking a toggle button or some other automated
action that requires no custom human input). `POST` on the other hand relies on
custom human interaction (forms) that might change from time to time. `POST`
should embrace the nature of what it's doing and not be idempotent and fail
instead. `PATCH` is a partial `PUT`, so should only be used for updating fields
(e.g. minimize bits sent over the wire).

## Headers
- __cache-control__: tell the freshness of a response
- __vary__: check if the passed in header is the same, break cache if isn't

- [best practices for using the vary header](https://www.fastly.com/blog/best-practices-for-using-the-vary-header)

## Status Codes
```txt
100 Continue
101 Switching Protocols
103 Early Hints
200 Ok
201 Created
202 Accepted
203 Non Authoritative Information
204 No Content
205 Reset Content
206 Partial Content
226 Im Used
300 Multiple Choice
301 Moved Permanently
302 Found
303 See Other
304 Not Modified
307 Temporary Redirect
308 Permanent Redirect
400 Bad Request
401 Unauthorized
402 Payment Required
403 Forbidden
404 Not Found
405 Method Not Allowed
406 Not Acceptable
407 Proxy Authentication Required
408 Request Timeout
409 Conflict
410 Gone
411 Length Required
412 Precondition Failed
413 Payload Too Large
414 URI Too Long
415 Unsupported Media Type
416 Requested Range Not Satisfiable
417 Expectation Failed
418 I'm a teapot
421 Misdirected Request
425 Too Early
426 Upgrade Required
428 Precondition Required
429 Too Many Requests
431 Request Header Fields Too Large
451 Unavailable For Legal Reasons
500 Internal Server Error
501 Not Implemented
502 Bad Gateway
503 Service Unavailable
504 Gateway Timeout
505 HTTP Version Not Supported
506 Variant Also Negotiates
510 Not Extended
511 Network Authentication Required
```

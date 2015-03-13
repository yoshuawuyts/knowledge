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

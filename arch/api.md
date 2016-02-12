# API
Building APIs is hard. It's tough to strike a balance between flexibility and
rigidity. Finding the right boundaries for individual elements is a continuous
process. In this section we'll be looking at the different elements that
constitute an API, and explore how we can standardize some parts of this.

- [errors](#errors)
- [validation](#validation)
- [headers](#headers)
- [status codes](#status-codes)
- [architecture](#architecture)

## errors
Errors are objects returned by the server. Based on a status code it's
determined if it's an error or not (generally `4xx / 5xx` status code ranges).
To build more tooling around error handling / creating it would be nice if
errors were predictable. I propose the following fields:
- __type__: The sort of error returned. Maps either onto the domain or the
  layers around it (`invalid_request_error`, `api_error`).
- __message__: A human-readable message giving more details about the error
- __error__: The parameters the error relates to. Optional. Maps 1:1 to json
  schema errors.
- __meta__: Optional extra information.

An example validation error. Here the error was caused by the client, so we
return an `invalid_request_error`. We explain why the error was returned and
show which parameters failed. Additionally we include the documentation
location for this particular request for further reference on how to structure
requests.
```js
{
  type: 'invalid_request_error',
  message: 'The request body was invalid',
  docs: 'https://api.mysite/name',
  error: [
    { field: 'data.name', message: 'is the wrong type' },
    { field: 'data.id', message: 'field is required' }
  ]
}
```
GitHub uses a `documentation_url` field in their errors for doc purposes. This
is useful to provide extra information. A `docs` field is generic and useful
for errors.

### types
Type signals what caused the error. Either the server made a mistake
(`api_error`), the client made a mistake (`invalid_request_error`), or the
request didn't meet the criteria of the domain.

By "meeting the criteria of the domain" I mean that some piece of business
logic decided the request was incorrect, and should therefore be completed as
non-successful. An example would be for a credit card company: a request can be
correctly received and process, but rejected because the card expired a few
years ago.

These errors can be considered to be soft because no infrastructure failed.
It's not uncommon for APIs to respond to these requests with a `200` code, and
include an `error` field. These API miss the point of HTTP, where semantics
should be derived from the status code and the body should only be consulted
for details.

### error
`error` is the technical specifics of what went wrong. Alternative names are:
`params`, `errors`, `fields`.

__resources__
- [stripe/api/errors](https://stripe.com/docs/api#errors)
- [uber/api](https://developer.uber.com/v1/api-reference/)

## hypermedia
Proper hypermedia architecture would constitute:
- `index` page with all available links
- `pagination` with pagination either per `?page=` or a unique id included in
  `_url`

Uris should be specified using uri templates. So that the client can expand
them as needed.
- [developer.github/hypermedia](https://developer.github.com/v3/#hypermedia)

## validation
[tbi]

## rate limiting
```sh
$ curl -i 'https://api.github.com/users/whatever?client_id=xxxx&client_secret=yyyy'
HTTP/1.1 200 OK
Date: Mon, 01 Jul 2013 17:27:06 GMT
Status: 200 OK
X-RateLimit-Limit: 5000
X-RateLimit-Remaining: 4966
X-RateLimit-Reset: 1372700873
```
- [developer.github/rate-limiting](https://developer.github.com/v3/#rate-limiting)

## headers
[tbi]

## status codes
[tbi]

## architecture
[tbi]

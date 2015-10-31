# logs and errors
You cannot fix what you don't know is broken. Keeping logs and handling errors
is crucial for any application in production.

## Stack traces

## Logging

## Error handling
Asynchronous error handling in JavaScript is a little different from most other
languages. Because `try ... catch` only operates within the same tick, it will
not work for asynchronous operations in JavaScript.

There are 2 kinds of errors in JS:
- __expected:__ Expected errors are the result of user interaction. They should
  be handled gracefully and never thrash the process. Generally these errors
  will result in `4xx`. These errors are passed around using the `errback`
  pattern.
- __unexpected:__ Aka `panic`s. Unexpected errors are bugs in the program.
  Unexpected errors should terminate the process, delegating control back to a
  process hypervisor. Before the process is killed, if possible any open
  connections should be closed with a `5xx` status code. These errors are
  passed around using `throw` and `process.on('unhandledException')`, `catch`
  should never be used for this.

### Class: Error
In JS `Error` is a global object that can be subclassed. All errors in
JavaScript should inherit from the `Error` class. New instances of `Error` can
be created by doing:
```js
new Error('db connection error')
```
The created error will have a name of `db connection error` and a stack trace.
Custom properties can be added to the error to provide more information.

Though relatively straight forward, the Error

### errback

### throw

### process.on('unhandledException')

### promises
There are many arguments against Promises, but probably the strongest argument
is the implicit `catch` in every Promise. By having `catch` inside every
Promise, `throw` will no longer panic the process. Instead the unexpected error
is swallowed by the Promise, and returned in the `.catch()` clause similar to
expected errors. This complicates error handling a lot, making it very hard to 

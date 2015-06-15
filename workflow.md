# workflow

- primarily static server, serves up all static files
- write up a static router, sans dynamic queries
- serve content on demand during development
- build static files on deployment, so it minimizes reliance on a backend
- allows for hard caching with hash busted URLs

- this means that you can use static servers interchangeably because you're
  only dealing with the filesystem
- dynamic content can then be layered on top of a static component of the
  server through whatever means you prefer.
- data should be made available through an api.
- dynamic frontend content should be generated on the client using data from
  the api, rather than being generated on the server
- modules should have an `npm start` command, which allows separate viewing of
  the element.

## order
- build system
- html & css (in parallel)
- frontend JS
- backend JS

## iceberg method
- make an ordered list of things you need to do
- work the list top to bottom
- only ever allowed to work on n things. If other things take priority, move
them down the list
- keep reprioritizing lists

### multiple icebergs
- plan space into calendar for when to tackle a list
- possibly place estimations next to the list

## reviews
- primary goal is for the codebase to improve
- secondary (but not far off) is for the person submitting code to succeed
  - helps grow the community
  - and draws new people into the project

- [how to write the perfect PR](https://github.com/blog/1943-how-to-write-the-perfect-pull-request)
- [thoughtbot code review guidelines](https://github.com/thoughtbot/guides/tree/master/code-review)
- [io.js onboarding](https://gist.github.com/yoshuawuyts/538077f9f8a658a8fdb6)


## content
Use the `marked.lexer` to unwrap markdown to key-value blobs. Key value blobs
can optionally contain an html and raw version.

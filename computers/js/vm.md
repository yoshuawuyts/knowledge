# vm
`require('vm')`

## run a string of code in the sandbox
The `vm` package takes an argument of `code` and one of `context` - you don't
get access to `stdin` or `stdout`, but you _can_ pass in custom methods: like a
custom method for `console.log`. Through this mechanism you can write back
values out of the sandbox maintaining semantics. It's pretty darn useful.

```js
const assert = require('assert')
const vm = require('vm')

const sandbox = {
  console: {
    log: (msg) => assert.equal(msg, 'hey there')
  }
}

const okCode = `
  console.log('hey there')
`

const ohnoCode = `
  console.log('hey where?')
`

vm.runInNewContext(okCode, sandbox)
// => :D

vm.runInNewContext(ohnoCode, sandbox)
// => :C
```

## See Also
- https://nodejs.org/api/vm.html

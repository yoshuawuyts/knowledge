# stubs

## proxyquire
```js
const proxyquire = require('proxyquire')

function stub () {
  console.log('so fast')
}
stub['@global'] = true
stub['@noCallThru'] = true
const foo = proxyquire('./darp', {
  'velocity-animate': stub
})
foo()
```
```js
const foo = require('velocity-animate')

module.exports = function () {
  foo()
}
```

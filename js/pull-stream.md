# pull-stream

## source
```js
//a stream of 100 random numbers.
var i = 100
var random = function () {
  return function (end, cb) {
    if(end) return cb(end)
    //only read 100 times
    if(i-- < 0) return cb(true)
    cb(null, Math.random())
  }
}
```

## through
```js
var map = function (read, map) {
  //return a readable function!
  return function (end, cb) {
    read(end, function (end, data) {
      cb(end, data != null ? map(data) : null)
    })
  }
}
```

## sink
```js
//read source and log it.
var logger = function () {
  return function (read) {
    read(null, function next(end, data) {
      if(end === true) return
      if(end) throw end

      console.log(data)
      read(null, next)
    })
  }
}
```

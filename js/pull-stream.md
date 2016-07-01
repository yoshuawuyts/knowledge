# pull-stream

```txt
┌────────┐   ┌─────────┐   ┌────────┐
│ Source │──▶│ Through │──▶│  Sink  │
└────────┘   └─────────┘   └────────┘
```

```txt
┌──────┐   ┌──────┐   ┌──────┐   ┌──────┐
│Source│──▶│ Sink │ ┌▶│ Sink │ ┌▶│ Sink │
└──────┘   ├──────┤ │ ├──────┤ │ └──────┘
           │Source│─┘ │Source│─┘
           └──────┘   └──────┘
            Through    Through
```

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
```js
function postTorrent (req, res, params) {
  return function writable (read) {
    return function readable (end, cb) {
      read(end, function (end, data) {
        if (end === true) return cb(true)
        if (end) return cb(end)
        cb(error.client('nooop'))
      })
    }
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

## Core methods
Core [pull-stream][ps] methods. Marble charts adapted from [xstream][xstream].

### Source
#### count
Create a stream that outputs `0 ... max`. by default, `max = Infinity`
```txt
        count(6)
0---1---2---3---4---5---6
```
```js
const source = pull.count(6)
const sink = pull.log()
pull(source, sink)
```

#### empty
#### error
#### infinite
#### keys
#### once
#### values

### Through
#### async-map
#### filter-not
#### filter
#### flatten
Turn a stream of arrays into a stream of their items.
```txt
---------------1----2---3--
-----a--b----c----d--------
         flatten()
-----a--b----c-1--d-2---3--
```
```js
const source = pull.values([
  pull.values([ 'a', 'b', 'c', 'd' ]),
  pull.values([ 1, 2, 3 ])
])
const sink = pull.log()
pull(source, sink)
```

#### map
Transforms each value from the source stream through a project function, to get
a stream that emits the transformed data.
```txt
--1---3--5-----7------
   map((i) => i * 10)
--10--30-50----70-----
```
```js
const source = pull.values([ 1, 3, 5, 7])
const through = pull.map((i) => i * 10)
const sink = pull.log()
pull(source, through, sink)
```

#### non-unique
#### take
#### through
#### unique

### Sink
#### collect
#### concat
#### drain
#### find
#### log
#### on-end
#### reduce

## See Also
- [streams in Node](https://medium.com/@yoshuawuyts/streams-in-node-ab9f13e15d5)
- [pull-stream.github.io][ps]
- [xstream][xstream]

[ps]: https://pull-stream.github.io/
[xstream]: https://github.com/staltz/xstream

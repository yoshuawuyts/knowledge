# pull-stream
Streams are an asynchronous abstraction that allows dealing with data in small
chunks, pushing bottlenecks into the IO layer. This usually leads to less
memory cost and increased performance, which is a very good thing. In streams,
data flows from a source, through a bunch of through streams, into a sink:
```txt
┌────────┐   ┌─────────┐   ┌────────┐
│ Source │──▶│ Through │──▶│  Sink  │
└────────┘   └─────────┘   └────────┘
```

Which using marble charts is:
```txt
        source()
-------------------------
        through()
-------------------------
         sink()
-------------------------
```
The x-axis in a marble chart is the progression of time. In JavaScript each
dash is roughly equivalent to a `process.nextTick()` call on the event loop.

In pull streams there are 3 types of streams. Source, through and sink. In
order to let data flow, a source and sink must be connected. Through streams
are combinations of sources and sinks, making every connection in the pipeline
a source and a sink that talk to each other.

Under the hood a `through` stream is nothing but a `sink` coupled to a
`source`. When the `source` gets called to provide more data, it calls the
`sink` to request more data. Through this mechanism the full flow of data is
corked until a `sink` is attached at the end that starts reading data.
Conceptually it looks like this:
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
var map = function sink (read, map) {
  //return a readable function!
  return function source (end, cb) {
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

```js
function myCoolFunction () {
  return (read) => (end, cb) => read(end, (end, data) => {
    if (end === true) return cb(true)
    if (end) return cb(end)

    cb(null, 'hey!')
  })
}

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
A stream with no contents (it just ends immediately)
```txt
        empty()
------------------------
```
```js
const source = pull.empty()
const sink = pull.log()
pull(source, sink)
```

#### error
A stream that errors immediately.
```txt
        error()
--e---------------------
```
```js
const source = pull.error()
const sink = pull.log()
pull(source, sink)
```

#### infinite
Create an unending stream by repeatedly calling a generator function (by
default, `Math.random()`)
```txt
    infinite(() => i++)
--1--2--3--4--5--6--7--8--
```
```js
var i = 0
const source = pull.infinite(() => i++)
const sink = pull.log()
pull(source, sink)
```
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
-----1--2----3----4--------
         flatten()
-----1--2----3-1--4-2---3--
```
```js
const source = pull.values([
  pull.values([ 1, 2, 3, 4 ]),
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
If test is a function, read data from the source stream and forward it
downstream until test(data) returns false. If opts.last is set to true, the
data for which the test failed will be included in what is forwarded.  If test
is an integer, take n item from the source.
```txt
--1---2--3----4---5--
   take(3)
--1---2--3|
```
```js
const source = pull.count()
const through = pull.take(3)
const sink = pull.log()
pull(source, through, sink)
```

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

## Patterns
### Eventual values
Return a stream from a function synchronously, emit data from it
asynchronously.
```js
const notify = require('pull-notify')
const pull = require('pull-stream')
const xhr = require('xhr')

pull(request('foobar.com'), pull.log())

function request (url) {
  const xhr$ = notify()

  xhr(url, (err, res, body) {
    if (err) return xhr$.abort(err)
    xhr$(body)
    xhr$.end()
  })

  return xhr$.listen()
}
```

## See Also
- [streams in Node](https://medium.com/@yoshuawuyts/streams-in-node-ab9f13e15d5)
- [pull-stream.github.io][ps]
- [pull-stream/pull-stream](https://github.com/pull-stream/pull-stream)
- [xstream][xstream]

[ps]: https://pull-stream.github.io/
[xstream]: https://github.com/staltz/xstream

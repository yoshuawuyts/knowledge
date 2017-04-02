# server sent events
SSE are a simpler alternative to websockets. So simple even you don't need any
packages for it, and can usually just inline it. It can also be polyfilled in
the browser so yayay.

## Node implementation
```js
var http = require('http')

http.createServer(function (req, res) {
  res.writeHead(200, {
    'Content-Type': 'text/event-stream',
    'Cache-Control': 'no-cache',
    'Connection': 'keep-alive'
  })
  res.write('retry: 10000\n')
  res.write('event: connecttime\n')
  res.write('data: ' + (new Date()) + '\n\n')
  res.write('data: ' + (new Date()) + '\n\n')

  interval = setInterval(function () {
    res.write('data: ' + (new Date()) + '\n\n')
  }, 1000)

  req.connection.addListener('close', function () {
    clearInterval(interval)
  }, false)
}).listen(8080)
```

## Browser implementation
This should probably be coupled with an exponential backoff library that also
implements jitter - that way you keep servers happy in case of a potential
outage and when stuff needs to reconnect.

```js
var source = new EventSource('stream')
source.addEventListener('message', function (event) {
  output.textContent = event.data
}, false)

source.addEventListener('connecttime', function (event) {
  console.log('Connection was last established at: ' + event.data)
}, false)

source.addEventListener('open', function (event) {
  document.body.addEventListener('click', function () {
    source.close()
    console.log('connection closed')
  })
}, false)

source.addEventListener('error', function (event) {
  if (event.target.readyState === EventSource.CLOSED) {
    source.close()
    console.log('Connection closed!')
  } else if (event.target.readyState === EventSource.CONNECTING) {
    console.log('Connection closed. Attempting to reconnect!')
  } else {
    console.log('Connection closed. Unknown error!')
  }
}, false)
```

## See Also
- http://cjihrig.com/blog/server-sent-events-in-node-js/
- https://github.com/Yaffle/EventSource

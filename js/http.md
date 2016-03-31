# http
HTTP is a standard protocol on top of the TCP transport protocol. It retries by
default, and is widely supported. Unlike many other programming languages, Node
ships with its own HTTP client as part of the core library. Given that Node was
originally intended as a JS version of Nginx it should not be surprising that
Node is quite good at HTTP.

`require('http')` in Node is often misunderstood, a lot of people never try it
out choosing to interface with HTTP through frameworks such as `express`
instead. Once you get past the somewhat rough documentation it turns out
`require('http')` is very versatile and only has a tiny API surface.

In this section I'll show some of the more common patterns using Node's HTTP
module.

## Server
### Basic
```js
const http = require('http')
const port = 1337

http.createServer(function (req, res) {
  res.setHeader('Content-Type', 'text/plain')
  res.end('hello world')
}).listen(port)
```

### Listen on random unused port & retrieve port
```js
const getPort = require('get-server-port')
const http = require('http')

const server = http.createServer(function (req, res) {
  res.setHeader('Content-Type', 'text/plain')
  res.end('hello world')
})

server.listen()
const port = getPort(server)
```

### Stream a file as a server response
```js
const http = require('http')
const fs = require('fs')

http.createServer(function (req, res) {
  res.setHeader('Content-Type', 'application/json')
  fs.createReadStream('./file.json').pipe(res)
}).listen()
```

### Switch on a url
A statusCode of `200` is the default. When no path is matched a `404` should be
returned.
```js
const http = require('http')
const url = require('url')

http.createServer(function (req, res) {
  res.setHeader('Content-Type', 'text/plain')

  const path = url.parse(req.url)

  // minimal router
  if (path === '/hello') return res.end('hello')
  if (path === '/error') return res.end('oh no!')
  res.statusCode = 404
  res.end('path not found')
}).listen()
```

## Client
### GET request
```js
const http = require('http')

// create a request and pipe the body to stdout
http.get('http://google.com', function (res) {
  res.pipe(process.stdout)
})
```

```js
const concat = require('concat-stream')
const http = require('http')

// create a request and parse the body
http.get('http://google.com', function (res) {
  res.pipe(concat({ string: true }, function (str) {
    process.stdout.write('status' + res.statusCode + '\n')
    process.stdout.write(str + '\n')
  }))
})
```

### POST request
```js
const concat = require('concat-stream')
const http = require('http')

// create a 'POST' request
const opts = { href: 'http://google.com', method: 'POST' }
const req = http.request(opts, res => res.pipe(process.stdout))

// pipe data into the 'POST' request. Once all data is
// passed the request will be sent off, equivalent to req.end().
fs.createReadStream('./foo.txt').pipe(req)
```

### Request url parsing
The `url` module synergizes extremely well with the `http` module. In order to
parse urls and destructure query strings do:
```js
const http = require('http')
const url = require('url')

http.createServer((req, res) => {
  // pass true to url.parse to also destructure the query object
  const url = url.parse(req.url, true)
  console.log(url)
  res.end()
}).listen()
```

## headers
Get a header:
```js
http.createServer(function (req, res) {
  console.log(req.headers['etag'])
  req.end()
}).listen()
```

Get a cookie:
```js
const Cookies = require('cookies')
http.createServer(function (req, res) {
  const cookies = cookies(req, res)
  const cookie = cookies.get('my-cookies')
  req.end(cookie ? 'cookie found' : 'no cookie found')
}).listen()
```

## Request to localhost
```js
const opts = {
  protocol: 'http:',
  hostname: 'localhost',
  port: conf.port,
  path: '/bundle.js',
  method: 'GET'
}

const req = http.request(opts, function (res) {
  t.equal(res.statusCode, 200, 'status was OK')
  t.equal(res.headers['content-type'], 'application/javascript', 'type is JS')
})
req.end()
```

## Download multipart file
Now this is super icky. Apparently `busboy` is fast, but it's also not pretty.
I'd very much like to find a better approach:
```js
// somehow busboy likes to throw errors if incorrect headers are passed :/
try {
  var busboy = new Busboy({ headers: req.headers })
} catch (e) {
  console.error(e)
}

busboy.on('file', function (fieldname, file, filename) {
  file.pipe(process.stdout)
})

req.pipe(busboy)
```

## Create multipart request
```js
const FormData = require('form-data')
const http = require('http')

const opts = {
  protocol: 'http:',
  hostname: 'localhost',
  port: config.API_PORT,
  path: '/torrent/test_image.jpg',
  headers: {
    'Content-type': 'multipart/form-data; boundary=XXX'
  },
  method: 'POST'
}

const req = http.request(opts)
const filePath = path.join(__dirname, '../resources/test_image.jpg')
const rs = fs.createReadStream(filePath)
const form = new FormData()
form.append('upload', rs)
form.pipe(req)
```

# http

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
parse url's and destructure queryStrings do:
```js
const http = require('http')
const url = require('url')

http.createServer((req, res) => {
  // pass true to url.parse to also destructure the query object
}).listen()
```

# electron

## main
### prevent slowdown while in background

```js
app.commandLine.appendSwitch("disable-renderer-backgrounding")
```
- https://pracucci.com/electron-slow-background-performances.html

### load window conditionally
Load from localhost in development, load from disk in production.
```js
var resolve = require('app-root-path').resolve
var path = require('path')

function windowURL (page) {
  if (isDev) {
    return 'http://localhost:8000/' + page
  }

  return path.join('file://', resolve('./renderer/out'), page, 'index.html')
}
```

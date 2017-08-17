# electron

## main
### prevent slowdown while in background

```js
app.commandLine.appendSwitch("disable-renderer-backgrounding")
```
- https://pracucci.com/electron-slow-background-performances.html

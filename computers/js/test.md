# test

## Testing outbound services

You can capture network requests in JS using `nock`:
```js
const nock = require('nock')
nock.recorder.rec()
```
Which will then output all data to stdout.

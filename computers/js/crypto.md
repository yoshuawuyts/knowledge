# crypto

## md5
```js
const crypto = require('crypto')
const fs = require('fs')

var id = crypto.createHash('md5')
  .update(fs.readFileSync('./source-file', 'utf8'))
  .digest('hex')
  .slice(0, 8)
```

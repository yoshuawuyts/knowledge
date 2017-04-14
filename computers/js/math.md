# math

## octals
Strict mode isn't happy with octal literals, instead do:
```js
var _0777 = parseInt('0777', 8)
```

## regex

## Character Classes
```txt
\c Control character
\s White space
\S Not white space
\d Digit
\D Not digit
\w Word
\W Not word
\x Hexade­cimal digit
\O Octal digit
```

### case insensitive search
```js
/string/i
```

### Match urls
```js
/* eslint-disable no-useless-escape */
const protocol = '^(http(s)?(:\/\/))?(www\.)?'
const domain = '[a-zA-Z0-9-_\.]+'
const tld = '(\.[a-zA-Z0-9]{2,})'
const params = '([-a-zA-Z0-9:%_\+.~#?&//=]*)'
const qs = '\?.*$'
/* eslint-enable no-useless-escape */
```

## See Also
- https://www.cheatography.com/davechild/cheat-sheets/regular-expressions/

# cloc
Count lines of code

## JavaScript
The special file name `-` must be passed to read from stdin. Also the name of
the file using `--stdin-name` to determine the syntax.
```sh
$ browserify index.js | cloc --stdin-name=index.js -
```

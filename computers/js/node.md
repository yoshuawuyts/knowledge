# node

## Child Process
### Spawning a child process
`fork(2)` is the baller way to spawn child processes; it's low level and used
in virtually every language. This is how you spanw a node process from another
node process:
```js
var fork = require('child_process').fork
var path = require('path')

var location = path.join(__dirname, 'index.js')
var config = { env: { foo: 'bar' } }
var args = []
var proc = fork(location, args, config)
```

To set some cool values (like shared socket, or shared file descriptors) you
can pass the [`stdio`
option](http://devdocs.io/node~6_lts/child_process#child_process_options_stdio):

```js
var config = {
  stdio: [ 'pipe', 'pipe', 'pipe', 'ipc' ]
}
```
It must always have 1 socket to do `ipc` over. If you want `stdin` and `stdout`
to be ignore you can change the `'pipe'` values to `'ignore'`.

## LLDB debuggin

- https://www.npmjs.com/package/llnode
```sh
$ brew install llnode

# link it
$ ln -sf /usr/local/opt/llnode/llnode.dylib \
    ~/Library/Application\ Support/LLDB/PlugIns/

$ lldb -- node <node commands>
```

```sh
# lldb commands
b     # set breakpoint
b ls  # list breakpoints
exit  # exit
r     # run the program
bt    # backtrace (C++ only)
v8 bt # lldb node magic JS backtrace
```
- https://lldb.llvm.org/lldb-gdb.html

# shell
Shell scripts are the essence of unix, they chain programs together treating
them as a black box that takes input and produces output. Languages don't
matter, as long as it supports stdin, stdout. This is an overview of the shell
language. Not bash, zsh, or anything fancy. Just your regular old
cross-platform `sh`.

## if-else statement
The conditional between brackets is checked for an exit code (e.g. only `0`
passes the test) and then it proceeds down the logic tree.
```sh
if [ 'hello' == 'world' ]; then
  echo hello
else
  echo nah
fi
```

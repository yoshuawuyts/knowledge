# tracing

## Trace function calls
```sh
$ uprobe -l <path-to-binary>  # Log symbols from binary
$ uprobe p:bash:readline      # trace the `readline()` symbol from `$ bash`
```

# strace
CLI tracing tool for linux

## Usage
```sh
$ strace -e trace=open,close,read,write,connect,accept <cmd>  # view open files
```

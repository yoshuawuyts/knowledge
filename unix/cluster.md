# cluster
Having a server is cool, but sometimes you need more than one server. There are
different approaches to this.

## ps + echo
`ps` is a way of outputting all sorts of data for processes.
```sh
$ echo $$                       # get current process pid
$ ps -A -o ppid,pid,stat,comm   # get processes for current pid
```
## jobs
Jobs is a way to manage processes started by the current process.
```sh
$ jobs   # get child processes for current pid + status of process
```

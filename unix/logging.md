# logging

## syslog(3)
`syslog(3)` is the C logging library in unix. It writes to `/log/<filename>` by
default.

## logger(1)
`logger(1)` is the standard log command in unix. It's the shell interface to
`syslog(3)` and provides a way to redirect stdout / stderr to the system
logger.
- [logs are streams not files](http://adam.herokuapp.com/past/2011/4/1/logs_are_streams_not_files/)

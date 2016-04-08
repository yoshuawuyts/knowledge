# logging

## syslog(3)
`syslog(3)` is the C logging library in unix. It writes to `/log/<filename>` by
default.

## logger(1)
`logger(1)` is the standard log command in unix. It's the shell interface to
`syslog(3)` and provides a way to redirect stdout / stderr to the system
logger. It's included in most Unix distributions by default.

`logger(1)` does not handle log rotation, as that is a different concern. All
it will do is log to the default system log. While this can be useful, when
using the `ndjson` format the additional information becomes useless.

__example entry__
```txt
Nov  3 11:57:23 localhost user.notice root: hello world!
```
__links__
- [12factor/logs](http://12factor.net/logs)
- [logs are streams not files](http://adam.herokuapp.com/past/2011/4/1/logs_are_streams_not_files/)

## logrotate(1)
`logrotate` is a tool that, well, rotates logs. "Rotating logs" is swapping
out log files for other log files, for example after a certain time limit
(every day) or when a file limit is hit.

`logrotate(1)` is developed by the Fedora team, and is not included in Busybox
and OS X by default. Here's some stuff it can do:
- rotate logs when a file size is reached
- continue to write information to new file after rotating the old file
- compress the rotated files
- specify compression options for rotated log files
- rotate the old log files with the date in the filename
- execute custom shell scripts immediately after log rotation
- remove older rotated log files

__files used__
```sh
/usr/sbin/logrotate  # the logrotate command itself
/etc/logrotate.conf  # main configuration file
/etc/logrotate.d/*   # per-program configuration
```

__links__
- [logrotate source](https://github.com/logrotate/logrotate)

## Log forwarding
- https://github.com/packetzoom/logzoom
- https://packetzoom.com/blog/logzoom-a-fast-and-lightweight-substitute-for-logstash.html
- https://www.elastic.co/products/beats/filebeat

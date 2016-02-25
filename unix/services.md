# services
Keeping services up and running requires practical knowledge of tooling.

## Lifecycle management
### starting a service
- [monit](https://mmonit.com/monit/)
- [openrc](https://en.wikipedia.org/wiki/OpenRC)

### stopping a service
To stop a service, use the `killproc(8)` command passing it a `PIDFILE`.
```sh
$ killproc -p ${PIDFILE} -d ${STOP_TIMEOUT} -SIGTERM $PROG
```

## Provisioning
[tbi]

## Monitoring
Running systems must be monitored to check for irregularities and track
performance.
- [sysdig](https://github.com/draios/sysdig)
- [strace](https://en.wikipedia.org/wiki/Strace)
- [systemtap](https://en.wikipedia.org/wiki/SystemTap)

- [50 shades of system monitoring](https://sysdig.com/50-shades-of-system-calls/)

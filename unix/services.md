# services
Keeping services up and running requires practical knowledge of tooling.

## Lifecycle management
### starting a service
Using monit:
```
check host app_name with address 127.0.0.1
  start "/sbin/start app_name"
  stop "/sbin/stop app_name"
  if failed port 80 protocol HTTP
    request /ok
    with timeout 5 seconds
    then restart
```
- [monit](https://mmonit.com/monit/)

Using upstart:
```
description "app_name"

start on startup
stop on shutdown

script
    # Node needs HOME to be set
    export HOME="path/to/node/app"

    exec sudo -u nodejs /usr/local/bin/node path/to/node/app/server.js production 2>>/var/log/app_name.error.log >>/var/log/app_name.log
end script
```
- [upstart]()
- [openrc](https://en.wikipedia.org/wiki/OpenRC)

### stopping a service
To stop a service, use the `killproc(8)` command passing it a `PIDFILE`.
```sh
$ killproc -p ${PIDFILE} -d ${STOP_TIMEOUT} -SIGTERM $PROG
```
- [killproc](http://www.linux-tutorial.info/modules.php?name=ManPage&sec=8&manpage=killproc)

## Provisioning
- [nomad](https://www.nomadproject.io/)

## Monitoring
Running systems must be monitored to check for irregularities and track
performance.
- [sysdig](https://github.com/draios/sysdig)
- [strace](https://en.wikipedia.org/wiki/Strace)
- [systemtap](https://en.wikipedia.org/wiki/SystemTap)

## Directory structures
```txt
~/db/        # symlinks to service databases
~/image/     # downloaded images
~/log/       # symlinks to service logs
~/script/    # scripts to interact with services
~/service/   # symlinks to currently running services
/var/log/    # log files
/var/www/    # currently running services
/var/db/     # database data, can be symlinked to separate mount
```

## Adding configuration to a service
This treads into the territory of opinions, and thus caution is adviced. In my
opinion services must be self-contained; which means they should carry their
own config. If config is dynamic, that too should be carried.

Testing a service should ideally be done in a containerized environment, to
fully mimick the OS situation. Else there's a discrepancy between the way the
service is interacted with during development, and during production. The
closer the two are, the better.

To hold configuration, a service should have a `config/` directory in its root
which would be overlaid 1:1 with the root directory (`/`).

## Naming of services
It's convention to prefix services with either `www-`, `api-` or `service-`.
Sometimes people will use `www-` to mark static websites, and `api-` or
`service-` to mark more backendy type of systems, but that kind of separation
can get messy fast. In the end, make a judgement call and monitor the services
you have, and eventually you'll find a naming scheme that creates a suitable
distinction.

- [50 shades of system monitoring](https://sysdig.com/50-shades-of-system-calls/)
- [benefit of using monit instead of upstart](http://stackoverflow.com/questions/4722675/is-there-benefit-to-using-monit-instead-of-a-basic-upstart-setup?rq=1)

# journalctl
Systemd log viewer

## Usage
```sh
# basics
$ timedatectl list-timezones   # list timezones
$ journalctl --utc             # view logs in UTC
$ journalctl -b                # display logs from current boot
$ journalctl --list-boots      # show boots
$ journalctl -k                # display kernel messages
$ journalctl --no-full         # short messages
$ journalctl -a                # long messages
$ journalclt --no-pager        # don't use the system pager
$ journalctl --disk-usage      # view disk usage

# config
$ sudo journalctl --vacuum-size=1G       # toss old logs at size limit
$ sudo journalctl --vacuum-time=1years   # toss old logs at time limit

# ranges
$ journalctl --since "2015-01-10 17:15:00"
$ journalctl --since yesterday
$ journalctl --since 09:00 --until "1 hour ago"

# filtering
$ journalctl -u nginx.service
$ journalctl -u nginx.service --since today
$ journalctl -u nginx.service -u php-fpm.service --since today
$ journalctl _PID=8088

# output formats
$ journalctl -b -u nginx -o json
$ journalctl -b -u nginx -o json-pretty

# active process monitoring
$ journalctl -n        # show last 10 entries
$ journalctl -n 20     # show last 20 entries
$ journalctl -f        # tail


# more reading
$ man systemd.journal-fields
```

## Persisting logs between boots
`/etc/systemd/journald.conf` must be configured:
```sh
[Journal]
Storage=persistent
```

## See Also
- https://www.digitalocean.com/community/tutorials/how-to-use-journalctl-to-view-and-manipulate-systemd-logs

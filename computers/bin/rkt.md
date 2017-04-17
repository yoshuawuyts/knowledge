# rkt
Container tool.

## Usage
```sh
$ ^]^]^]     # Exit container
$ rkt list   # List running containers
```

## Systemd
`rkt(1)` can be integrated with any init process. Systemd is such an init
process. To test out a container before it's started we use `systemd-run`.
```sh
# run a one-off container locally, for testing purposes
$ systemd-run --slice=machine rkt run --insecure-options=image "$PWD/<image>"

# Log out results
journalctl -M <container_id>
```

## See Also
- https://coreos.com/rkt/docs/latest/using-rkt-with-systemd.html

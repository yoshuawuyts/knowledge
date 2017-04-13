# systemd
The system daemon. A rather controversial piece of technology that also happens
to be very useful. Where the kernel is a rather complex place of system calls,
systemd offers an abstraction on top that requires much less manual work. For
better or worse - it's often times more coupled than raw system calls, but
lower level than most high level tools (e.g. kubernetes, docker).

## Service files
Unit files are files that are used to configure systemd. User-defined unit
files are in `/etc/systemd/system`, system defined unit files are in
`/lib/systemd/system`. Unit files end in `.service`

```sh
[Unit]
Description=MyApp
After=docker.service
Requires=docker.service

[Service]
TimeoutStartSec=0
ExecStartPre=-/usr/bin/docker kill busybox1
ExecStartPre=-/usr/bin/docker rm busybox1
ExecStartPre=/usr/bin/docker pull busybox
ExecStart=/usr/bin/docker run --name busybox1 busybox /bin/sh -c "trap 'exit 0' INT TERM; while true; do echo Hello World; sleep 1; done"

[Install]
WantedBy=multi-user.target
````

## Nspawn
`systemd-nspawn` is the systemd container engine. It's minimal and rather
inflexible, but also very simple. It does a whole bunch of namespacing, and
when run through a service file it can even do networking and resource
limiting. It's pretty great hey.

To run processes in the background you must use a unit file.

```sh
# Clone Debian
$ mkdir debian-tree
$ debootstrap --arch=amd64 unstable debian-tree

# Run a container
$ systemd-nspawn -D debian-tree/ /bin/echo "hello, outside world!"
$ systemd-nspawn -D debian-tree/ /bin/bash
$ systemd-nspawn -D debian-tree/ /sbin/init

# Bind a volume
$ systemd-nspawn -D debian-tree --bind /var/hostdir:/var/containerdir /bin/sh

# Manage containers
$ machinectl list
$ machinectl status debian-tree
$ machinectl reboot debian-tree
$ machinectl poweroff debian-tree

# Manage containers remotely
$ machinectl -H foo@example.com
$ machinectl -H foo@example.com:debian-tree

# Auto start containers
$ mv ~/debian-tree /var/lib/machines/debian-tree
$ systemctl start systemd-nspawn@debian-tree.service    # start now
$ systemctl enable systemd-nspawn@debian-tree.service   # autostart on boot
$ systemctl disable systemd-nspawn@debian-tree.service  # exit

# View logs
$ journalctl -M debian-tree

# Profile
$ systemd-analyze -M debian-tree       # list startup time
$ systemd-analyze blame -M debian-tree # break down the boot time per-unit
```

```systemd
# my-container.service
[Unit]
Description=Container myalpine

[Service]
ExecStart=/usr/bin/systemd-nspawn \
  --quiet \
  --keep-unit \
  --boot \
  --link-journal=try-guest \
  --directory=/var/lib/container/mynewdebian \
  --network-macvlan=eth0
KillMode=mixed
Type=notify
RestartForceExitStatus=133
SuccessExitStatus=133

[Install]
WantedBy=multi-user.target
```

## See Also
- https://chimeracoder.github.io/docker-without-docker/#24
- http://0pointer.net/blog/systemd-for-administrators-part-xxi.html
- https://gist.github.com/sfan5/52aa53f5dca06ac3af30455b203d3404

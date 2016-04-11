# fs
Unix filesystem.

## hierarchy
```
/              Root dir
/bin           Binaries
/dev           Devices
/etc           Host-specific system-wide config
/etc/opt       Add-on config
/etc/X11       X window system config
/home          User home dir
/lib           Libraries for binaries
/media         Removable media
/mnt           Temporary fs's
/opt           Optional apps
/proc          Virtual fs for process & kernel
/root          Home dir of root user
/sbin          Essential system binaries
/srv           Site-specific data served by system
/tmp           Temporary files
/usr           Secondary hierarchy for read-only user data
/usr/bin       Non-essential binaries for all users
/usr/include   Standard include files
/usr/local     Tertiary hierarchy for local data
/usr/sbin      Non-essential system binaries (daemons)
/share         Architecture independent shared data
/src           Source code
/var           Variable files
/var/cache     Application cache data
/var/lib       State information. Used by DBs and the like.
/var/lock      Lock files. Which resources are used.
/var/log       Log files
/var/mail      Mailboxes
/var/opt       Variable data from `/opt/`
/var/run       Information about running system since boot
/var/spool     Tasks waiting to be processed
/var/tmp       Temp files preserved between boots
```
- [Filesystem Hierarchy Standard](http://en.wikipedia.org/wiki/Filesystem_Hierarchy_Standard)

## proc

## tmp files
Sometimes you need a temporary file to store data in. There are several
approaches to this, though one of the best supported ways of achieving this is
using the `mktemp(1)` tool.

### mktemp
OS X / GNU `mktemp` are different, which is heaps confusing. Generate a random
file or directory:
```sh
$ mktemp -d dist/profile-XXXXXX    # create tmp dir
$ mktemp dist/profile-XXXXXX       # create tmp file
```

### /tmp
Not every distro adheres to the Linux
[Filesystem Hierarchy Standard](https://en.wikipedia.org/wiki/Filesystem_Hierarchy_Standard),
but no need to sweat about it, we can create our own:
```sh
$ sudo mkdir /tmp
$ sudo chmod 1777 /tmp   # open to everyone + set sticky bit
```

## creating filesystems
__tools__
- `lsblk(1)` - list partitions on a device
- `mkfs(1)` - create a new filesystem from a device
- `mount(1)` - mount a filesystem on a node
- `unmount(1)` - unmount a filesystem on a node
- `mountpoint(1)` - see if a directory is a mountpoint
- `dd(1)` - convert and copy a file(system)
- `losetup(1)` - setup and control loop devices

## loop devices
[ tbi ]

## print file size in gigabytes of file
```sh
$ local_raw_size="$(stat --printf="%s" "$disk_file")"
$ dc -e "$local_raw_size 1024 / 1024 / 1024 / p"
```

## See Also
- [OpenBSD jumpstart](http://www.openbsdjumpstart.org/#/)

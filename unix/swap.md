# swap
Sometimes you go out of RAM and it's cool to offload some of it to disk. With
SSDs being common these days it's actually a pretty great idea. To do this
swapfiles exist: volume allacations that can be used to offload RAM to.

## Creating swap
Creating swap space goes in 3 steps: creating the volume, mounting the volume
and persisting the volume in `fstab`.
```sh
# Instrumentation
$ sudo swapon -s  # see if swap is enabled
$ free -m         # see memory allocation
$ df -h           # see HDD allocation

# Creating swapfile
$ sudo dd if='/dev/zero' of='/swapfile' bs='1G' count='4'  # slow
$ sudo fallocate -l 4G /swapfile                           # fast

# Enabling swapfile
$ sudo chmod 600 swapfile  # set permissions
$ sudo mkswap /swapfile    # setup swap space
$ sudo swapon /swapfile    # use swap space

# Persisting swapfile
$ printf '/swapfile   none    swap    sw    0   0\n' >> /etc/fstab
```

## Configuration
### Swappiness
There's a "swappiness" param that determines how often your system will use
swap. 0 is only when critical, while 100 is as often as possible.

```sh
$ cat /proc/sys/vm/swappiness                       # view swappiness
$ sudo sysctl vm.swappiness=10                      # set temporary swappiness
$ printf 'vm.swappiness=10\n' >> /etc/sysctl.conf   # persist swappiness
```

### Cache pressure
The `vfs_cache_pressure` setting determines how much the system will choose to
cache inode and dentry info. Inode lookups are rather costly. 0 is as often as
possible, 100 is almost never. It defaults to 100.

```sh
$ cat /proc/sys/vm/vfs_cache_pressure                       # view
$ sudo sysctl vm.vfs_cache_pressure=50                      # set temporary
$ printf 'vm.vfs_cache_pressure=50\n' >> /etc/sysctl.conf   # persist
```

## See Also
- https://www.digitalocean.com/community/tutorials/how-to-add-swap-on-ubuntu-14-04

# btrfs
Butter filesystem, better filesystem. Copy-on-write filesystem lik zfs but open
source.

## Installation
```sh
$ sudo apt-get install btrfs-tools  # Deb based
$ sudo yum install btrfs-progs      # RHEL based
```

## Creating btrfs volume
```sh
# Inspect
$ lsblk -f   # Show block devices

# Allocate a file to become the block device
$ dd if=/dev/zero of=/btrfs.img bs=1G count=4  # slow
$ sudo fallocate -l 4G /btrfs.img              # fast

# Enable btrfs block device
$ sudo losetup loop0 /btrfs.img   # Point loopback device to our image
$ sudo mkfs.btrfs /dev/loop0      # Format as btrfs

# Open volume as block device and mount
$ mkdir /var/btrfs                          # Create directory
$ sudo mount '/dev/loop0' '/var/btrfs'      # Mount block device as volume
$ sudo btrfs filesystem show '/var/btrfs'   # Inspect volume
```

## See Also
- https://github.com/stamf/shocker

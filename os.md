# os
Operating system installation, management and more.

## Boot from usb
Create a bootable USB.
```sh
$ hdiutil convert -format UDRW -o out in.iso
$ diskutil list
$ diskutil partitionDisk /dev/disk2 1 "Free Space" "unused" "100%"
$ sudo dd if=out.dmg of=/dev/disk2 bs=1m
$ diskutil eject /dev/disk2
```

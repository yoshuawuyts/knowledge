# images
Servers should be a commodity, not a special snowflake that needs attention.
Images are snapshots that can be loaded onto a server, containing all the
information needed to get to work instantly.

There are 2 types of images:
- __hypervisors:__ are images that host containers. Generally stay alive for
  longer periods of time and should only start be in charge of managing
  containers.
- __containers:__ are images that do one thing well. Generally they will be
  completely self-contained, requiring only a few ports to be open.

Containers themselves can be split up in 2 types:
- __services:__  handle application logic. Whether it's handle HTTP requests,
  host a server or do crunch numbers, `services` take care of it all.
- __sidekicks:__ handle secondary tasks such as log forwarding, sending health
  checks to the mothership and other secondary jobs.

## AMI
`AMI`'s are Amazon's custom image format.

## Qemu
`qemu` is a command to build images for either `xen` or `KVM`.

## KVM
`KVM` is a virtual machine that resides on the kernel level. It's as low level
as you can get for virtual-machines. If you need Linux-based virtualization
then `KVM` is the solution.

## Packer
`packer` is a tool by hashicorp that can stamp out images for multiple compile
targets based on a single configuration. Targets include: `kvm`, `docker` and
`AMI`.

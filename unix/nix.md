# nix
`nix` is a purely functional package manager that enable programs to run in
isolation without relying on implicit global state.

## Installation
```sh
$ curl https://nixos.org/nix/install | sh
```
And then source a shell script so commands work:
```sh
. .nix-profile/etc/profile.d/nix.sh
```

## Files
```sh
/nix/store/       # global package store, names are saved by hash
~/.nix-profile/   # nix settings directory
```

## Separate user account for nix
As root:

```sh
$ adduser nix
$ mkdir -m 0755 /nix && chown nix /nix
```

From now on, all the operations we do on the shell are done from this nix user:
```sh
# su - nix
$ tar -xf nix-1.7-x86_64-linux.tar.bz2
$ cd nix-1.7-x86_64-linux
$ ./install
```

## Profiles
- [profiles manual](http://nixos.org/nix/manual/#sec-profiles)

## Expressions
Describe packages and how to build them.
- [nix expressions manual](http://nixos.org/nix/manual/#chap-writing-nix-expressions)

## Environments
tbi

## See Also
- [nix pills](http://lethalman.blogspot.it/)

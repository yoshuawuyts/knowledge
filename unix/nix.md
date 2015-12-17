# nix
`nix` is a purely functional package manager that enable programs to run in
isolation without relying on implicit global state.

## Sections
- REPL
- language basics
- functions & imports
- deriviations

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
Nix pills `3 - 7` are essential, `8 - 12` are useful.
- [nix pills](http://lethalman.blogspot.it/)
- [nix pkgs contributors guide](https://nixos.org/nixpkgs/manual/)
- [nix-manual/stdenv](https://nixos.org/nixpkgs/manual/#chap-stdenv)
- [nix-pills/stdenv](http://lethalman.blogspot.com.au/2015/08/nix-pill-19-fundamentals-of-stdenv.html)

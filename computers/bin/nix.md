# nix
`nix` is a purely functional package manager that enable programs to run in
isolation without relying on implicit global state.

## Manage packages
```sh
$ nix-env -q                       # list installed packages
$ nix-env -qaP | grep nodejs       # search for a package
$ nix-env -i nodejs-5.7.1          # install a package by name
$ nix-env -iA nixpkgs.nodejs-5_x   # install package by attribute
$ nix-env -uA nixpkgs.nodejs-5_x   # update package by attribute
$ nix-env -u                       # update all packages
$ nix-env -e nodejs-5.7.1          # uninstall package by name
$ nix-channel --update             # update binary channel
$ nix-collect-garbage              # clean up nix store
```
- [getting started with nix](https://www.domenkozar.com/2014/01/02/getting-started-with-nix-package-manager/)
- [nixos cheat sheet](https://nixos.org/wiki/Cheatsheet)

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
/nix/store/             # global package store, names are saved by hash
~/.nix-profile/         # nix settings directory
~/.nixpkgs/config.nix   # user specific nix config
```

## System config for a specific user
This creates a user specific config. Place this file in
`~/.nixpkgs/config.nix`. It then exposes the `all` collection (collections are
like packages), so when you run `nix-env -i all` this collection will be
installed.
```nix
{
  packageOverrides = pkgs_: with pkgs_; {
    all = with pkgs; buildEnv {
      name = "all";
      paths = [
        ash
        busybox
        htop
      ];
    };
  };
}
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

## Inheriting values
Sometimes you want to inherit a value from the underlying OS; to do this the
"inherits" value can be used.
```nix
with import <nixpkgs> {};

stdenv.mkDerivation main {
  inherit bash;
}
```

## See Also
Nix pills `3 - 7` are essential, `8 - 12` are useful.
- [nix pills](http://lethalman.blogspot.it/)
- [nix pkgs contributors guide](https://nixos.org/nixpkgs/manual/)
- [nix-manual/stdenv](https://nixos.org/nixpkgs/manual/#chap-stdenv)
- [nix-pills/stdenv](http://lethalman.blogspot.com.au/2015/08/nix-pill-19-fundamentals-of-stdenv.html)
- [pinning nix versions](https://gist.github.com/yoshuawuyts/042342257126301387d2)

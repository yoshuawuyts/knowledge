# apt

## apt-get
### install a bunch of deps, cleanly
```sh
#!/bin/sh
pkgs=()
which htop >/dev/null || pkgs+=(htop)
which iftop >/dev/null || pkgs+=(iftop)
which mosh >/dev/null || pkgs+=(mosh)
which screen >/dev/null || pkgs+=(screen)
which tmux >/dev/null || pkgs+=(tmux)
which git >/dev/null || pkgs+=(git)
which jq >/dev/null || pkgs+=(jq)

if [ ! -z "${pkgs}" ]; then
  apt-get install -qq -y "${pkgs[@]}"
fi
```

## Usage
```sh
$ apt-cache search <package-name>   # Seach for a package
$ apt-cache show <package-name>     # View more details on a package
```

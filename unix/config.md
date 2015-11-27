# config

## shell
The following files are essential:
- `~/.profile`
- `~/.inputrc`
- `~/.shrc`

## ~/.bash_profile
To make bash play ball, do the following:
```sh
# ~/.bash_profile
source ~/.profile   # Get the paths
source ~/.shrc      # get aliases
```

## ~/.inputrc
Readline configuration; vim bindings are more convenient than emacs
```sh
set editing-mode vi
set keymap vi-command
```
Alternatively if you're SSH'ing into a remote box with no config and bash you
can do:
```sh
$ set -o vi
```

## ~/.shrc
Generic shell config; can be sourced from any shell
```sh
# utf-8 support
LC_ALL="C.UTF-8"

# aliases
alias la="ls -oahpln --color=auto --group-directories-first -o"
alias e="vim"
alias g="git"
alias s="git status"
```

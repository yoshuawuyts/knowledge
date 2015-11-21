# shell
Shell scripts are the essence of unix, they chain programs together treating
them as a black box that takes input and produces output. Languages don't
matter, as long as it supports stdin, stdout. This is an overview of the shell
language. Not bash, zsh, or anything fancy. Just your regular old
cross-platform `sh`.

## if-else statement
The conditional between brackets is checked for an exit code (e.g. only `0`
passes the test) and then it proceeds down the logic tree.
```sh
if [ 'hello' == 'world' ]; then
  echo hello
else
  echo nah
fi
```

## command line switches
`getopt(1)` is the way to handle CLI flag switches in shell. It's built into
most, if not all shell distributions. It uses `getopt(3)` under the hood.

Arguments are passed with or without colons; e.g.
[tbi]
```sh
#! /bin/sh

USAGE="Usage: `basename $0` [-hv] [-o arg] args"

# Parse command line options.
while getopts hvo: OPT; do
    case "$OPT" in
        h) echo $USAGE; exit 0 ;;
        v) echo "`basename $0` version 0.1"; exit 0 ;;
        o) OUTPUT_FILE=$OPTARG ;;
        \?) echo $USAGE >&2; exit 1; ;; # getopts issues an error message
    esac
done

# Remove the switches we parsed above.
shift `expr $OPTIND - 1`

# We want at least one non-option argument.
# Remove this block if you don't need it.
if [ $# -eq 0 ]; then
    echo $USAGE >&2
    exit 1
fi

# Access additional arguments as usual through
# variables $@, $*, $1, $2, etc. or using this loop:
for PARAM; do
    echo $PARAM
done
```
- [command line option parsing in shell](http://blog.mafr.de/2007/08/05/cmdline-options-in-shell-scripts/)
- [using getopt to get long cmd options](https://stackoverflow.com/questions/402377/using-getopts-in-bash-shell-script-to-get-long-and-short-command-line-options/7948533#7948533)
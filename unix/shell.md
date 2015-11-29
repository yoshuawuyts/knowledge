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

## loop
Looping over `ls` output can cause trouble for filenames that contain spaces,
globs or other odd characters. Instead it's safer to use a glob. In shell loops
also run once if there's no match, so an extra line is needed to `break` the
loop if no matches are found.
```sh
# loop over files in directory
for file in *.wav; do
  [ -e $file ] || break  # handle the case of no *.wav files
  echo "$file"
done
```
- [shellcheck/loop](https://github.com/koalaman/shellcheck/wiki/SC2045)

## command line switches
`getopt(1)` is the way to handle CLI flag switches in shell. It's built into
most, if not all shell distributions. It uses `getopt(3)` under the hood.

`getopt` solves the problem of flag parsing; e.g. the following are considered
equivalent after being parsed by `getopt`:
```sh
$ cmd -aoarg file file
$ cmd -a -o arg file file
$ cmd -oarg -a file file
$ cmd -a -oarg -- file file
```

`getopt` takes argument flags as the first argument (more on colons later) and
the list of arguments to apply the flags on as the second argument. It's
generally recommended to pass the special variable `$*` as the second argument.
```sh
$ getopt <flags> <argument-list>
$ getopt fvdo:i:: $*
```

### defining flags to parse
Using `getopt` requires some trickery. Arguments can be anything: from special
characters (such as globs) to filenames with spaces. To make sure everything is
parsed correctly and errors are handled there's a bit of boilerplate we must
use:
```sh
args="$(getopt abo: $*)"
if [ $? != 0 ]; then
  echo 'Usage: ...'
  exit 2
fi
eval set -- "$args"
```

In the snippet above there's quite a lot going on. To check if `getopt` is able
to parse all arguments, the exit code of the variable assignment is checked.
Assigning the output to `eval set` directly would swallow the exit code so we
must assign it to a variable before that.

After running `getopt`, we must redefine our arguments. `set --` does this. In
order to correctly parse whitespaces in arguments we must run it through
`eval`, resulting in `eval set --` to redefine our arguments.

### flag types & arguments
`getopt` uses delimiters to signal if arguments are optional or not, and what
type of arguments they take. Using the example of "option":
- `o` boolean `-o` flag
- `o:` `-o` takes a required argument in the form
- `o::` `-o` takes an optional argument

### long flags
GNU `getopt` has support for `--flag` style flags (long flags), while simple
`getopt` does not. In extended setopt long options are passed with `--long`
while short options are passed with `--option`. Regular `setopt` has no flags
and just parses short options.

This snippet detects which version is available and allows setting of the
appropriate flags:
```sh
getopt -T > /dev/null
if [ $? -eq 4 ]; then
  # GNU enhanced getopt is available
  eval set -- "$(getopt --long help,output:,version --options ho:v -- "$@")"
else
  # Original getopt is available
  eval set -- "$(getopt ho:v "$@")"
fi
```

### parsing arguments
Once the argument flags have been provided the actual arguments must be parsed.
There are several options:
- boolean switch
- switch with an argument e.g. do `var=$2; shift 2 ;;`
- `--` delimiter, shift and signal a break to parse remaining args
- none of the above, which means break

```sh
while true; do
  case "$1" in
    -v | --verbose ) VERBOSE=true; shift ;;
    -d | --debug ) DEBUG=true; shift ;;
    -m | --memory ) MEMORY="$2"; shift 2 ;;
    --debugfile ) DEBUGFILE="$2"; shift 2 ;;
    -- ) shift; break ;;
    * ) break ;;
  esac
done
```
- [command line option parsing in shell](http://blog.mafr.de/2007/08/05/cmdline-options-in-shell-scripts/)
- [using getopt to get long cmd options](https://stackoverflow.com/questions/402377/using-getopts-in-bash-shell-script-to-get-long-and-short-command-line-options/7948533#7948533)
- [cross platform getopt](http://stackoverflow.com/a/4300224/1541707)
- [getopt vs getopts](http://blog.onetechnical.com/2012/07/16/bash-getopt-versus-getopts/)

## parallel
```sh
fn1 () {
  sleep 3
  echo 'cmd1 done'
}

fn2 () {
  sleep 2
  echo 'cmd2 done'
}

(fn1 & fn2) | cat
echo 'all done'
```

## test if command is available
```sh
#!/bin/sh
if [ "$(uname)" = "Darwin" ]; then
  which gmktemp > /dev/null || exit 1
  alias mktemp="gmktemp"   # typical action on OS X for Linux compat
fi
```

## special variables
- [unix special variables](http://www.tutorialspoint.com/unix/unix-special-variables.htm)

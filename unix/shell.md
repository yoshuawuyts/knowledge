# shell
Shell scripts are the essence of unix, they chain programs together treating
them as a black box that takes input and produces output. Languages don't
matter, as long as it supports stdin, stdout. This is an overview of the shell
language. Not bash, zsh, or anything fancy. Just your regular old
cross-platform `sh`.

## variable expansion
```sh
${variable?word}    # complain if undefined
${variable-word}    # use new value if undefined
${variable+word}    # opposite of the above
${variable=word}    # use new value if undefined, and redefine
${variable:?word}   # complain if undefined or null
${variable:-word}   # use new value if undefined or null
${variable:+word}   # opposite of the above
${variable:=word}   # use new value if undefined or null, and redefine
```
With variable expansion the result does get evaluated immediately though, so in
order to prevent that the statement must be preceded by a `: (null)` character:
```sh
: ${foobar:-hello}
echo "$foobar"
```
- [grymoire/shell/curly-brace-expansion](http://www.grymoire.com/Unix/Sh.html#uh-36)

## if-else statement
The conditional between brackets is checked for an exit code (e.g. only `0`
passes the test) and then it proceeds down the logic tree. In shell everything
is a string. To do equality checks there's the common `=` which does a string
(lexical) check, and there's `-eq` which performs a numeric check. `-eq` and
`-ne` are useful to operate based on exit codes.
```sh
if [ 'hello' = 'world' ]; then
  echo hello
else
  echo nah
fi
```

Or if you're used to C-syntax languages, there's the more familiar looking
version:
```sh
# one line
[ "$1" -eq 0 ] && { echo hi && exit 1; }

# multi-line
[ ! "$?" -eq 0 ] && {
  echo hi
  exit 1
}
```
`{}` creates a non-subshell grouping; `()` creates a subshell grouping which
will not exit the program if `exit 1` is provided.

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
```sh
# loop over command output separated by newlines
cat <filename> | while IFS= read -r my_var; do
  echo "$s"
done
```
- [shellcheck/loop](https://github.com/koalaman/shellcheck/wiki/SC2045)
- [shellcheck/multi-line-loop](https://github.com/koalaman/shellcheck/wiki/SC2066)

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

### all together now
```sh
# set CLI flags
getopt -T > /dev/null
if [ "$?" -eq 4 ]; then args="$(getopt --long help --options h -- "$*")"
else args="$(getopt h "$*")"; fi
[ ! $? -eq 0 ] && { usage && exit 2; }
eval set -- "$args"

# parse CLI flags
while true; do
  case "$1" in
    -h|--help) usage && exit 1 ;;
    -- ) shift; break ;;
    * ) break ;;
  esac
done

# assert argv count
[ "$#" != 0 ] && { usage && exit 1; }
```
- [command line option parsing in shell](http://blog.mafr.de/2007/08/05/cmdline-options-in-shell-scripts/)
- [using getopt to get long cmd options](https://stackoverflow.com/questions/402377/using-getopts-in-bash-shell-script-to-get-long-and-short-command-line-options/7948533#7948533)
- [cross platform getopt](http://stackoverflow.com/a/4300224/1541707)
- [getopt vs getopts](http://blog.onetechnical.com/2012/07/16/bash-getopt-versus-getopts/)
- [longform-getopt-gist](https://gist.github.com/yoshuawuyts/dd400238230b371d9caf)

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

## read from stdin
```sh
while read line; do
  echo "$line"
done
```

## recursively read a directory
```sh
find ./templates | while read file; do
  echo "$file"
done
```

## printf
There exist different flavors of `echo`, where the two main versions conflict
with each other. `printf` is the successor to `echo` and is far more powerful.

To provide syntax highlighting in most editors it's preferable to use `""`
over `''` despite character expansion not being necessary as it's handled by
`printf`.
```sh
$ printf "hello world"                 # echo 'hello world'
$ printf "%s %s" "$var1" "$var2"       # echo contents from var1 and var2
$ printf "%b" "\x1b[1;32mhi\x1b[0m""   # echo 'hi' in green
```

## requiring files
Sometimes a program has multiple commands, and it makes sense to split it into
separate files. When a file is symlinked the paths should continue to link to
the correct files. There's _1 easy trick_ to achieve this:
```sh
$ dirname=$(dirname "$(readlink -f "$0")")
$ cat "$dirname"/foo/bar.txt
```

## named pipes / inter-shell communication
Named pipes are cool to create background processes with that can be addressed
by name to do stuff with. It creates a physical file on the system that can be
used from any shell process. The code below passes the output of `pipe` to
`cat`, which then writes to `out.txt`. When passing in a command to `pipe`
(`ls -la` in this case), it pops back out at the other side.
```sh
$ mkfifo pipe
$ cat < pipe > output.ext
$ ls -la > pipe
```

## detect if script is executed by sudo
`sudo` is user 0 on the system, so we can check for that:
```sh
if [ "$(id -u)" -ne 0 ]; then
  printf "shocker(1) should be executed as sudo\n"
  exit 1
fi
```

## readonly
Variables can be made immutable-ish by using `readonly`:
```sh
readonly foo='bar'
```

## math
either `bc` or `dc` work; but `$(())` seems to work on `dash` too
```sh
$ echo $((1 + 1))
```

## check if variable is not set
```sh
if [ -z "$my_var" ]; then
  printf "var not set\n"
fi
```

## find where a command is located
```sh
$ type nginx
```

## Get unix epoch time
Time since unix epoch, unix epoch, epoch time
```sh
$ date +'%s'
```

## Prompt
Prompt for input, create a shell CLI:
```sh
printf 'Would you like to install? (Y/n)\n'
read -r x
if [ "$x" = "y" ];then
  ...
elif [ "$x" = "" ]; then
  ...
fi
```

And from inside a function:
```sh
choose () {
  printf 'What kind of project do you want to create? ' > /dev/tty
  printf '(base|node|rust)\n' > /dev/tty
  printf '❯ ' > /dev/tty
  read -r project
  echo "$project"
}
```

## See Also
- [grymoire/shell](http://www.grymoire.com/Unix/Sh.html)
- [rich's sh tricks](http://www.etalabs.net/sh_tricks.html)

# sh
Shell scripts and tools.

## man pages
```
  1   User Commands
  2   System Calls
  3   C Library Functions
  4   Devices and Special Files
  5   File Formats and Conventions
  6   Games et. Al.
  7   Miscellanea
  8   System Administration tools and Deamons
```

## File testing in sh
```
-b filename     block special file
-c filename     special character file
-d dirname      check for directory existence
-e filename     check for file existence
-f filename     check for regular file existence not a directory
-G filename     check if file exists and is owned by effective group ID.
-g filename     true if file exists and is set-group-id.
-k filename     sticky bit
-L filename     symbolic link
-O filename     true if file exists and is owned by the effective user id.
-r filename     check if file is a readable
-S filename     check if file is socket
-s filename     check if file is nonzero size
-u filename     check if file set-user-id bit is set
-w filename     check if file is writable
-x filename     check if file is executable
-z <string> ... true if the length of the string is non-zero
```

__example__
```sh
#!/bin/bash
file=./file
if [ ! -e "$file" ]; then
  echo "File does not exist"
else
  echo "File exists"
fi
```

## Pipe stdout to multiple commands
```sh
$ cat file.txt | tee >(pbcopy) >(do_stuff) >(do_more_stuff) | grep errors
```

## Find and replace in multiple files
```sh
$ ag -l <pattern> | xargs sed -i '' -E 's/<old>/<new>/g'
```

## Delete a range of lines
```
$ cat file.txt | sed -e '1,2d'
```

## Manipulate columns with awk
```sh
$ cat file.txt | awk '{$3=$1; gsub(/0[12345]_/, "", $3); $2="|"}{print}'
```
- [source](https://gist.github.com/yoshuawuyts/e964b7bda440d893979e)

## Check for value, fill in if it doesn't exist
```sh
$ screen_width=${COLUMNS:-$(tput cols)}
```

## Connect to ssh server
```sh
ssh -i <path/to/file> <name>@<ip>
```
or with a `~/.ssh/configfile`
```
ssh <Host>
```

## list all open files for user
```sh
lsof -u <ownername>
```

## named pipes / inter-shell communication
Named pipes are cool to create background processes with that can be addressed
by name to do stuff with. It creates a physical file on the system that can be
used from any shell process. The code below passes the output of `pipe` to
`cat`, which then writes to `out.txt`. When passing in a command to `pip`
(`ls -la` in this case), it pops back out at the other side.
```sh
$ mkfifo pipe
$ cat < pipe > output.ext
$ ls -la > pipe
```

## follow logs as they grow
```sh
$ tail -r <file>
```

## execute a command in npm module dir
```txt
npm ex <module name> <command> ... execute a command in the module dir
```

## dig
DNS lookup utility
```sh
# example
$ dig @127.0.0.1 -p 5000 something.foo +short
1.1.1.1
```

## vifm
Vim like file manager. Useful to do bulk directory operations. Offers different
views on files.

## smxi/sgfxi
System / gfx configuration tool.

- [website](http://smxi.org/)

## refind
boot manager

- [website](http://www.rodsbooks.com/refind/)

## lspci
list all pci devices

- [man page](http://man.cx/lspci)

## lsblk
List all available devices. Useful to determine how to partition.

## chroot
Can be used to repair machines that have lost root access / are unbootable for
other reasons. Live cd's ftw! Also used to reset the `pid` of a tree of
processes, a commonly known technique used with `docker`.

- [arch wiki](https://wiki.archlinux.org/index.php/Change_root)

## manage audio players
```sh
$ playerctl
```

## pipe stderr to stdout
```sh
# bash
$ <command> 2>&1 /dev/null

# POSIX sh
$ <command> >/dev/null 2>&1
```

## print multiline string
```sh
cat << EOF
  oh my, such nice text
EOF
```

## detect if script is sourced
```sh
if [ "$_" = "$0" ]
  then echo 'yup, script is directly called'
  else echo 'nope, script is not directly called'
fi
```

## Switch statement
```sh
case $1 in
  "")         usage; exit 1 ;;
  -h|--help)  usage; exit ;;
  -l|--link)  link "$@" ;;
  *)          readonly name=$1 ;;
esac
```

## Format text to be &lt;80 chars
```sh
$ fmt -80
```

## Create random file name
```sh
$ echo $RANDOM
```

## See Also
- [cleaning an arch installation](http://blog.andreascarpino.it/cleaning-an-arch-linux-installation/)
- [the art of the cli](https://github.com/jlevy/the-art-of-command-line)
- [awesome shell](https://github.com/alebcay/awesome-shell)
- [unofficial bash strict mode](http://redsymbol.net/articles/unofficial-bash-strict-mode/)
- [uselessness of cat](http://www.smallo.ruhr.de/award.html)
- [stronger shell](http://m.odul.us/blog/2015/8/12/stronger-shell)
- [the bash guide](http://guide.bash.academy/)
- [the art of the command line](https://github.com/jlevy/the-art-of-command-line)

# awk
Every program has a structure of:
```sh
pattern { action }
```

## Print column
Print first column
```sh
$ awk '{print $1}'
```

## Remove first column
```sh
$ awk '{ $1=""; print }'   # remove first column
$ awk '{ $3=""; print }'   # remove third column
```

## Set multiple columns to same value
```sh
$ awk '{ $1=$2=$3=""; print }'
```

## Execute system command
Sometimes you want to run a shell command on a value. Arguments can be passed
by `awk` by setting the variable after it. The `system` call returns the status
code from the command run:
```sh
$ awk '{ system("printf " $7); print }'
```

If, however, you want to capture the system output do:
```sh
awk '{
  cmd="printf %02d " $7
  while (cmd | getline line) {
    $7=line
  }
  close(cmd)
  print
}
```
- [stackoverflow: how can I pass a variable to an awk script?](http://stackoverflow.com/questions/20646819/how-can-i-pass-variables-from-awk-to-a-shell-command)

## See Also
- [grymoire](http://www.grymoire.com/Unix/Awk.html)

# find

## Exclude directory
```sh
# ignore single directory
$ find . -path './.git' -prune -o -print

# ignore multiple directories
$ find . -type d \( -path dir1 -o -path dir2 -o -path dir3 \) -prune -o -print
```

## recursively read a directory
```sh
find ./templates | while read file; do
  echo "$file"
done
```

## find specific files
```sh
find ./*/start | while read start; do
  echo "$file"
done
```

## Find directory on machine
```sh
$ find / -name '<dirname>' -type d
```

## See Also
- [grimoire/find](http://www.grymoire.com/Unix/Find.html)

# find

## Exclude directory
```sh
# ignore single directory
$ find . -path './.git' -prune -o -print

# ignore multiple directories
$ find . -type d \( -path dir1 -o -path dir2 -o -path dir3 \) -prune -o -print
```

## See Also
- [grimoire/find](http://www.grymoire.com/Unix/Find.html)

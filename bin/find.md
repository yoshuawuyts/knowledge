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
find ./templates | while read file;
  echo "$file"
done
```

## find specific files
```sh
find */init | while read file;
  echo "$file"
done
```

## See Also
- [grimoire/find](http://www.grymoire.com/Unix/Find.html)

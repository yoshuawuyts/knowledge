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

- [grymoire](http://www.grymoire.com/Unix/Awk.html)

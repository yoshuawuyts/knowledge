# sed
`sed` is the superman of UNIX stream editing.
- Input Stream
- Pattern Space
- Hold Buffer
- Output Stream

> Think about the spaces this way - sed reads the input stream and produces the
> output stream. Internally it has the pattern space and the hold buffer. Sed
> reads data from the input stream until it finds the newline character \n.
> Then it places the data read so far, without the newline, into the pattern
> space. Most of the sed commands operate on the data in the pattern space.
> The hold buffer is there for your convenience. Think about it as temporary
> buffer. You can copy or exchange data between the pattern space and the hold
> buffer. Once sed has executed all the commands, it outputs the pattern space
> and adds a \n at the end.
- [source](http://www.catonmat.net/blog/worlds-best-introduction-to-sed/)

## Capture groups
```sh
$ sed -e 's/version=\(.+\)/\1/'
```

## Match zero or more instances
```sh
$ sed 's/.*//g'
```

## Insert on first line of file
```sh
$ sed "1 i my magical line"
```

## Delete first line of output
```sh
$ printf '1\n2\n3\n' | sed -e '1d'
# => 2
# => 3
```

## Get file extension
```sh
$ echo abc.txt | sed 's/.*\.//'
# => txt
```

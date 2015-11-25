# stream editing
Any program in unix can be described as the following:
```js
input | transform | output
```
This section is about transforming data.

## stdin
This will output all data passed to stdin.
```sh
# transform.sh
while read stdin; do
  echo "$stdin"
done
```
Unix pipes now work:
```sh
$ ./transform.sh < ./my-file.log
$ echo 'hello world' | ./transform.sh
```

## tr

## sed
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

## awk
### select a column
```sh
$ echo 'hi there' | awk '{print $2}'    # single column
there
$ echo 'hi there' | awk '{print $2 $1}' # multi column
therehi
```

## See Also
- [idiomatic awk](http://backreference.org/2010/02/10/idiomatic-awk/)
- [world's best introduction to sed](http://www.catonmat.net/blog/worlds-best-introduction-to-sed/)

## grep
### inverse grep / exclude data
```sh
$ grep -v hello < ./file.txt   # exclude 'hello'
```

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
[tbi]

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

## file minification
__tar__
```sh
$ tar xzf out.tgz file1 file2
$ git ls-files | grep gateway | tar cfz target.tgz -T -
```
__zip__
```sh
$ zip out.zip file1 file2
```

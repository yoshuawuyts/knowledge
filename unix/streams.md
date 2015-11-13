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
  echo stdin
done
```
Unix pipes now work:
```sh
$ ./transform.sh < ./my-file.log
$ echo 'hello world' | ./transform.sh
```

## tr

## sed

## awk

# rename
Rename files based on patterns

## Usage
```sh
$ rename <expression> <files to target>
```

## Strip all leading numbers from files ending on .mp3
```sh
$ rename -n 's/\d{2}_(.*)/$1/' *.mp3
```

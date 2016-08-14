# jq

## Sort a nested object by a key name
Get the data for the key .meetings:
```sh
$ jq '.meetings'
```

Move down to the context of '.meetings' without modifying data:
```sh
$ jq '{meetings: .meetings}'
```

Sort the keys `.meetings.meetingname`:
```sh
$ jq '{meetings: .meetings | sort_by(.meetingName)}'
```

## strip quotes from output
```sh
$ jq -r '.version' < './package.json'
```

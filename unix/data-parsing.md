# data parsing
Parse data on the command line.

## json
To parse JSON use `jq`.

```sh
# Sort a nested object by a key name
$ jq '.meetings'  # get the data for the key .meetings
$ jq '{meetings: .meetings}' # move down to the context of '.meetings' without
                             # modifying data
$ jq '{meetings: .meetings | sort_by(.meetingName)}'  # sort by the key
                                                      # '.meetings.meetingName'
```

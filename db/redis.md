# Redis
Redis is a big collection of data structures that are held in memory and
flushed to disk every so often.

## CRUD
```sh
$ SET <key>  # set key, update if already exists
$ GET <key>  # get key
$ DEL <key>  # delete key
```

## Expire keys
Expire keys time out after a specified value. Imo it's best to set it in ms
rather than seconds to keep compat with unix.
```sh
$ PSETEX <key> <timeout> <value>   # set an expiry key in miliseconds
$ SETEX <key> <timeout> <value>    # set an expiry key in seconds
```

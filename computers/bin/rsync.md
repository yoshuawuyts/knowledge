# rsync
Remote sync utility, allows you to transfer files from a local dir to remote
dir. Unlike `scp(1)` it requires a daemon to run on the remote host, so it has
some overhead - tho it's worth it.

## Usage
```sh
$ rsync -a ./<local_dir> <remote>:~/<remote_outdir>  # sync to remote
$ rsync -a <remote>:~/remotedir ~/<local_outdir>     # sync from remote
```

## Flags
```txt
-r   # recurse
-a   # recurse and keep permissions, times, etc.
-z   # use compression
-P   # report progress, allow pausing

--delete    # delete files from dest if they've been removed locally
--exclude   # exclude files based on a filter
--include   # include files that have been excluded
```

## See Also
- https://www.digitalocean.com/community/tutorials/how-to-use-rsync-to-sync-local-and-remote-directories-on-a-vps

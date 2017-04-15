# tar
Create an archive by concatenating all files and do some indexing magic. The
arguments in `tar(1)` are positional, so remember that stuff like `-C` comes
after the filename, whereas `-czf` / `-xzf` come before the filename.

## Create archives
Archives are created relative to root `/` unless the `-C` flag is passed.
Creating a tar relative to root is useful for system images.
```sh
$ tar -czf archive.tgz ./my-dir       # Tar relative to root
$ tar -cvf archive.tgz -C ./my-dir .  # Tar relative to ./my-dir
$ tar -czf /tmp/my-dir .              # Tar the local dir to /tmp/my-dir
```

## Extracting archives
```sh
$ tar -xzf ./archive.tgz -C ./target-dir  # Untar to ./target-dir
```

## Merging archives
Remember that this will not work if you also gzip.
```sh
# slow
$ tar --concatenate -vf buffer1.tar buffer2.tar

# fast
$ cat buffer1.tar >> image.tar
$ cat buffer2.tar >> image.tar
```

## See Also
- http://stackoverflow.com/questions/939982/how-do-i-tar-a-directory-of-files-and-folders-without-including-the-directory-it
- https://superuser.com/questions/941475/concatenate-multiple-tar-files-in-one-command

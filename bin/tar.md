# tar
Create an archive by concatenating all files and do some indexing magic.

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

## See Also
- [stackoverflow](http://stackoverflow.com/questions/939982/how-do-i-tar-a-directory-of-files-and-folders-without-including-the-directory-it)

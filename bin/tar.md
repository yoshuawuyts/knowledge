# tar

## create a archive from a directory
Tar creates archives by default from the root `/` directory. This preserves
absolute paths, and if extracted to root, it should fall nicely in place. This
is ideal for creating deployment artifacts:
```sh
$ tar -czf archive.tgz ./my-dir
```

If you want to create an archive from a directory, you must pass the `-C` flag,
which changes the directroy from root. Then pass `.` to specify that the
current directory must be archived:
```sh
$ tar -czvf archive.tgz -C ./my-dir .
```
- [stackoverflow](http://stackoverflow.com/questions/939982/how-do-i-tar-a-directory-of-files-and-folders-without-including-the-directory-it)

## extract archive into directory
```sh
$ tar -xzf ./archive.tgz -C /target/directory
```

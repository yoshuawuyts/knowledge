# diff
Tools to manage changes with the diff file format.

## diff(1)
Compare files line by line in the diff format.

## diff3(1)
Compare three files line by line in the diff format.

## patch(1)
Apply a diff file to an original. Takes a patch file produced by the diff
command.

## git(1)
Version control systems (VCS) were built out of a historical frustration with
manually creating, mailing and applying patches. Git has appeared as the clear
winner in the VCS landscape, and uses the `diff` format to show changes between
commits.

## diffstat(1)
Pretty diff graphs. Example:
```txt
$ git diff | diffstat
 index.js |    1 -
 1 file changed, 1 deletion(-)
```

## terraform(1)
Produces diff output for infrastructure.

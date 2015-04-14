# git

### Remove a current tracking relationship
```sh
$ git branch --unset-upstream
```
[source](http://www.lornajane.net/posts/2014/understanding-tracking-branches-in-git)


### Cryptographically sign all commits
```text
commit.gpgsign
```
[source](http://stackoverflow.com/a/20628543/1541707)

## Revert a single file with uncommitted changes to HEAD
```sh
git checkout <filename>
```

## Unstage a file
```sh
$ git reset HEAD <file>
```

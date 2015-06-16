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
$ git checkout <filename>
```

## Unstage a file
```sh
$ git reset HEAD <file>
```

## Print changed files in last commit
```sh
$ git show --name-only [commit]
```

## Undo commit and keep changes
```sh
$ git reset --soft @~1
```
[source](http://stackoverflow.com/a/20066339/1541707)

## Merge patches like you're a kernel maintainer
Kernel maintainers use the `git am` (apply-mail) command to get patches from a
mailing list. GitHub has extended this command in `hub` to work with PR urls.
This allows you to omit those nasty merge commits and update / prune files
where needed.
```sh
$ hub am -3 <pull-request-url>
```
- [merge PR considered harmful](http://blog.spreedly.com/2014/06/24/merge-pull-request-considered-harmful/)

## See Also
- [how to undo almost anything with git](https://github.com/blog/2019-how-to-undo-almost-anything-with-git)
- [git koans](http://stevelosh.com/blog/2013/04/git-koans/)

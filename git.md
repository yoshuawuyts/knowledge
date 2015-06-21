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

## branching model
`git-flow` is probably the most popular branching technique out there. It's
also shit. There are many caveats in it, and it doesn't use some use git
primitives where it should. A succesful branching model for simple projects is:
- master is the truth of all merges
- each feature gets their own branch on the contributor's fork
- whenever a feature is done, it's pulled in
- master gets a tag

It doesn't need to be complex to be efficient. If hotfixes need to be
performed, master can be rebased on top of that. If features need to be
removed, just request the commit range for certain tag to roll back. Tada!

- [git flow considered harmful](http://endoflineblog.com/gitflow-considered-harmful)
- [follow up: git flow considered harmful](http://endoflineblog.com/follow-up-to-gitflow-considered-harmful)
- [chronological git history is silly](https://news.ycombinator.com/item?id=9745966)

## See Also
- [how to undo almost anything with git](https://github.com/blog/2019-how-to-undo-almost-anything-with-git)
- [git koans](http://stevelosh.com/blog/2013/04/git-koans/)

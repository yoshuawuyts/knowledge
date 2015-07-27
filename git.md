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

## Git hooks
Git hooks are a way of extending git with functionality whenever things happen.
To some extent `npm` has a similar mechanism called `npm scripts`. Git hooks
are just regular bash scripts that are triggerd on actions. They live in
`.git/hooks`, where the filename is the name of the hook. The following hooks
are available:
```txt
applypatch-msg
pre-applypatch
post-applypatch
pre-commit
prepare-commit-msg
commit-msg
post-commit
pre-rebase
post-checkout
post-merge
pre-receive
update
post-receive
post-update
pre-auto-gc
post-rewrite
pre-push
```

Git hook scripts cannot be checked into version control. If you want to persist
scripts between multiple users you must check them into the project itself and
then symlink them back in, preferably in a bootstrap script. Be careful when
running hard symlinks though, as they will overwrite any local files. Instead
create an aggregator script that moves the original scripts and then imports
the original + repo scripts back in.

Example functionality for git hooks includes: checking code style on commit,
running extended tests on rebase to master, verify that no commit is performed
while rebasing, semver is updated, etc.
- [githooks](http://githooks.com/)
- [putting git hooks into repository](http://stackoverflow.com/questions/3462955/putting-git-hooks-into-repository)

## git bisect
Find by binary search the change that introduced a bug.

- [git_bisect - linus torvalds](http://yarchive.net/comp/linux/git_bisect.html)

## Manage main and fix commits
By running `--fixup` you can create fix commits for your main commit; this is a
better alternative to continuously rebasing on top of your previous commit.
With `--autosquash` the `--fixup` commits are automatically squashed into their
relevant commit.
```sh
$ git commit --fixup <commit-sha>   # automatically marks your commit as a fix
                                    # of a previous commit
$ git rebase -i --autosquash        # automatically organize merging of these
                                    # fixup commits and associated normal
                                    # commits
```
- [stackoverflow/easily-fixup-past-commit](http://stackoverflow.com/questions/3103589/how-can-i-easily-fixup-a-past-commit)
- [git-fixup](https://github.com/deiwin/git-dotfiles/blob/docs/bin/git-fixup)
- [keep your branch clean with git fixup and autosquash](http://fle.github.io/git-tip-keep-your-branch-clean-with-fixup-and-autosquash.html)

## See Also
- [how to undo almost anything with git](https://github.com/blog/2019-how-to-undo-almost-anything-with-git)
- [git koans](http://stevelosh.com/blog/2013/04/git-koans/)
- [jonathanong/git-style-guide](https://github.com/jonathanong/git-style-guide)

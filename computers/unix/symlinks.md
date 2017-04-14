# symlinks

Directory structures in Unix are very similar to an API. All logs end up in
`/var/log`, all dotfiles in `~/`, etc. etc. Much of this stuff can
be found in the [filesystem hierarchy standard][fhs].

The point here is that despite files being quite standard, they sometimes
deviate a bit. If you're checking in dotfiles to source control for example,
do they belong in your `~/Repositories` folder or in `~/`? Eventually they
should be in both, and symlinks let you do just that.

Symlinks are also amazing for local development. [Someone][large-apps] said:

> 'The secret to building large apps is never build large apps.'

That means building small parts that work together.
But you also sometimes need to build parts in parallel without any friction.
Again: symlinks to the rescue! That way you can build different moving parts in
a separate section, and then symlink them into the place the fileystem expects
them to be.

## See also
- [linklocal](http://github.com/timoxley/linklocal) (node)

[fhs]: http://www.pathname.com/fhs/
[large-apps]: https://github.com/timoxley/best-practices#never-build-large-apps

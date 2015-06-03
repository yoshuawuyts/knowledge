# vim
## Resources
- [vim cheat sheet](http://vim.rtorr.com/)

## Load vim with no plugins enabled
```sh
vim -u NONE -N
```
[source](http://stackoverflow.com/questions/4261785/temporarily-disable-some-plugins-using-pathogen-in-vim)


## Delete without overriding your last yank
All yank and delete operations write to the unnamed register by default. However, the most recent yank and most recent delete are always stored (separately) in the numbered registers. The register 0 holds the most recent yank. The registers 1-9 hold the 9 most recent deletes (with 1 being the most recent).

In other words, a delete overwrites the most recent yank in the unnamed register, but it's still there in the 0 register. The blackhole-register trick (`"_dd`) mentioned in the other answers works because it prevents overwriting the unnamed register.

You reference a register using double quotes, so pasting the most recently yanked text can be done like this:
```txt
"0p
```
This is an excellent reference:

- http://blog.sanctum.geek.nz/advanced-vim-registers/

[source](http://stackoverflow.com/a/14241768/1541707)

## Commenting lines
Using the [`vim-commentary`](https://github.com/tpope/vim-commentary) plugin:
```txt
gcc  comment out a line
gc   comment out the target of a motion
gcap comment out a paragraph
```

## Surrounding
Surrounding words with quotes, brackets, XML tags and more can be done with
the [`vim-surround`](https://github.com/tpope/vim-surround) plugin.
```txt
cs"'  change the surroundings from " to '
cs'   delete the ' surroundings
ysiw] change surroundings to ]
```

## Substitute multiple variants of a word
With [`vim-abolish`](https://github.com/tpope/vim-abolish) plugin installed:
```txt
:%Subvert/facilit{y,ies}/building{,s}/g
```

## Faster actions
Use the [`vim-unimpaired`](https://github.com/tpope/vim-unimpaired) plugin to
quickly access pairs of mappings (new lines, next files, etc.)

## Marks
Marks are a dope way to navigate between files or within a file. If Ctags
aren't working as planned, having marks is pretty neat. It's also neat when
there's a large file in which you have to jump between sections (and `(` and `)`
won't do).
```text
ma      set mark at current cursor position
'a      jump to location of mark a
`a      jump to position (line and column) of mark a
:marks  list all current marks
```

## Stash vim session
In vim do `C-z` to stash the window in the background. To pop it run:
```sh
$ fg
```

## Undo trees
With [gundo](https://github.com/sjl/gundo.vim/) installed:
```txt
GundoShow
```

## Delete all lines that match a pattern
```txt
:g/<regex>/d
```

## Stop using hjkl
Using hjkl is an antipattern as it's slow. Instead use the other motions.

- [article](http://vimcasts.org/blog/2013/02/habit-breaking-habit-making/)

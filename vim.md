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

## Set line numbers
```viml
:set nu    "enable numbers
:set nonu  "disable number
:set nu!   "toggle
```

## Sort lines
```viml
:sort u  "alphabetically
```

## Split windows
```viml
Cw-s  "split window horizontally
Cw-v  "split window vertically
```

## Buffers
Buffers are often a misunderstood / underused feature of vim, even though they
are essential to creating a fast workflow. Many editors use tabs to manage open
files, but with `vim` it's recommended to use buffers for this. Vim also has
tabs, but they're used differently.
```txt
tabs     " open one per feature; workspaces
buffers  " open one per file, nested under a workspace
```

Tabs can be used as different views on buffers. Buffers are used to manage
files. By themselves buffers are a bit tedious to work with, but by using
[`ctrlp`](https://github.com/ctrlpvim/ctrlp.vim) they become more usable
through fuzzy finding.

A good mapping for `ctrlp` is:
```viml
map <Leader>b :CtrlPBuffer<CR>  " Where leader is ideally set to spacebar
```

### buffer commands
```viml
enew        " open an empty buffer
bd          " close a buffer
<bufno> bd  " close a specific buffer
ls          " list open buffers
ls!         " list all open buffers (including unlisted)
```

- [tabs vs buffers madness](https://joshldavis.com/2014/04/05/vim-tab-madness-buffers-vs-tabs/)
- [how do you prefer to switch buffers](http://stackoverflow.com/questions/327411/how-do-you-prefer-to-switch-between-buffers-in-vim)
- [ctrlpvim/ctrlp.vim](https://github.com/ctrlpvim/ctrlp.vim)

## Visual block mode
Visual block mode is useful for bulk changes to patterns that are hard to capture
with a regex; usually columns.
```vim
C-v  " enter visual block mode
A    " enter insert mode
I    " enter insert mode
esc  " execute changes made in insert mode
```

## Format text to fit lines
With `:set tw=80` do:
```viml
gq
```

## Jump to character
```viml
f<char>  " jump to character
F<char>  " jump back to char
t<char>  " jump to char before match
;        " repeat jump
,        " repeat jump back
```

## See Also
- [how to boost your vim productivity](http://sheerun.net/2014/03/21/how-to-boost-your-vim-productivity/)

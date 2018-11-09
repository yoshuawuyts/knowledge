# vim
## Resources
- [vim cheat sheet](http://vim.rtorr.com/)

## Load vim with no plugins enabled
```sh
vim -u NONE -N
```
[source](http://stackoverflow.com/questions/4261785/temporarily-disable-some-plugins-using-pathogen-in-vim)


## Delete without overriding your last yank
All yank and delete operations write to the unnamed register by default.
However, the most recent yank and most recent delete are always stored
(separately) in the numbered registers. The register 0 holds the most recent
yank. The registers 1-9 hold the 9 most recent deletes (with 1 being the most
recent).

In other words, a delete overwrites the most recent yank in the unnamed
register, but it's still there in the 0 register. The blackhole-register trick
(`"_dd`) mentioned in the other answers works because it prevents overwriting
the unnamed register.

You reference a register using double quotes, so pasting the most recently
yanked text can be done like this:
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
m<char>  set mark at current cursor position
ma       set mark at a, bound to file
mA       set mark to A, bound to project
'a       jump to location of mark a
`a       jump to position (line and column) of mark a
:marks   list all current marks
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
b <bufno>   " open buffer number
b#          " open last buffer (or <C-^> / <C-6>)
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

## Folds
```viml
zc " close a fold
zC " close all folds
zo " open a fold
zO " open all folds under selection
za " toggle a fold
zA " toggle all folds under selection
zM " Close all folds in the file ("m" stands for "More")
zm " Close folds in the file level by level
zR " Open all folds in the file ("r" stands for "Reduce")
zr " Open folds in the file level by level

set foldlevel=0 " Set the indentation level at which folds should start
```
- https://stackoverflow.com/questions/4559571/how-can-i-automatically-fold-all-functions-in-a-file-with-vim#4559609
- https://vi.stackexchange.com/questions/4627/change-what-vim-displays-when-there-is-a-fold#4628
- https://vi.stackexchange.com/questions/6635/get-rid-of-extra-fold-dashes-when-using-vim-folding
- https://github.com/chrisbra/vim_dotfiles/blob/a1a106081884647922395fe4512aa8fcc887d64c/plugin/CustomFoldText.vim

## Macros
```vim
q<key>          record macro at <key>
q               stop recording
@<key>          play back macro
:<num> @<key>   play back macro <num> time
```

## Open url under cursor in browser
```vim
gx
```

## Disable conceal
In 2011 vim added a feature called `conceal` which allows syntax parts to be
shadowed, allowing "nicer" views. For some people (like me) it feels as if
reality is distorted, and want to disable this. Luckily that's possible through
a setting:
```vim
set cole=0   " disable conceal
set cole=2   " enable conceal
```

## Stop wrapping lines
Or doing linebreaks
```viml
:set nowrap
```

## Autocommands
Sometimes you wanna script a thing when you create a new file or start editing
one; you can trigger stuff using autocommands:
```viml
autocmd BufNewFile,BufReadPost *.md :Goyo
```

## Disable `.viminfo`
```viml
set viminfo="NONE"
```

## Enable Spellcheck
Enable spellchecking in Vim.
```viml
set spell
```

```txt
z=  " View suggestions for word.
zg  " Mark a word as OK.
zw  " Mark a word as Not OK.
```

## Replacing multiple cursors
Multiple cursors isn't needed when using vim. Instead there's a combination of
methods to apply the same functionality using stock options.

```txt
* (normal mode)  " Find all definitions for the current word under the cursor.
g* (normal mode) " Same as `*`, but with a less strict filter.
g# (normal mode) " Same as `*`, but with a less strict filter.
cgn              " Go to the next definition of the word, and edit it.
.                " Replay the previous replacement on the next word.
gn               " Go to the next definition of the word (without replacing).
```

The flow to replace multiple cursors then becomes:
- Search for the definition under the cursor using `*` or `g*`.
- Apply `cgn` to enter edit mode on the next node.
- Replay action as many times as you need using `.`.


### Key Binding
Replacing values in a file is a relatively common action. To speed this up it
might be useful to bind it to a key or key combination. The following snippet
edits the word under the cursor, and allows replaying the action on next
instances using `n` and `.`. The key feature here is that it can skip over false
positives, which makes it highly flexible.

```vim
nmap ! g*Ncw
```

### See Also
- https://medium.com/@schtoeffel/you-don-t-need-more-than-one-cursor-in-vim-2c44117d51db
- http://vim.wikia.com/wiki/Search_and_replace_the_word_under_the_cursor
- http://vim.wikia.com/wiki/Search_and_replace#substitute_last_search

## Jump between panes
Pane switching doesn't need to be a pain.
```vim
C-w C-w  " Jump to next pane in order.
C-w C-j  " Jump to bottom pane.
C-w C-k  " Jump to top pane.
C-w C-h  " Jump to left pane.
C-w C-l  " Jump to right pane.
```

## Hex mode
Sometimes you need to edit binary files. That's what hex mode is for. The
recommended way to do this is by installing a hex mode plugin. If you want to do
it manually, it can be done by calling out to `%!xxd`.

```vim
%!xxd        " Enter the hex mode editor
:set binary  " Prevent errors in hex mode
%!xxd -r     " Exit hex mode
```

But it's better to install the [hexmode](https://github.com/fidian/hexmode) vim
plugin. Map it to `Command + H` with the following key bindings:
```vim
nnoremap <C-H> :Hexmode<CR>
inoremap <C-H> <Esc>:Hexmode<CR>
vnoremap <C-H> :<C-U>Hexmode<CR>
```

And to auto-detect it for certain files:
```
let g:hexmode_patterns = '*.bin,*.exe,*.dat,*.o'
```

- https://github.com/fidian/hexmode
- http://vim.wikia.com/wiki/Improved_Hex_editing

## Select Line Without Newline
```viml
vgl  " select line without newline
```
- https://stackoverflow.com/a/41456368/1541707

## Align tables
- https://github.com/junegunn/vim-easy-align
```viml
; Align tables using <Leader> + \
au FileType markdown vmap <Leader><Bslash> :EasyAlign*<Bar><Enter>
```


## See Also
- [how to boost your vim productivity](http://sheerun.net/2014/03/21/how-to-boost-your-vim-productivity/)
- [vim for writing](https://www.swamphogg.com/2015/vim-setup/)
- [learn viml the hard way](http://learnvimscriptthehardway.stevelosh.com/chapters/12.html)
- [spell checking for vim](https://robots.thoughtbot.com/vim-spell-checking)

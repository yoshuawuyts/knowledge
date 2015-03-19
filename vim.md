# vim

## Load vim with no plugins enabled
```sh
vim -u NONE -N
```
[source](http://stackoverflow.com/questions/4261785/temporarily-disable-some-plugins-using-pathogen-in-vim)


## Delete without overriding your last yank
All yank and delete operations write to the unnamed register by default. However, the most recent yank and most recent delete are always stored (separately) in the numbered registers. The register 0 holds the most recent yank. The registers 1-9 hold the 9 most recent deletes (with 1 being the most recent).

In other words, a delete overwrites the most recent yank in the unnamed register, but it's still there in the 0 register. The blackhole-register trick (`"_dd`) mentioned in the other answers works because it prevents overwriting the unnamed register.

You reference a register using double quotes, so pasting the most recently yanked text can be done like this:
```vi
"0p
```
This is an excellent reference:

- http://blog.sanctum.geek.nz/advanced-vim-registers/

[source](http://stackoverflow.com/a/14241768/1541707)

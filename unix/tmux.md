# tmux

## Selections
The current pane id is available as `$TMUX_PANE`. For example to full screen
the current pane, do:
```sh
$ tmux resize-pane -t $TMUX_PANE -Z
```

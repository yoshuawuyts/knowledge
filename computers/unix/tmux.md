# tmux

## Selections
The current pane id is available as `$TMUX_PANE`. For example to full screen
the current pane, do:
```sh
$ tmux resize-pane -t $TMUX_PANE -Z
```

## Base settings
```tmux
bind j select-pane -D
bind k select-pane -U
bind h select-pane -L
bind l select-pane -R

bind | split-window -h
bind - split-window -v

bind -r J resize-pane -D 5
bind -r K resize-pane -U 5
bind -r H resize-pane -L 5
bind -r L resize-pane -R 5

set -g pane-border-fg white
set -g pane-active-border-fg cyan
set -g pane-active-border-bg cyan
```

## Copy mode
```tmux
PREFIX [  # enter copy mode
?         # search
/         # search
Enter     # grab text
Space     # grab text
PREFIX ]  # paste text
```

## See Also
- http://danielallendeutsch.com/blog/16-using-tmux-properly.html

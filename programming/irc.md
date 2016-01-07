# irc

## installation
`weechat` is the best CLI client as it integrates very nicely with bitlbee. To
enable plugins, we must set some flags during compilation. On OS X do:
```txt
$ brew install weechat --with-perl --with-python
```

## commands
```txt
/part <msg>   # leave a channel
/away         # set yourself as away
```

## weechat
```txt
/window merge        # close a window without closing buffers; merges all
/window splitv 50    # split window vertically
```

### enable mouse support
```txt
/set weechat.look.mouse on
/mouse enable
```
- [mouse support](http://dev.weechat.org/post/2011/07/26/Mouse-support-and-free-movement-of-cursor)

### save layout
install `autojoin` (`/plugin install autojoin.py`)
```txt
/save   # store layout
```
- [weechat quickstart guide](https://weechat.org/files/doc/devel/weechat_quickstart.en.html)
- [weechat bitlbee guide](http://zanshin.net/2015/01/10/a-guide-for-setting-up-weechat-and-bitlbee/)
- [pascal poitras weechat config](https://pascalpoitras.com/my-weechat-configuration/)

## bitlebee
```txt
acc add twitter yoshuawuyts   # add yoshuawuyts on twitter
acc twitter on                # activate twitter account
```

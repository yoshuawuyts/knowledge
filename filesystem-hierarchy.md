# Filesystem hierarchy

## Unix Filesystem Hierarchy Standard [(wiki)][fhs]
```
/              Root dir
/bin           Binaries
/dev           Devices
/etc           Host-specific system-wide config
/etc/opt       Add-on config
/etc/X11       X window system config
/home          User home dir
/lib           Libraries for binaries
/media         Removable media
/mnt           Temporary fs's
/opt           Optional apps
/proc          Virtual fs for process & kernel
/root          Home dir of root user
/sbin          Essential system binaries
/srv           Site-specific data served by system
/tmp           Temporary files
/usr           Secondary hierarcy for read-only user data
/usr/bin       Non-essential binaries for all users
/usr/include   Standard include files
/usr/local     Tertiary hierarchy for local data
/usr/sbin      Non-essential system binaries (daemons)
/share         Architecture independent shared data
/src           Source code
/var           Variable files
/var/cache     Application cache data
/var/lib       State information. Used by db's and the like.
/var/lock      Lock files. Which resources are used.
/var/log       Log files
/var/mail      Mailboxes
/var/opt       Variable data from `/opt/`
/var/run       Information about running system since boot
/var/spool     Tasks waiting to be processed
/var/tmp       Temp files preserved between boots
```

## Node Module Filesystem Hierarchy
```
bin              -> executable
docs             -> documentation
lib              -> source files
test             -> tests
```

## Project Filesystem Hierarchy
```
bin              -> executable
docs             -> documentation
lib              -> source files
pkg              -> packages (local modules)
sh               -> shell scripts
test             -> integration tests
```

## See also
- [Filesystem Hierarchy Standard][fhs]

[fhs]: http://en.wikipedia.org/wiki/Filesystem_Hierarchy_Standard

## Personal filesystem hierarchy
Or how to structure your personal documents well. This is what I'm going for now,
maybe some other programmers might find it useful as well! All my files are stored
in `Google Drive(3)` and symlinked from `~/mnt/Google Drive` to `~/`.

```
etc/              other
  pwd/            `pass(1)` password files
lib/              resources that can be consumed
log/              scanned materials, sorted by date (2011, 2012, etc.)
media/            media
  cam/            img from camera
  read/           books, articles and the like
  screen/         screenshots
  tmp/            images that don't belong in any of the above
  vid/            movies, gifs
  wall/           wallpapers
  write/          stories, notes, posts
tmp/              temporary files
src/              source code
usr/              personal files, sorted by topics
  accounts/       account info & restoration files
  img/            avatar pictures
```

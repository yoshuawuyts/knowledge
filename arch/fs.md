# Filesystem hierarchy

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


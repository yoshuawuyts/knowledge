# locales

## List locales
```sh
$ locale
```

## Generating locales
Missing locales are generated with locale-gen:
```sh
$ locale-gen en_US.UTF-8
```

## Create a locale
```sh
$ sudo localedef -i en_US -f UTF-8 en_US.UTF-8
```

## See Also
- [resolve locale settings on linux](https://xuri.me/2015/09/06/resolve-setting-locale-failed-on-linux.html)

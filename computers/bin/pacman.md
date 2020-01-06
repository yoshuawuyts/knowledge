# pacman

```txt
$ sudo pacman -Qi <package>         # Check which packages depend on this package
$ sudo pacman -R <package>          # Remove package
$ sudo pacman -Qqdt                 # List packages that are orphaned
$ sudo pacman -Rns $(pacman -Qqdt)  # Remove all orphaned packages
```

# Upgrade PGP Key Signatures

```sh
$ sudo pacman -Sy archlinux-keyring && pacman -Syyu
```

# permissions

## users
### create user
Either `useradd` or `adduser`
```sh
$ adduser <name>
```

## groups
### create group
```sh
$ groupadd <name>
```

### add user to group
```sh
$ sudo usermod -G <group> <user>
```

## passwords
### edit password
```sh
# opens interactive session
$ sudo passwd <user>
```

## namespaces
[tbi]

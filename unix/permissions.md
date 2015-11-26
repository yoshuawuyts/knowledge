# permissions
Manage who can do what on the system.

## users
### create user
Either `useradd` or `adduser`
```sh
$ sudo adduser <name>
$ sudo mkdir /home/<name>   # create a home dir for user
```

## groups
Groups have combined settings; individual users can be added to groups which
then inherit the permissions of the group.

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

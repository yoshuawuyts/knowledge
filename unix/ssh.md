# ssh
SSH is the _secure shell protocol_. It allows you to attach your terminal
window to a remote server and execute commands in it. It's highly useful.

## Attach to remote server
```sh
$ ssh <username>@<ip>
```

## Connect to ssh server
```sh
ssh -i <path/to/file> <name>@<ip>
```
or with a `~/.ssh/configfile`
```
ssh <Host>
```

## Managing configuration
`ssh(1)` obtains configuration data from the following sources in the following
order:
1.   command-line options
2.   user's configuration file (~/.ssh/config)
3.   system-wide configuration file (/etc/ssh/ssh_config)
```txt
# ~/.ssh/config
Host SERVER1
   IdentitiesOnly yes
   IdentityFile ~/.ssh/id_rsa_SERVER1

Host SERVER2
   IdentitiesOnly yes
   IdentityFile ~/.ssh/id_ed25519_SERVER2

Host server1
  HostName server1.cyberciti.biz
  User nixcraft
  Port 4242
  IdentityFile /nfs/shared/users/nixcraft/keys/server1/id_rsa
```
- [create ssh config file on linux](http://www.cyberciti.biz/faq/create-ssh-config-file-on-linux-unix/)

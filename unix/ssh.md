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

## Add SSH key to server
```sh
$ ssh-keygen -f ~/.ssh/<key-name>         # interactively generate keys
$ ssh-copy-id -i <ssh-keyfile> <remote>   # copy key to remote
```
- [3-steps-to-ssh](http://www.thegeekstuff.com/2008/11/3-steps-to-perform-ssh-login-without-password-using-ssh-keygen-ssh-copy-id/)

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

## Files
- `~/.ssh`: holds all `ssh` configuration
- `~/.ssh/known_hosts`: connect to a server, make sure it's not an
  impersonator.
- `~/.ssh/authorized_keys`: let the server authenticate the user.

## Copying files
### rsync
`rsync` is probably the fastest way of getting files across, _but_ it has one
major caveat: it needs to be installed on both sides. If that's the case then
copying files over is easy-peasy.

```sh
# recursively copy files to remote
$ rsync -r -e ssh <username>@<remote>:<path> <files-to-copy>
$ rsync -r -e ssh foobar@127.0.0.1:/~ ./my-dir
```

To specify the location of `rsync` on the remote you can pass in the
`--rsync-path=` flag.

### scp
Secure copy is a less performant alternative to `rsync` but does not require to
be installed on both sides. On the flip side: it acts more as an extension to
`ssh` than `rsync` by allowing similar configuration to be passed in.

`scp` reads commands from `source > destination`, and thus allows copying files
from remote to remote.
```sh
$ scp [opts] <source> <destination>
$ scp <files-to-copy> <user>@<remote>:<path>         # copy a file
$ scp -i ./linux/id_rsa ./file.dat user@10.0.0.1:~/  # use an ssh id file
$ scp -r [!.]* user@10.0.0.1:~/   # copy dir recursively excluding dotfiles
```

To copy a file with an intermediate host using `scp`:
[ tbi ]
- [stackoverflow/intermediate-host-copy](http://superuser.com/questions/276533/scp-files-via-intermediate-host)

### connection multiplexing
Multiple connections can be shared using the `-M` flag.

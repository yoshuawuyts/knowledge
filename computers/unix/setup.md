# setup
There's a few steps you need to take to configure your server.

## Usage
```sh
$ ssh root@<server_ip>              # Login to server
$ adduser addmin --ingroup admin    # Create user named admin
$ usermod -aG sudo admin            # Give sudo privilege to admin

# setup ssh keys

# disable password login
$ sed -ie 's/PasswordAuthentication yes/PasswordAuthentication no/' /etc/ssh/sshd_config
```

## See Also
- https://www.digitalocean.com/community/tutorials/initial-server-setup-with-ubuntu-16-04
- https://www.digitalocean.com/community/tutorials/how-to-edit-the-sudoers-file-on-ubuntu-and-centos

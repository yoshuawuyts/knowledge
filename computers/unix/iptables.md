# iptables
Iptables is the port configuration tool for linux.

## Persisting iptables rules
Iptables does not persist rules by default. To persist them we must install a
tool called `iptables-persistent`.

```sh
$ sudo apt-get install iptables-persistent
$ sudo service iptables-persistent start
```

## Open ports for mosh
```sh
$ sudo iptables -I INPUT 1 -p udp --dport 60000:61000 -j ACCEPT
```

## See Also
- https://www.digitalocean.com/community/tutorials/how-to-install-and-use-mosh-on-a-vps
- https://www.digitalocean.com/community/tutorials/how-to-set-up-a-firewall-using-iptables-on-ubuntu-12-04

# port knocking
Hide your SSH daemon by requireing remotes open connections to a sequence of
ports first.

## Configuring iptables
```sh
# Accept all loopback traffic
$ sudo iptables -A INPUT -i lo -j ACCEPT

# Allow traffic, related to established connections (so SSH continues to work)
$ sudo iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT

# Allow TCP traffic on port 80
$ sudo iptables -A INPUT -p tcp --dport 80 -j ACCEPT

# Drop everything that isn't allowed
$ sudo iptables -A INPUT -j DROP
```

## See Also
- https://www.digitalocean.com/community/articles/how-to-set-up-a-firewall-using-ip-tables-on-ubuntu-12-04
- https://www.digitalocean.com/community/tutorials/how-to-use-port-knocking-to-hide-your-ssh-daemon-from-attackers-on-ubuntu

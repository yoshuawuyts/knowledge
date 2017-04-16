# port knocking
Hide your SSH daemon by requireing remotes open connections to a sequence of
ports first.

## Firewall configuration
```sh
# Accept all loopback traffic
$ sudo iptables -A INPUT -i lo -j ACCEPT

# Allow traffic, related to established connections (so SSH continues to work)
$ sudo iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT

# Allow TCP traffic on port 80
$ sudo iptables -A INPUT -p tcp --dport 80 -j ACCEPT

# Drop everything that isn't allowed
$ sudo iptables -A INPUT -j DROP

# List all rules
$ sudo iptables -S

# Persist iptables rules
$ sudo apt-get install iptables-persistent
$ sudo service iptables-persistent start
```

## Knocking configuration
Port knocking can either be done through `knockd` or directly through
`iptables`.
```sh
$ sudo apt-get install knockd
```
`/etc/knockd.conf`
```txt
[options]
    UseSyslog

[SSH]
    sequence = 5438,3428,3280,4479
    tcpflags = syn
    seq_timeout = 15
    start_command = /sbin/iptables -I INPUT 1 -s %IP% -p tcp --dport 22 -j ACCEPT
    cmd_timeout = 10
    stop_command = /sbin/iptables -D INPUT -s %IP% -p tcp --dport 22 -j ACCEPT
```

## Configuring iptables
```sh
sudo iptables -P INPUT ACCEPT
sudo iptables -P FORWARD ACCEPT
sudo iptables -P OUTPUT ACCEPT
sudo iptables -F

sudo iptables -N KNOCKING
sudo iptables -N GATE1
sudo iptables -N GATE2
sudo iptables -N GATE3
sudo iptables -N PASSED

sudo iptables -A INPUT -m conntrack --ctstate ESTABLISHED,RELATED -j ACCEPT
sudo iptables -A INPUT -i lo -j ACCEPT
sudo iptables -A INPUT -p protocol --dport port -j ACCEPT
sudo iptables -A INPUT -p tcp --dport 80 -j ACCEPT
sudo iptables -A INPUT -j KNOCKING

sudo iptables -A GATE1 -p tcp --dport 1111 -m recent --name AUTH1 --set -j DROP
sudo iptables -A GATE1 -j DROP
sudo iptables -A GATE2 -m recent --name AUTH1 --remove
sudo iptables -A GATE2 -p tcp --dport 2222 -m recent --name AUTH2 --set -j DROP
sudo iptables -A GATE2 -j GATE1
sudo iptables -A GATE3 -m recent --name AUTH2 --remove
sudo iptables -A GATE3 -p tcp --dport 3333 -m recent --name AUTH3 --set -j DROP
sudo iptables -A GATE3 -j GATE1
sudo iptables -A PASSED -m recent --name AUTH3 --remove
sudo iptables -A PASSED -p tcp --dport 22 -j ACCEPT
sudo iptables -A PASSED -j GATE1
sudo iptables -A KNOCKING -m recent --rcheck --seconds 30 --name AUTH3 -j PASSED
sudo iptables -A KNOCKING -m recent --rcheck --seconds 10 --name AUTH2 -j GATE3
sudo iptables -A KNOCKING -m recent --rcheck --seconds 10 --name AUTH1 -j GATE2
sudo iptables -A KNOCKING -j GATE1
```

## Knocking
```sh
#!/bin/bash -e

ports="1111 2222 3333"
host="your_server"

for x in $ports; do
  nmap -Pn --host_timeout 201 --max-retries 0 -p $x $host
  sleep 1
done
ssh user@${host}
```

## See Also
- https://www.digitalocean.com/community/articles/how-to-set-up-a-firewall-using-ip-tables-on-ubuntu-12-04
- https://www.digitalocean.com/community/tutorials/how-to-use-port-knocking-to-hide-your-ssh-daemon-from-attackers-on-ubuntu
- https://wiki.archlinux.org/index.php/Port_knocking
- https://www.digitalocean.com/community/tutorials/how-to-configure-port-knocking-using-only-iptables-on-an-ubuntu-vps

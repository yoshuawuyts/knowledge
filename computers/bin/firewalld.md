# firewalld
```sh
$ sudo firewall-cmd --state          # verify the service is running and reachable
$ firewall-cmd --get-default-zone    # see which zone is currently selected
$ firewall-cmd --get-active-zones    # see which zones are active
$ sudo firewall-cmd --list-all       # print default configuration
$ firewall-cmd --get-zones           # list available zones

$ firewall-cmd --get-services                       # list available services
$ sudo firewall-cmd --list-services                 # list enabled services
$ sudo firewall-cmd --add-service=http              # temporarily add http service
$ sudo firewall-cmd --add-service=http --permanent  # permanently add http service

$ sudo firewall-cmd --zone=public --add-port=5000/tcp
$ sudo firewall-cmd --zone=public --list-ports
$ sudo firewall-cmd --zone=public --add-port=4990-4999/udp
$ sudo firewall-cmd --zone=public --permanent --add-port=5000/tcp
$ sudo firewall-cmd --zone=public --permanent --add-port=4990-4999/udp
$ sudo firewall-cmd --zone=public --permanent --list-ports
```
- https://www.digitalocean.com/community/tutorials/how-to-set-up-a-firewall-using-firewalld-on-centos-7

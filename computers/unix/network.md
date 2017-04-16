# networking
Unix networking stuff.

## TCP / IP networking model
```
layer 0
  physical layer
  standards for voltages, frequencies and other physical properties

layer 1
  link layer
  communication with devices that share a physical connection
  Ethernet, 802.11a/b/g/n, and Token Ring

layer 2
  network layer
  talk to machines without knowing where they are physically located
  IP

layer 3
  transport layer
  determine which program to deliver messages to, usually through port numbers
  TCP, UDP

layer 4
  application layer protocol
  set of protocols for a specific program
  HTTP, SMTP, IMAP, POP3
```

On top of that there are an infinite amount of layers, most commonly:
```
layer 5
  security layer
  make sure communication is private
  TLS, SSL

layer 6
  API layer
  talk to a program living within the application layer
  Twitter API, GitHub API
```

But these layers are not fixed, sometimes protocols switch between layers. OSI,
for example, has 7 layers.

- [quora/http-tcp](http://www.quora.com/What-is-the-difference-between-HTTP-protocol-and-TCP-protocol)

## UDP
- unreliable
- little overhead

## TCP
- reliable (retries)
- overhead

## nc(1)
`nc` is the `netcat` command. It does anything surrounding TCP and UDP /
Datagram.

### create a direct chat connection
Implement a rudimentary IRC chat using netcat. When a connection is
established, messages will be echoed on both sides.
__terminal 1__
```sh
$ nc -l 1337  # listen on port 1337
```
__terminal 2__
```sh
$ nc localhost 1337
```

### Create a proxy
```sh
#!/bin/sh
# proxy.sh

url=$2
```
```sh
$ nc -lk 127.0.0.1 80 | ./proxy.sh
```

### Port scanning
Using netcat:
```sh
$ nc -z host.example.com 20-30
```

## DNS
DNS is about naming things and pointing them in the right direction. DNS uses
"records" to store this relation. There's different types of records:
- __MX:__ mail exchange record, specify a mail server that can accept emails
- __A:__ return a 32-bit IPv4 address
- __AAAA:__ return a 128-bit IPv6 address
- __CNAME:__ canonical name record, alias of one name to another
- __CERT:__ certificate record, stores PGP, etc.
- __SRV:__ service locator, used for newer protocols instead of creating a new
  record type
- __TXT:__ used for human readable data, can be leveraged for protocols
- __URL:__ used for redirects

Records are cached by default using a TTL (Time To Live) value. Using `dig(1)`:
```txt
;; ANSWER SECTION:
yoshuawuyts.com.        299     IN      A       104.27.133.163
yoshuawuyts.com.        299     IN      A       104.27.132.163
```
The value here is 299 seconds.

__tools__
- `dnsmasq(1)` - DNS forwarder and server
- `dig(1)` - DNS lookup utility

### DNS Response Policy Zone
- [dns-rpz](http://www.redpill-linpro.com/sysadvent/2015/12/08/dns-rpz.html)
- [wikipedia/response-policy-zone](https://en.wikipedia.org/wiki/Response_policy_zone)
- [DNS: the good parts](https://www.petekeen.net/dns-the-good-parts)

## ICE
Interactive Connectivity Establishment. Used amongst others for WebRTC
handshakes.
- https://en.wikipedia.org/wiki/Interactive_Connectivity_Establishment

### NAT
Network Address Translation
- https://en.wikipedia.org/wiki/Network_address_translation

### STUN
Session Traversal Utilities for NAT. Standardized set of methods and a network
protocol to allow an end host to discover its public IP address if it is
located behind a NAT.
- https://en.wikipedia.org/wiki/STUN

### TURN
Traversal Using Relays around NAT (TURN) is a protocol that assists in
traversal of network address translators (NAT) or firewalls for multimedia
applications. It may be used with the Transmission Control Protocol (TCP) and
User Datagram Protocol (UDP). It is most useful for clients on networks
masqueraded by symmetric NAT devices. TURN does not aid in running servers on
well known ports in the private network through a NAT; it supports the
connection of a user behind a NAT to only a single peer, as in telephony, for
example.
- https://en.wikipedia.org/wiki/Traversal_Using_Relays_around_NAT

## QUIC
- [moving the web from TCP to UDP](https://ma.ttias.be/googles-quic-protocol-moving-web-tcp-udp/)

## Inspect if port is open
`ss(1)` is a tool to validate if a port is open.
```sh
$ ss -tl4 '( sport = :<port_number> )'
$ ss -tl4 '( sport = :22 )'
```

Or using `netstat(1)`
```sh
$ netstat -lnt
```
- http://stackoverflow.com/questions/9609130/quick-way-to-find-if-a-port-is-open-on-linux

## Change ip of router
```sh
$ open http://192.168.1.1/
$ open http://192.168.0.1/
```

## See Also
- [Beej's guide to network programming](http://beej.us/guide/bgnet/output/html/singlepage/bgnet.html)
- [cloudflare interview questions](https://blog.cloudflare.com/cloudflare-interview-questions/)
- [A look inside google's datacenter networks](http://googlecloudplatform.blogspot.nl/2015/06/A-Look-Inside-Googles-Data-Center-Networks.html?m=1)
- [RFC7719: DNS terminology](https://tools.ietf.org/html/rfc7719)
- [iptraf](http://iptraf.seul.org/)
- https://www.digitalocean.com/community/tutorials/how-to-set-up-a-firewall-using-iptables-on-ubuntu-12-04

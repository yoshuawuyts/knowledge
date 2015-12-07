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
```sh
$ nc -z host.example.com 20-30
```

## DNS
[ explain how DNS works ]

__tools__
- `dnsmasq(1)` - DNS forwarder and server
- `dig(1)` - DNS lookup utility

## See Also
- [Beej's guide to network programming](http://beej.us/guide/bgnet/output/html/singlepage/bgnet.html)
- [cloudflare interview questions](https://blog.cloudflare.com/cloudflare-interview-questions/)
- [A look inside google's datacenter networks](http://googlecloudplatform.blogspot.nl/2015/06/A-Look-Inside-Googles-Data-Center-Networks.html?m=1)

# queues

- they're pretty cool to handle small bursts of information
- not good in handling persistant overload
- dropping packages is the only way of doing that correctly, TCP is reliable
so it'll retry and thus won't matter.
- measure where your choking point is at. Knowledge is key.
- use back pressure to do stuff, that lets us drop packages reliably if we need
to

# See also
- [queues don't fix overload](http://ferd.ca/queues-don-t-fix-overload.html)

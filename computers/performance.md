# Performance

## General principles
The general recipe for scalable architecture looks roughly as follows.

> Each core must be supplied with own work and own data and work on it
> independently.

1. __Create enough threads__: K * P threads where P is the number of processors
   (hardware threads), and K is a number of 1..4 (where 1 is ideal, but not
   always). Don't have worker threads, they'll mostly sit idle otherwise.
2. __Have work distribution/balancing mechanisms:__ make sure work is
   distributed evenly between threads, and there is a feedback mechanism to
   (re)distribute it.
3. __Don't extensively use mutexes:__ mutexes are anti-threads; they serve as a
   means of suppressing concurrency.
4. __Eliminate shared mutable state:__ mutation of shared state causes
   cache-coherence traffic, and can become a bottleneck in your system.

- http://www.1024cores.net/home/scalable-architecture/general-recipe
- http://www.1024cores.net/home/scalable-architecture/parallel-disk-io
- http://www.1024cores.net/home/scalable-architecture/task-scheduling-strategies

## Analysis
### Tools
```sh
vmstat         # General system info
mpstat         # CPU info
iostat         # Disk info
netstat        # Network info
lsblk          # Show devices
lsblk -f       # show disk partitions
```

### See Also
- https://www.youtube.com/watch?v=LMx2poY9ORM
- http://www.1024cores.net/home/scalable-architecture/general-recipe
- http://www.1024cores.net/home/parallel-computing/cache-oblivious-algorithms
- http://judy.sourceforge.net/doc/shop_interm.pdf
- https://lmax-exchange.github.io/disruptor/files/Disruptor-1.0.pdf
- http://0x80.pl/

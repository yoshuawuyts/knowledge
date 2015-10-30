# programming
General programming knowledge, useful tidbits and parts to pick up along the
way.

## [Numbers Every Programmer Should Know](http://www.eecs.berkeley.edu/~rcs/research/interactive_latency.html)

Latency Comparison Numbers

Operation                               |           Time           |    Comparison (L1 cache reference = 1 second) |
:---------------------------------------|:------------------------:|:---------------------------------------------:|
*L1* cache reference                    |          0.5 ns          |     0:00:01                                   |
Branch mispredict                       |          5 ns            |     0:00:10                                   |
*L2* cache reference                    |          7 ns            |     0:00:14                                   |
Mutex lock/unlock                       |         25 ns            |     0:00:50                                   |
Main memory reference                   |        100 ns            |     0:03:20                                   |
Compress *1K* bytes with Zippy          |      3,000 ns            |     1:40:00                                   |
Send *1K* bytes over *1 Gbps* network   |     10,000 ns ~= 0.01 ms |     5:33:20                                   |
Read *4K* randomly from SSD*            |    150,000 ns ~= 0.15 ms | 3 days, 11:20:00                              |
Read *1 MB* sequentially from memory    |    250,000 ns ~= 0.25 ms | 5 days, 18:53:20                              |
Round trip within same datacenter       |    500,000 ns ~= 0.5  ms | 11 days, 13:46:40                             |
Read *1 MB* sequentially from SSD*      |  1,000,000 ns ~= 1 ms    | 23 days, 3:33:20                              |
Disk seek                               | 10,000,000 ns ~= 10  ms  | 231 days, 11:33:20                            |
Read *1 MB* sequentially from disk      | 20,000,000 ns ~= 20  ms  | 462 days, 23:06:40                            |
Send packet CA->Netherlands->CA         | 150,000,000 ns ~= 150 ms | 3472 days, 5:20:00                            |


###Notes
----------
> 1 ns = 10-9 seconds
>
> 1 ms = 10-3 seconds
>
> *Assuming ~1GB/sec SSD*

###Credit
------
By Jeff Dean:               http://research.google.com/people/jeff/

Originally by Peter Norvig: http://norvig.com/21-days.html#answers

This gist:                  https://gist.github.com/jboner/2841832



## Meta programming
Programming on a higher level; not bound by languages.

- [project euler](https://projecteuler.net/)

## Debugging
There's only so much humans can do to prevent bugs. Languages like `rust` are
appealing because once you get them running they will not crash. These are some
methods humans can use to find out why computers aren't doing what you ought
them to do.

- triage (printing)
- disinfect (linting)
- monitor (logging)
- traces (stack traces)
- test (test cases)
- scan (???)
- probe (dynamic tracing in dynamic langs)
- observe (direct interaction)
- intervene (built-in debugger)
- operate (inspector tooling)
- biopsy (resource leaks)
- postmortem (gdb / lldb)

- [bug clinic](https://github.com/othiym23/bug-clinic)

## See Also
- [programmer competency matrix](http://sijinjoseph.com/programmer-competency-matrix/)
- [let's build a simple interpreter](http://ruslanspivak.com/lsbasi-part1/)

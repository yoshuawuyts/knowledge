# test
How to test unix applications.

## Test smells
- [test-smells](https://github.com/testdouble/test-smells/tree/master/smells)

## http / https stress testing
[ tbi ]
- [siege(1)](http://linux.die.net/man/1/siege)
- [wrk(1)](https://github.com/wg/wrk)

```sh
$ wrk -c 400 -d 30 -t 12 http://localhost:1337
```

## visual diffs
Envato team does some cool stuff with visual changes:
- run guardian visual diff command
- check %change of visual changes
- if many changes create dynamic block step
- trigger chatops to prompt for unblock

## fuzz testing
> Fuzz testing or fuzzing is a software testing technique, often automated or
> semi-automated, that involves providing invalid, unexpected, or random data
> to the inputs of a computer program. The program is then monitored for
> exceptions such as crashes, or failing built-in code assertions or for
> finding potential memory leaks. Fuzzing is commonly used to test for security
> problems in software or computer systems. It is a form of random testing
> which has been used for testing hardware or software.
- [wikipedia](https://en.wikipedia.org/wiki/Fuzz_testing)

Fuzzing can be done with tools such as `zzuf` and `american fuzzy lop`.
- [the beginners guide to fuzzing](https://fuzzing-project.org/tutorial1.html)

### American Fuzzy Lop
- [american fuzzy lop (afl)](http://lcamtuf.coredump.cx/afl/)
- [how to fuzz a server with afl](https://www.fastly.com/blog/how-fuzz-server-american-fuzzy-lop)
- [bug hunting with afl](https://josephg.com/blog/bug-hunting-with-american-fuzzy-lop/)
- [fuzzing.info](https://fuzzing.info/papers/)
- [	Ask HN: What is the emerging state of the art in fuzzing techniques?](https://news.ycombinator.com/item?id=12078243)

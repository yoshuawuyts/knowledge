# test
How to test unix applications.

## http / https stress testing
[ tbi ]
- [siege(1)](http://linux.die.net/man/1/siege)
- [wrk(1)](https://github.com/wg/wrk)

```sh
$ wrk -c 400 -d 30 -t 12 http://localhost:1337
```

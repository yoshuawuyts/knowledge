# Executors

General resources on executors and threads.

## Cache-line hints

E.g. provide numbers to know the sizes of objects so they can be cached. C++
proposal that's a counterpart to `hardware_concurrency`:

> - __Destructive interference size__: a number that’s suitable as an offset
> between two objects to likely avoid false-sharing due to different runtime
> access patterns from different threads.

> - __Constructive interference size__: a number that’s suitable as a limit on
> two objects’ combined memory footprint size and base alignment to likely
> promote true-sharing between them.

- https://isocpp.org/files/papers/P0154R1.html

## Thread pool in C++

> Further high-level threading facilities such as thread pools have been
> remanded to a future C++ technical report. They are not part of C++11, but
> their eventual implementation is expected to be built entirely on top of the
> thread library features.

- https://en.wikipedia.org/wiki/C%2B%2B11#Threading_facilities

## References
- https://isocpp.org/files/papers/p1348r0.html#biblio-p0443r9
- http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2018/p0443r9.html
- http://www.open-std.org/jtc1/sc22/wg21/docs/papers/2018/p0761r2.pdf

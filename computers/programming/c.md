# c
The c lang

## libuv
To build a program with `libuv`, install it through homebrew and do:
```sh
$ gcc -luv main.c -o main
```

## Hello world
C code:
```c
#include <stdio.h>

int main () {
  printf("Hello World\n");
  return 0;
}
```

Then compile with:
```sh
$ gcc main.c -o main
```

Be careful when combining with other programs though, as the `main` symbol  is
unique and can only exist once per program. It's common to use a namespace to
prevent conflicts. `libuv` uses `uv_` as its prefix.

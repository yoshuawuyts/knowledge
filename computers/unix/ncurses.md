# ncurses

## Handle screen resize
```c
#include <signal.h>

void do_resize(int dummy) {
  .....
}

int main(...) {
  signal(SIDWINCH, do_resize);
  ....
}
```

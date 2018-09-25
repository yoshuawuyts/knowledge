# gdb
GNU Debugger.

## Debugging an assembly program
```sh
$ gdb ./binary
```
```sh
b _start                    # breakpoint at _start
b _start                    # breakpoint at _start
run                         # start the program
s                           # step to next instruction
r <argument>                # pass arguments to the CLI
info registers              # print out all registers
info registers <register>   # print out info for a specific register
show args                   # print all CLI args
quit                        # exit
C-r                         # reverse search
```

## Config
```txt
set history save on                  # Enable history
set history size 100000000           # Set max history size
set history remove-duplicates 100    # Dedupe last 100 history entries
```

## Debugging Rust

## References
- http://smallcultfollowing.com/babysteps/blog/2018/09/21/office-hours-0-debugging-with-gdb/

# gdb
GNU Debugger.

## Debugging an assembly program
```sh
$ gdb ./binary
```
```sh
b _start                    # breakpoint at _start
run                         # start the program
s                           # step to next instruction
r <argument>                # pass arguments to the CLI
info registers              # print out all registers
info registers <register>   # print out info for a specific register
show args                   # print all CLI args
quit                        # exit
```

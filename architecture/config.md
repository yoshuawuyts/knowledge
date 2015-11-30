# config
A program has 2 phases: `initialization` and `io-loop`. During initialization
resources are loaded, connections are made and configuration is passed around.
When the initialization is over, the program starts responding to input with
output in a loop.

## initialization
Initialization should assume to be asynchronous. For more complex setups an
event-based approach might be a solution for certain cases (see `OpenRC` in
Unix).

# tracing

## Internal vs external instrumentation

Instrumentation generally comes in two kinds:

- __Agents (external)__: poll into a process to find out what's going on.
- __Libraries (internal)__: the process itself generates the traces of what's
  going on.

Internal traces can create relationships that external tracers may not be
able to do. For example by only looking at stack traces the system will not
have enough information to reconstruct async contexts. But when using
*internal* tracing, the runtime has all the information needed to provide
this.

__Using external tracing is easier to use to instrument an existing system. But
using internal tracing can provide more information.__

## Black box vs white box systems

Instrumentation comes in two flavors:

- __black box__: treat a system as a single entity you cannot adjust (e.g.
                MySQL DB, legacy app, mainframe)
- __white box__: a system you can add custom instrumentation into and create
                timings for subsystems for

Always start with white-box tracing. This is the part of the system you can
control. Adding black-box tracing can be done by creating a span for it
inside the white-box system. E.g. "here's a call with some parameters to our
mainframe that took 40ms".

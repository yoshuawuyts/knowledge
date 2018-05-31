# wasm notes

## Problem: Endianness of Typed Arrays
- When passing data into WASM, the endianness is architecture-dependent.
- We can't change the endianness of the typed arrays.
- We can perhaps figure out a data view that has a fixed endianness.
- Changing the typed array constructor is hard, they're already exciting - and
  buffer constructor is even more exciting.
- Seems to be concensus that there is a problem, and solutions are possible.
- Aborted to give time to talk about more problems without diving too deep into
  a: single problem.
- A problem is that people don't know that it isn't specified.

## Native Modules
- What are native modules? What do we mean by native modules?
  - I/O vs no I/O is a difference.
  - Seems most people that have questions are about how to I/O.
- Interesting use cases with WASM and native modules.
  - For example bitfields can be massively improved in perf b/c of instruction
    set.
  - Cost of native modules in C++ is too high. In WASM the cost is better.
  - Enables new use cases there.
  - Source maps optimization is along the same lines.
- Privileges
  - Native modules have privileges
  - Wasm doesn't magically have these privileges; things need to happen
  - (talk about how to for example load .dll's from native modules, and expose
    it to WASM)
- Node + WASM (how for example)
  - Would expose a function in a namespace
  - People are seeing cool possibilities of building POSIX like systems with
    this.

## WASM + Threadpool
- Can we schedule WASM on the Libuv threadpool?
- V8 (Benedict) says it's possible that V8 is already setup to allow for this.
- SIMD is also almost done; impl is almost done. Spec is on its way.
- (Some talk about SIMD details & architectures. MIPS & stuff)

## New Features Coming down
- no breaking changes
- no versioning
- can feature test
- format is *super* stable (shall not change!)

### Unwritten rules in wasm
- If you load more than 4k it's tricky
- Lots of stuff about instantiate is impl specific

### import foo.wasm
- Is that asynchronously instantiated?
- It's async initialized.
- People are literally waving hands a little bit, so not entirely?
- Spec doesn't require this to be async. So can be done synchronously.
- ES6 has two passes: instantiation and evaluation.
- If you import from WASM in V8, it'll just feed into the same flow.
- There's a discrepancy between sync / async. Node / Browser.
- This overlaps with modules talk. Going to be delegated to the modules session,
  because there's lots of stuff about the require / import difference with async
  stuff.

## What's the system call story for Node
- Possible solution: We could have a syscall table - call a syscall number, and
  it goes!
- Maybe we could do better?
- (Talk about allowing to call directly into WASM / pass memory into it.)

### How do we deal with function pointer callbacks?
- Give me the wasm instance that called me
- give me the table
- give me the function pointer from the table
- v8 says that this is closed to possible.
- there are already "naked" wasm VMs that do this
- mafintosh: this is the biggest improvement we could do for native modules
  already

## How do we share memory between multiple instances?
[redacted]

## Instantiate Streaming
- When you're instantiating a module, if it's streaming you can decode & compile
  WASM modules as they're coming in.
- There are some missing bits in Node to enable this.
- V8 has this already.
- Didn't catch a clear resolution for Node in this.
- Emil: why is this a good idea if we just read it from disk?
- (room seems to be like, "ehhhh, good question")
- Emil: is the goal to deprecate the sync instantiation?
- they're not recommended.
- mafintosh: it's the only way to do it in Node.
- (talk about base64 being fun to do - okay in practice for small modules, which
  is good enough)

## Gyp & toolchain
- WASM has a few interesting implications.
- WASM ABI isn't stable (!).
- Eventually we might want this so we can link multiple WASM things together.
- Rust gets around this by having source packages.
- C++ would want the same thing.
- Ryan: gn has some of the same abstractions gyp has, it shouldn't be too
  difficult to enable it. gn is great.
- Myles: well, not necessarily. Believe there's not the same matrix of support
  for runtimes that gyp has right now. Not sure what work is needed to support
  them.
- Benedict: v8 might help here. Basil? (followed by laughter in the room, and
  wasm jokes).
- mafintosh: can't we support both?
- Myles: it costs time & resources.
- Myles: This topic is important because if we want people to get this to work,
  we need a good story here.
- (bunch of talk about how C++ & WASM are done _today_)
- Node's NAPI is great. Libuv is not ABI stable - that's probably a thing to
  look at stabilizing. Those guarantees would make things a lot better.

## What's the process after this workshop?
- What do we think are the next steps after this?
- How to instantiate streaming in Node would be one next topic to discuss?
- Myles: this might be a reason to bring Fetch into Node. As an abstraction over
  h2 / h1.
- Myles: tangent - APM for WASM?

### WASM modules
- emil: would be nice to have a version with modules, and without modules
- mafintosh: (agreeing) - would be nice to have a version that doesn't have
  modules. Capabilities thing.
- emil: what is host bindings?
- It's more like web / js host bindings. Stuff like calling "new" from WASM. Or
  allocating string literals without needing to pass it in from JS.
- (people realizing host bindings might have different meanings)
- (might be referred to as ring0 modules; other things)
- (that's not host bindings)
- There is no clear name about what to call that.

### How to stay up to date?
- Twice a month video chat that you can check in.
- Notes are taken, and sent out.
- Subgroups are coming up, so that might be a thing.
- Would be nice if there was a WASM digest.

### Who is represented in WASM meetings
- Are Node people represented? (not many it seems)
- Blockchain people
- Chip vendors
- CDNs
- Googlers
- Myles: Maybe what we could do is getting some more people represent the Node
  usecases.
- Myles: there's a need for having a clear difference for what is available in
  the VM / what is in the spec.
- Lin: there's definitely space in the chats. The working groups is a different
  story.
- Webassembly.org/contribute has some things - been meaning to update tho.

### next steps on WASM / Node modules
- mafintosh: libuv probably. (posix?) It's out posix, basically.
- hard part is creating the first API. The rest should be fairly straight
  forward after that.
- One thing to consider is to not talk about this too much in a node specific
  context, but think about how to make this more generic.
- mafintosh: can we do something about this today?
- benedict: we see node as one of the first customers of using these bindings.
- (people realizing host bindings might have different meanings)
- (people talking about how to follow up *today* - meet over lunch, talk more)

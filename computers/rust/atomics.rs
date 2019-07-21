# atomics
## Create a global counter
```rust
use std::sync::atomic::{AtomicUsize, Ordering};

// Setup the atomic as a static.
static COUNTER: AtomicUsize = AtomicUsize::new(0);

// Can now call this from anywhere.
let id = COUNTER.fetch_add(1, Ordering::SeqCst);
```

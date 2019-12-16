`async-ctrlc` is an async wrapper of the [`ctrlc`] crate.

```rust
use async_ctrlc::CtrlC;
use async_std::{prelude::FutureExt, task::sleep};
use std::time::Duration;

#[async_std::main]
async fn main() {
    let ctrlc = CtrlC::new().expect("cannot create Ctrl+C handler?");
    println!("Try to press Ctrl+C");
    ctrlc.race(async {
        let mut i = 0;
        loop {
            println!("... {}", i);
            sleep(Duration::from_secs(1)).await;
            i += 1;
        }
    }).await;
    println!("Quitting");
}
```

[`ctrlc`]: https://github.com/Detegr/rust-ctrlc

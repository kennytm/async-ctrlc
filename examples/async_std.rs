// Copyright 2019 kennytm. Licensed under MIT OR Apache-2.0.

use async_ctrlc::CtrlC;
use async_std::{prelude::FutureExt, task::sleep};
use std::time::Duration;

#[async_std::main]
async fn main() {
    let ctrlc = CtrlC::new().expect("cannot create Ctrl+C handler?");
    println!("Try to press Ctrl+C");
    ctrlc
        .race(async {
            let mut i = 0;
            loop {
                println!("... {}", i);
                sleep(Duration::from_secs(1)).await;
                i += 1;
            }
        })
        .await;
    println!("Quitting");
}

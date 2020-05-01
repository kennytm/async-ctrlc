// Copyright 2020 kennytm. Licensed under MIT OR Apache-2.0.

use async_ctrlc::CtrlC;
use async_std::prelude::StreamExt as _;

#[async_std::main]
async fn main() {
    let ctrlc = CtrlC::new().expect("cannot create Ctrl+C handler?");
    println!("Try to press Ctrl+C 3 times");
    let mut stream = ctrlc.enumerate().take(3);
    while let Some((count, _)) = stream.next().await {
        println!("{} x Ctrl+C!", count + 1);
    }
    println!("Quitting");
}

extern crate rjpb;

use std::env;
use rjpb::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let executor = Executor::new(&args);
    executor.run();
}

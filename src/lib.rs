#![feature(fn_must_use)]

extern crate json;
extern crate pikkr;
extern crate serde;
#[cfg_attr(test, macro_use)]
extern crate serde_json;

mod executor;
mod targets;

pub use executor::Executor;

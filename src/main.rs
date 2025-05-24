extern crate core;

mod util;
mod models;
mod storage;
mod config;
mod context;
mod query;
mod key;

fn main() {
    let mut opt = Some(String::from("hello"));
    let s = &mut opt.unwrap();


    println!("Hello, world!");
}

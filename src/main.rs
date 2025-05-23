extern crate core;

mod util;
mod models;
mod storage;
mod config;
mod context;

fn main() {
    let mut opt = Some(String::from("hello"));
    let s = &mut opt.unwrap();


    println!("Hello, world!");
}

extern crate core;
mod util;
mod models;
mod storage;
mod config;

fn main() {
    let mut opt = Some(String::from("hello"));
    let s = &mut opt.unwrap();


    println!("Hello, world!");
}

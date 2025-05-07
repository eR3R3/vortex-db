extern crate core;

mod rocks_db;
mod util;
mod models;


fn main() {
    let mut opt = Some(String::from("hello"));
    let s = &mut opt.unwrap();


    println!("Hello, world!");
}

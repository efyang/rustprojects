#![feature(iter_arith)]
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("names.txt").unwrap(); 
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();
    println!("{:?}", data.rsplit('"').collect::<Vec<&str>>());
}

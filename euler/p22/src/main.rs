#![feature(iter_arith)]
use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = File::open("names.txt").unwrap(); 
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();
    let names = data.split('"')
        .collect::<String>();
    let mut namelist = names.split(",")
        .collect::<Vec<&str>>();
    namelist.sort();
    println!("{:?}",namelist);
}

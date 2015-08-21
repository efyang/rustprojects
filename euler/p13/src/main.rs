use std::io::prelude::*;
use std::fs::File;

fn main() {
    let mut f = try!(File::open("numbers.txt")); 
    let mut data: Vec<u8> = Vec::new();
    let mut data2 = String::new();
    try!(f.read_to_string(&mut data2));
    println!("{}",data2);
    //for chunk in data.iter().chunks(52) {
        //println!("{:?}",chunk);
    //}
}

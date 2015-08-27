#![feature(iter_arith)]
#![feature(convert)]
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn get_alphabetical_value(name: &String, alphabet: &HashMap<&str, u64>) -> u64 {
    name.chars().filter_map(|x| alphabet.get(x.to_string().as_str())).sum() 
}

fn main() {
    let mut alphabet: HashMap<&str, u64> = HashMap::new(); 
    alphabet.insert("A", 1);
    alphabet.insert("B", 2);
    alphabet.insert("C", 3);
    alphabet.insert("D", 4);
    alphabet.insert("E", 5);
    alphabet.insert("F", 6);
    alphabet.insert("G", 7);
    alphabet.insert("H", 8);
    alphabet.insert("I", 9);
    alphabet.insert("J", 10);
    alphabet.insert("K", 11);
    alphabet.insert("L", 12);
    alphabet.insert("M", 13);
    alphabet.insert("N", 14);
    alphabet.insert("O", 15);
    alphabet.insert("P", 16);
    alphabet.insert("Q", 17);
    alphabet.insert("R", 18);
    alphabet.insert("S", 19);
    alphabet.insert("T", 20);
    alphabet.insert("U", 21);
    alphabet.insert("V", 22);
    alphabet.insert("W", 23);
    alphabet.insert("X", 24);
    alphabet.insert("Y", 25);
    alphabet.insert("Z", 26);


    let mut f = File::open("names.txt").unwrap(); 
    let mut data = String::new();
    f.read_to_string(&mut data).unwrap();
    let mut names = data.split('"')
        .collect::<String>()
        .split(",")
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    names.sort();
    let mut total: u64 = 0;
    for index in 0..names.len() {
       total += (index + 1) as u64 * get_alphabetical_value(&names[index], &alphabet)
    }
    println!("Total: {}", total);
}

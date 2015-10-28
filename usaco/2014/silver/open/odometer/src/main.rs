use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn parse(file: &str) -> Vec<isize> {
    let mut f = File::open(file).expect("Failed to open file");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Failed to read to string");
    s.trim()
        .split_whitespace()
        .map(|x| x.parse::<isize>().expect("Failed to parse to int"))
        .collect::<Vec<isize>>()
}

fn write_data(data: usize) {
    let mut f = File::create("odometer.out").expect("Failed to open file for writing");
    println!("{}", data);
    f.write_all(data.to_string().as_bytes()).expect("Failed to write to file");
}

fn check_interesting(i: isize) -> bool {
    let mut charcounts = HashMap::new();
    let istring = i.to_string();
    let slength = istring.len();
    let halflength = (slength as f64/2.0).ceil() as isize;
    
    for c in istring.chars() {
        if charcounts.contains_key(&c) {
            if let Some(item) = charcounts.get_mut(&c) {
                if *item >= halflength {
                    return true;
                } else {
                    *item += 1;
                }
            }
        } else {
            charcounts.insert(c, 1);
        }
    }

    if charcounts.values().max().unwrap() >= &halflength {
        true
    } else {
        false
    }
}

fn get_interesting(start: isize, end: isize) -> usize {
    let mut results = Vec::with_capacity((end - start) as usize);
    println!("{} {}", start, end); 
    for i in start..(end + 1) {
        if check_interesting(i) {
            results.push(i);
        }
    }
    results.len()
}

fn main() {
    let parsed = parse("odometer.in");
    let results = get_interesting(parsed[0], parsed[1]);
    write_data(results);
}

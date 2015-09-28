#![feature(result_expect)]
#![feature(convert)]
use std::fs::File;
use std::io::prelude::*;

fn main() {
    //(size, position)
    let data: Vec<(usize, usize)> = read_data();
    println!("{:?}", data);
    //println!("{:?}", iterate(&data));
}

fn read_data() -> Vec<(usize, usize)> {
    fn parseline(line: &str) -> (usize, usize) {
        let linewords: Vec<&str> = line.split_whitespace().collect::<Vec<&str>>();
        let size: usize = linewords[0].parse::<usize>().unwrap();
        let position: usize = linewords[1].parse::<usize>().unwrap();
        (size, position)
    }

    let mut strinput: String = String::new();
    let mut input = File::open("trapped.in").expect("Failed to open file trapped.in");
    input.read_to_string(&mut strinput).expect("Failed to parse data.");
    let data = strinput
        .lines()
        .skip(1)
        .map(|line| parseline(line))
        .collect::<Vec<(usize, usize)>>();
    data
}


fn write_data(data: &usize) {
    let mut output = File::create("trapped.out").expect("Failed to create file trapped.out()");
    output.write_all(data.to_string().into_bytes().as_slice());
}

fn iterate(data: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    unimplemented!();
}



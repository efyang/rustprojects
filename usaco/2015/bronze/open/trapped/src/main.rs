#![feature(result_expect)]
#![feature(convert)]
#![feature(slice_splits)]
use std::fs::File;
use std::io::prelude::*;

fn main() {
    //(size, position)
    let data: Vec<(usize, usize)> = read_data();
    println!("{:?}", iterate(&data));
    println!("{:?}", data);
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
    let mut output = File::create("trapped.out").expect("Failed to create file trapped.out");
    output.write_all(data.to_string().into_bytes().as_slice()).expect("Failed to write data.");
}

fn iterate(data: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    fn get_distances(data: &Vec<(usize, usize)>) -> Vec<usize> {
        //gets distances between bales
        let init = data.split_last().unwrap().1;
        let tail = data.split_first().unwrap().1;
        let distances: Vec<usize> = init.iter().zip(tail.iter())
            .map(|datapair| (datapair.1).1 - (datapair.0).1)
            .collect::<Vec<usize>>();
        distances
    }

    fn remove_bale_posns(data: &Vec<(usize, usize)>, posns: &Vec<usize>) -> Vec<(usize, usize)> {
        //removes the bales that match with the posns
        data.to_owned()
            .into_iter()
            .filter(|x| !posns.contains(&x.1))
            .collect::<Vec<(usize, usize)>>()
    } 

    let distances = get_distances(data);
    let mut dataclone = data.to_owned();
    dataclone.reserve(0);

    for bale in dataclone.iter() {
        println!("{:?}", bale);
    }
    //for bale in data.iter().rev() {
        
    //}
    dataclone
}



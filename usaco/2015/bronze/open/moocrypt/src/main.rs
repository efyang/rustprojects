#![feature(convert)]
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    let mut infile = File::open("moocrypt.in").unwrap();
    let mut input: String = String::new(); 
    infile.read_to_string(&mut input).unwrap();
    let mut data: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        data.push(line.chars().collect::<Vec<char>>());
    }
    data = data.into_iter().skip(1).collect::<Vec<Vec<char>>>();
    let mut collected: HashMap<String, usize> = HashMap::new();
    let mut results: Vec<&usize> = Vec::new();
    get_horizontals(data.clone(), &mut collected);
    get_verticals(data.clone(), &mut collected);
    get_diagonals(data.clone(), &mut collected);
    for x in collected.iter() {
        results.push(x.1);
    }
    let mut outfile = File::create("moocrypt.out").unwrap();
    outfile.write_all(format!("{}", results.into_iter().max().unwrap())
                      .into_bytes()
                      .as_slice()).unwrap();
}

fn is_moo(strchunk: &Vec<char>) -> bool {
    strchunk[0] != 'M' && strchunk[1] != 'O' && strchunk[1] == strchunk[2] && !strchunk.contains(&'-')
}

fn get_horizontals(graph: Vec<Vec<char>>, collected: &mut HashMap<String, usize>) {
    for line in graph {
        for n in line.windows(3){
            let strchunk: Vec<char> = n.to_vec();
            let revstring: Vec<char> = n.iter().rev().map(|x| x.clone()).collect::<Vec<char>>();
            let sstrchunk: String = strchunk.iter().map(|x| x.clone()).collect::<String>();
            let srevstring: String = revstring.iter().map(|x| x.clone()).collect::<String>();
            if is_moo(&strchunk) || is_moo(&revstring) {
                let hashkeys: Vec<String> = collected.keys().map(|x| x.clone()).collect::<Vec<String>>();
                if hashkeys.contains(&sstrchunk) {
                    let entryval = collected.entry(sstrchunk).or_insert(1);
                    *entryval += 1;
                }
                else if hashkeys.contains(&srevstring) {
                    let entryval = collected.entry(srevstring).or_insert(1);
                    *entryval += 1;
                }
                else {
                    collected.insert(sstrchunk.clone(), 1);
                }
            } 
        }
    }
}

fn get_verticals(graph: Vec<Vec<char>>, collected: &mut HashMap<String, usize>) {
    let mut newgraph: Vec<Vec<char>> = Vec::new();
    let line_len = graph.first().unwrap().len();
    for _ in 0..line_len {
        newgraph.push(Vec::new()); 
    }
    for line in graph {
        for x in 0..line_len {
            newgraph[x].push(line[x].clone());
        }
    }
    get_horizontals(newgraph.clone(), collected);
}

fn get_diagonals(graph: Vec<Vec<char>>, collected: &mut HashMap<String, usize>) {
    let maxshift = graph.len() - 1;
    let mut leftshift: Vec<Vec<char>> = graph.clone();
    let mut rightshift: Vec<Vec<char>> = graph.clone();
    for zeroes in (1..maxshift + 1).rev() {
        for _ in 0..zeroes {
            leftshift[maxshift - zeroes].insert(0,'-');
            rightshift[maxshift - zeroes].push('-');
        }
        for _ in 0..zeroes {
            leftshift[zeroes].push('-'); 
            rightshift[zeroes].insert(0,'-');
        }
    }
    get_verticals(leftshift, collected);
    get_verticals(rightshift, collected);
}

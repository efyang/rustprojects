#![feature(convert)]
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

fn main() {
    //let input = "08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08 \
    //49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00 \
    //81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65 \
    //52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91 \
    //22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80 \
    //24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50 \
    //32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70 \
    //67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21 \
    //24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72 \
    //21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95 \
    //78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92 \
    //16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57 \
    //86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58 \
    //19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40 \
    //04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66 \
    //88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69 \
    //04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36 \
    //20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16 \
    //20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54 \
    //01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48 ";
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

    //let mut itemlist: Vec<char> = Vec::new();
    //let mut numlist: Vec<u64> = Vec::new();
    //let mut graph: Vec<Vec<u64>> = Vec::new();
    //let mut products: Vec<u64> = Vec::new();
    //for num in input.to_string().chars() {
        //itemlist.push(num);
    //}
    //for chunk in itemlist.chunks(3) {
        //let mut new_string: Vec<char> = Vec::new();
        //for c in chunk {
            //new_string.push(c.clone());
        //}
        //new_string.pop().unwrap();
        //let the_string: String = new_string.into_iter().collect();
        //numlist.push(the_string.parse::<u64>().unwrap());
    //}
    //for chunk in numlist.chunks(20) {
        //graph.push(chunk.to_vec());
    //}
    
    //products.push(get_horizontal_max(graph.clone()));
    //products.push(get_vertical_max(graph.clone()));
    //products.push(get_diagonal_max(graph.clone()));
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
    //let mut products: Vec<u64> = Vec::new();
    for line in graph {
        for n in line.windows(3){
            //products.push(n.to_vec()); 
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
    }//fix to work with rectangles
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

#![feature(split_off)]
#![feature(append)]

fn main() {
    println!("{:?}", pandita_to_limit(&1000000u64, &vec![0,1,2,3,4,5,6,7,8,9u64]));
}

fn pandita_to_limit(limit: &u64, items: &Vec<u64>) -> Vec<u64> {
    let mut sequence: Vec<u64> = items.clone();
    let mut permutations: u64 = 1;
    let seq_length: usize = sequence.len();
    let mut k: usize;
    let mut i: usize = 0;
    let mut index: isize;
    if seq_length <= 1 {
        return sequence;
    }
    sequence.sort();

    while &permutations < limit {
        k = seq_length;
        index = seq_length as isize - 2;
        while index >= 0 {
            if sequence[index as usize] < sequence[index as usize + 1] {
                k = index as usize;
                break;
            }
            index -= 1;
        }
        if k == seq_length {
            return sequence;
        }
        index = seq_length as isize - 1;
        while index >= 0 {
            if sequence[index as usize] > sequence[k] {
                i = index as usize;
                break;
            } 
            index -= 1;
        }
        sequence.swap(k, i);
        let mut suffix: Vec<u64> = sequence.split_off(k + 1)
            .iter()
            .map(|x| *x)
            .rev()
            .collect::<Vec<u64>>();
        sequence.append(&mut suffix);
        //println!("{:?}", permutations);
        permutations += 1;
    }
    sequence
}



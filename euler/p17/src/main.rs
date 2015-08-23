const NUMLENGTH: [u32; 20] = [4,3,3,5,4,4,3,5,5,4,3,6,6,8,8,7,7,9,8,8];
//0-19
const TENNUMLENGTH: [u32; 8] = [6,6,5,5,5,7,6,6];
//20-90 by tens
const OVERNUMLENGTH: [u32; 2] = [7,8];
//100,1000 (100 always have an and in them)

fn number_to_length(n: u32) -> u32 {
    let mut vec_rep: Vec<u32> = n.to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .rev()
        .collect();
    let mut length: u32 = 0;
    let mut was_thousand: bool = true;
    if vec_rep.len() == 4 {
        if vec_rep.last().unwrap().clone() == 0 {
            vec_rep.pop().unwrap();
            was_thousand = false;
        } 
        else {
            length += NUMLENGTH[vec_rep.pop().unwrap() as usize];
            length += OVERNUMLENGTH[1];
        }
    } 
    if vec_rep.len() == 3 {
        if vec_rep.last().unwrap().clone() == 0 {
            vec_rep.pop().unwrap();
        }
        else {
            if was_thousand && (vec_rep.first().unwrap().clone() != 0 || vec_rep.get(1).unwrap().clone() != 0) {
                length += 3;
            }
            length += NUMLENGTH[vec_rep.pop().unwrap() as usize];
            length += OVERNUMLENGTH[0];     
        } 
    }
    if vec_rep.len() == 2 {
        if vec_rep.last().unwrap().clone() == 1 {
            let newvec: Vec<String> = vec_rep.clone()
                .iter()
                .map(|x| x.to_string())
                .rev()
                .collect();
            length += NUMLENGTH[newvec.concat().parse::<usize>().unwrap()];
            vec_rep.pop().unwrap();
            vec_rep.pop().unwrap();
        } 
        else if vec_rep.last().unwrap().clone() == 0 {
            vec_rep.pop().unwrap();
        } 
        else {
            length += TENNUMLENGTH[(vec_rep.pop().unwrap() as usize) - 2];
        }
    }
    if vec_rep.len() == 1 {
        if vec_rep.last().unwrap().clone() != 0 {
            length += NUMLENGTH[vec_rep.pop().unwrap() as usize];
        }
    }
    return length; 
}

fn main() {
    let mut letters: u32 = 0;
    for x in 1..1001 {
        letters += number_to_length(x);
    }
    println!("{}", letters);
}

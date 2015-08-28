#![feature(iter_arith)]
//max is to 7 
const POWARRAY: [u64; 10] = [0, 1, 32, 243, 1024, 3125, 7776, 16807, 32768, 59049];

fn is_power_sum(n: &u64) -> bool {
    n == &n.to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| POWARRAY[x as usize])
        .sum::<u64>()
}

fn main() {
    let mut sum: u64 = 0;
    for x in (10..1000000u64) {
        if is_power_sum(&x) {
            sum += x;
        }
    }
    println!("{}", sum);
}

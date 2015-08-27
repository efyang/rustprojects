#![feature(iter_arith)]
fn divisor_sum(n: &u64) -> u64 {
    (1..(n/2 + 1)).filter(|&x| n % x == 0).sum::<u64>()
}

fn is_abundant_number(n: &u64) -> bool {
    &divisor_sum(n) > n
}

fn get_abundants(limit: &u64) -> Vec<u64> {
    if limit > &28123u64 {
        (1..28124u64).filter(|x| is_abundant_number(&x)).collect::<Vec<u64>>()
    } 
    else {
        (1..*limit).filter(|x| is_abundant_number(&x)).collect::<Vec<u64>>()
    }
}

fn get_all_abundants() -> Vec<u64> {
    get_abundants(&28112u64)
}

fn main() {
    let abundants: Vec<u64> = get_all_abundants();
    println!("{:?}", abundants);
}

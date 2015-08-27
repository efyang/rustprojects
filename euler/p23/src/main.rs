#![feature(iter_arith)]
fn divisor_sum(n: &u64) -> u64 {
    (1..(n/2 + 1)).filter(|&x| n % x == 0).sum::<u64>()
}

fn is_abundant_number(n: &u64) -> bool {
    &divisor_sum(n) > n
}

fn get_abundants(limit: &u64) -> Vec<u64> {
    (1..*limit).filter(|x| is_abundant_number(&x)).collect::<Vec<u64>>()
}

fn get_all_abundants() -> Vec<u64> {
    get_abundants(&28124u64)
}

fn get_abundant_sums() -> Vec<u64> {
    let abundants: Vec<u64> = get_all_abundants();
    let mut abundant_sums: Vec<u64> = Vec::new();
    for x in abundants.iter() {
        for y in abundants.iter() {
            if x + y < 28124 {
                abundant_sums.push(x + y);
            }
        }
    }
    abundant_sums 
}

fn get_non_abundant_sum() -> u64 {
    let abundant_sums = get_abundant_sums();
    (1..28124u64).filter(|x| !abundant_sums.contains(&x)).sum::<u64>() 
}

fn main() {
    println!("Sum of all non-abundants: {:?}", get_non_abundant_sum());
}

#![feature(step_by)]
#![feature(iter_arith)]
fn sieve(limit: u64) -> Vec<u64> {
    let mut list: Vec<u64> = (3..limit).step_by(2).collect();
    list.reverse();
    let mut primes: Vec<u64> = vec![2,3];
    let mut prime: u64 = 3;

    while !list.is_empty() {
        list = list.into_iter().filter(|&item| item % prime != 0).collect();
        prime = list.pop().unwrap();
        primes.push(prime.clone());
    } 
    return primes;
}

fn main() {
    println!("Sum: {}", sieve(2000000 as u64).iter().sum::<u64>());
}

#![feature(step_by)]
#![feature(iter_arith)]
#![feature(append)]
fn is_prime(n: u64) -> bool {
    if n % 2 == 0 { return false;}
    for x in (3..((n as f64).sqrt() as u64) + 1).step_by(2) {
        if n % x == 0 { return false;}
    }
    return true;
}

fn main() {
    let list: Vec<u64> = (3..2000001u64).step_by(2).collect();
    let mut primes: Vec<u64> = vec![2];
    let mut newlist: Vec<u64> = list.into_iter().filter(|&n| is_prime(n)).collect();
    primes.append(&mut newlist);

    println!("Sum is: {}", primes.iter().sum::<u64>());
}

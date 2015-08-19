#![feature(step_by)]
#![feature(append)]
mod prime;
use prime::{factorize,is_prime};

fn main() {
    let mut factors: Vec<u64> = prime::factorize(600851475143 as u64);
    factors.sort();
    println!("Largest prime factor of 600851475143 is: {}", factors.last().unwrap());
}

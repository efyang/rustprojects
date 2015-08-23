#![feature(iter_arith)]
extern crate num;
use num::bigint::{BigUint, ToBigUint};
fn factorial(n: BigUint) -> BigUint {
    let one = 1.to_biguint().unwrap();
    if n != one {
        return n.clone() * factorial(n - one);
    }
    else {
        return n;
    }
}
fn main() {
    let facsum: u64 = factorial(100u64.to_biguint().unwrap())
        .to_string()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .map(|x| x as u64)
        .sum();
    println!("Sum of 100!: {}", facsum);
}

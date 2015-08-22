#![feature(iter_arith)]
extern crate num;
use num::bigint::{BigUint,ToBigUint};

fn main() {
    let bigtwo: BigUint = 2u64.to_biguint().unwrap();
    println!("{}", num::pow(bigtwo, 1000usize)
            .to_string()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .sum::<u32>());
}

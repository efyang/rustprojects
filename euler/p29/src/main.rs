extern crate num;
use num::bigint::{BigUint, ToBigUint};
use num::pow;

fn main() {
    let mut terms: Vec<BigUint> = Vec::new();
    for a in (2..101) {
        for b in (2..101) {
            terms.push(num::pow(a.to_biguint().unwrap(), b as usize));
        }
    }
    terms.sort();
    terms.dedup();
    println!("{}", terms.len());
}

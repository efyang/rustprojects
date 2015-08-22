extern crate num;
use num::bigint::{BigUint, ToBigUint};
fn factorial(n: &BigUint) -> BigUint {
    let one = 1.to_biguint().unwrap();
    if n != one {
        return n.clone() * factorial(n - one);
    }
    else {
        return n;
    }
}

fn combination(n: BigUint, r: BigUint) -> BigUint {
    return factorial(&n) / (factorial(&r) * factorial(n - r));
}

fn main() {
    println!("{}", combination(40u64.to_biguint().unwrap(),20u64.to_biguint().unwrap()));
}

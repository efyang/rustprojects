extern crate num;
use num::bigint::{BigUint, ToBigUint};

fn fib_list_to_max_length(max: u64) -> u64 {
    let mut fib_list: Vec<BigUint> = vec![1.to_biguint().unwrap(), 1.to_biguint().unwrap()];
    let mut index;
    let mut next_fib: BigUint;
    loop{
        index = fib_list.len() - 1;
        next_fib = &fib_list[index] + &fib_list[index - 1];
        fib_list.push(next_fib.clone());
        if next_fib.to_string().len() == max as usize {
            break
        }
    }
    fib_list.len() as u64 
}
fn main() {
    println!("{}", fib_list_to_max_length(1000u64));
}

#[cfg(test)]
mod test {

    #[test]
    fn digit_test() {
        assert_eq!(12, super::fib_list_to_max_length(3));
    }
}

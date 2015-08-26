#![feature(iter_arith)]
fn divisor_sum(n: &u64) -> u64 {
    (1..(n/2 + 1)).filter(|&x| n % x == 0).sum::<u64>()
}

fn is_amicable(n: &u64) -> bool {
    let nsum: u64 = divisor_sum(n);
    &divisor_sum(&nsum) == n && &nsum != n
}

fn amicable_number_sum(limit: u64) -> u64 {
    (1..limit).filter(|x| is_amicable(x)).sum::<u64>()
}

fn main() {
    println!("Sum of all amicable numbers under 10000: {}", amicable_number_sum(10000));
}

#[cfg(test)]
mod test {
    #[test]
    fn divisor_sum() {
        assert_eq!(284u64, super::divisor_sum(&220));
        assert_eq!(220u64, super::divisor_sum(&284));
    }
    
    #[test]
    fn is_amicable() {
        assert!(super::is_amicable(&220u64));
    }
}

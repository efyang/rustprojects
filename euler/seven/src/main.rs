fn prime_to_all( n: u64, values: Vec<u64>) -> bool {
    for x in values {
        if n % x == 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut primes: Vec<u64> = vec![2,3,5,7,11,13];
    let mut prime_count: u64 = 6;
    let mut test: u64 = 13;

    while prime_count < 10001 {
        test += 2;
        if prime_to_all(test, primes.clone()) {
            primes.push(test);
            prime_count += 1;
        }
    }
    println!("{}",test);
}

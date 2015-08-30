#![feature(step_by)]

fn main() {
    let mut maxes: (i64, i64) = (0, 0);
    let mut max: usize = 0;
    for a in -999..1000i64 {
        for b in -999..1000i64 {
            let prime_length: usize = quadratic_consecutive_prime_length(a, b);
            if prime_length > max {
                maxes = (a, b);
                max = prime_length;
            }
        }
    }
    println!("{}", maxes.0 * maxes.1);
}

fn is_prime(n: i64) -> bool {
    if n % 2 == 0 {
        return false;
    } 
    for x in (3..((n as f64).sqrt() as i64 + 1)).step_by(2) {
        if n % x == 0 {
            return false;
        }
    }
    return true;
}

fn quadratic(n: i64, a: i64, b: i64) -> i64 {
    (n * n) + (a * n) + b
}

fn quadratic_consecutive_prime_length(a: i64, b: i64) -> usize {
    let mut n: usize = 0;
    while is_prime(quadratic(n as i64, a, b).abs()) {
        n += 1;
    }
    n
}

#[cfg(test)] 
mod tests {

    #[test]
    fn quadratic_consecutive_prime_length() {
        assert_eq!(80usize, super::quadratic_consecutive_prime_length(-79i64, 1601i64));
        assert_eq!(40usize, super::quadratic_consecutive_prime_length(1i64, 41i64)); 
    }
}

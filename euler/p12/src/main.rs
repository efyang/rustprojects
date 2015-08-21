#![feature(iter_arith)]

fn factorize(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = vec![1];
    for i in 2..(((n + 1) as f64).sqrt() as u64) {
        if n % i == 0 {
            factors.push(i);
            factors.push(n/i);
        }
    }
    factors.push(n);
    return factors;
}

fn triangulate(n: u64) -> u64 {
    return (1..(n + 1)).sum();   
}

fn main() {
    let mut n: u64 = 28;
    loop {
        let triangle = triangulate(n);
        if factorize(triangle).len() > 500 {
            println!("First triangle number to have over 500 divisors: {}", triangle);
            break;
        }
        n += 1;
    }
}

pub fn is_prime(n: u64) -> bool{
    if n % 2 == 0 {
    return false;
    }
    for x in (3..(n as f64).sqrt() as u64 + 1).step_by(2) {
        if n % x == 0{
            return false;
        }
    }
    return true;
}

pub fn factorize(n: u64) -> Vec<u64> {
    let mut factors: Vec<u64> = Vec::new();
    if even(n) {
        factors.push(2);
        factors.append(&mut factorize(n/2));
    }
    else{
        for x in (3..(n as f64).sqrt() as u64 + 1).step_by(2) {
            if n % x == 0{
                factors.append(&mut factorize(x));
                factors.append(&mut factorize(n/x));
            }
        }       
    }
    if factors.is_empty() {
        return vec![n];
    }
    else{
        return factors;
    }
}

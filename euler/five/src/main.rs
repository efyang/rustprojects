fn isdivisible(n: i64) -> bool {
    for x in 2..21 as i64 {
        if n % x != 0 {
            return false;
        }
    }
    return true;
}

fn main() {
    let mut current: i64 = 2520;
    loop {
        if isdivisible(current) {
            println!("{}", current);
            break;
        }
        current += 20;
    }
}

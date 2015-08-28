fn get_spiral_sum(length_limit: u64) -> u64 {
    let mut length: u64 = 1;
    let mut sum: u64 = 1;
    let mut current: u64 = 1;
    while length < length_limit {
        length += 2;
        for _ in 0..4 {
            current += length - 1;
            sum += current;
        }
    }
    sum
}

fn main() {
    println!("Sum of 1001x1001 spiral: {}", get_spiral_sum(1001u64));
}

#[cfg(test)]
mod tests {
    #[test]
    fn five_spiral_test() {
        assert_eq!(super::get_spiral_sum(5u64), 101u64);
    }
}

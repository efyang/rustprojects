fn is_leap_year(year: u64) -> bool {
    let yearchars: Vec<u32> = year.to_string()
        .chars()
        .rev()
        .filter_map(|x| x.to_digit(10))
        .collect();
    if yearchars[0] == 0 && yearchars[1] == 0 {
        //century
        year % 400 == 0
    }
    else {
        year % 4 == 0
    }
}

fn main() {
    println!("Hello, world!");
    let mut day = 1;
    for year in 1901..2001u64 {
        
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn not_leap_year() {
        assert!(!super::is_leap_year(1900u64));
    }

    #[test]
    fn leap_year() {
        assert!(super::is_leap_year(1904u64));
    }
}

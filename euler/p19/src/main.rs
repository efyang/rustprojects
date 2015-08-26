fn is_leap_year(year: &u64) -> bool {
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
    let mut day = 366u64 % 7;
    let mut sundays = 0;
    for year in 1901..2001u64 {
        let month_lengths: [u64; 12];
        if is_leap_year(&year) {
            month_lengths = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        }
        else {
            month_lengths = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        }

        for month in month_lengths.iter() {
            day += *month;
            if day % 7 == 0 {
                sundays += 1;
            }
        }
        day = day % 7;
    }
    println!("{}", sundays);
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

fn main() {
    let mut string: String;
    let mut rev_string: String;
    let mut palindromes: Vec<i64> = Vec::new();
    let mut value: i64;
    for x in (100..1000).rev() {
        for y in (100..1000).rev() {
            value = x*y;
            string = value.to_string();
            rev_string = string
                .chars()
                .rev()
                .collect();
            //println!("{} {}", string, rev_string);
            if string == rev_string {
                palindromes.push(value);
                break;
            }
        }
    }
    palindromes.sort();
    println!("{}", palindromes.last().unwrap());
}

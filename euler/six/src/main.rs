fn main() {
    let mut square_sum: i64 = 0;
    let mut sum: i64 = 0;
    for x in 1..101 as i64 {
        square_sum += x.pow(2);
        sum += x;
    }

    println!("Difference: {}", sum.pow(2) - square_sum);
}

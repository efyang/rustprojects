#![feature(iter_cmp)]

fn main() {
    println!("longest recurring cycle : 1/{}", (2..1000usize).max_by(|x| longest_recurring_cycle(1, *x)).unwrap() );
}

fn longest_recurring_cycle(n: usize, d: usize) -> usize {
    let rem = n % d;
    let mut rems: Vec<usize> = vec![rem];
    let mut value: usize = (rem * 10) % d;
    let mut length: usize = 0;
    loop {
        value = (rem * value * 10) % d;
        if rem == 0 || rems.contains(&value) {
            break;
        }
        length += 1;
        rems.push(value);
    }
    length
}



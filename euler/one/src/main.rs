#![feature(step_by)]
use std::thread;
use std::sync::mpsc::channel;

fn sum_multiples_to_1000(base_number : i32) -> i32 {
    let mut sum: i32 = 0;
    for i in (base_number..1000).step_by(base_number){
        if base_number == 3 && i%5 != 0 || base_number == 5 {
            sum += i;
            println!("{}",i);
        }
    }
    return sum;
}

fn main() {
    let mut sum: i32 = 0;
    let mut base: i32 = 1;
    let (sender,receiver) = channel::<i32>();
    for _ in 0..2 {
        base += 2;
        let sender = sender.clone();
        thread::spawn(move || {
                sender.send(sum_multiples_to_1000(base)).unwrap();
        });
    }
    for _ in 0..2{
        sum += receiver.recv().unwrap();
    }
    println!("Sum is: {}",sum);
}

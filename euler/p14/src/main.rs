#![feature(step_by)]
use std::thread;
use std::sync::mpsc::channel;
extern crate num_cpus;

fn collatz_length(n : u64) -> u64 {
    let mut n: u64 = n.clone();
    let mut length: u64 = 1;
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
        length += 1;
    }
    return length;
}


fn main() {
    let max_threads = num_cpus::get();
    let mut max_items: (u64,u64) = (1,1);
    let (tx, rx) = channel::<(u64, u64)>();
    for n in (1..1000000u64).step_by(max_threads as u64) {
        for x in 0..max_threads as u64 {
            let tx = tx.clone();
            thread::spawn( move || {
                tx.send(((n + x),collatz_length((n + x)))).unwrap();
            });
        }
        for _ in 0..max_threads {
            let data = rx.recv().unwrap();
            if data.1 > max_items.1 {
                max_items = data.clone();
            }
        }
    }
    println!("{}",max_items.0);
}

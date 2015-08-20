#![feature(step_by)]
#![feature(iter_arith)]
use std::thread;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
extern crate num_cpus;

fn get_indices(thread_n: u64, total_proc: u64, list_length: u64) -> (u64, u64) {
    let start: u64 = thread_n * list_length/total_proc;
    let end: u64 = ((thread_n + 1) * list_length/total_proc) - 1;
    return (start, end) 
} 

fn sieve(limit: u64) -> Vec<u64> {
    let max_threads: u64 = num_cpus::get() as u64;
    let mut list: Vec<u64> = (3..limit).step_by(2).collect();
    list.reverse();
    let mut primes: Vec<u64> = vec![2,3];
    let mut prime: u64 = 3;

    while !list.is_empty() {
        list = list.into_iter().filter(|&item| item % prime != 0).collect();
        prime = list.pop().unwrap();
        primes.push(prime.clone());
    } 
    return primes;
}

fn main() {
    //println!("Sum: {}", sieve(2000000 as u64).iter().sum::<u64>());
}

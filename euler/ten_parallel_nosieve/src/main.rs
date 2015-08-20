#![feature(step_by)]
#![feature(iter_arith)]
#![feature(append)]
use std::thread;
//use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
extern crate num_cpus;

fn get_indices(thread_n: u64, total_proc: u64, list_length: u64) -> (u64, u64) {
    let start: u64 = thread_n * list_length/total_proc;
    let end: u64 = (thread_n + 1) * list_length/total_proc;
    return (start, end);
} 

fn vector_cut(vec: Vec<u64>, start: u64, end: u64) -> Vec<u64> {
    let mut new_vec: Vec<u64> = Vec::new();
    for i in (start as usize) ..(end as usize) {
        new_vec.push(vec.get(i).unwrap().clone());
    }
    return new_vec;
}

fn is_prime(n: u64) -> bool {
    if n % 2 == 0 {
        return false;
    }
    for x in (3..((n as f64).sqrt() as u64) + 1).step_by(2) {
        if n % x == 0 { return false;}
    }
    return true;
}

fn sieve(limit: u64) -> Vec<u64> {
    let max_threads: u64 = num_cpus::get() as u64;
    let mut list: Vec<u64> = (3..(limit + 1)).step_by(2).collect();
    let mut primes: Vec<u64> = vec![2,3];
    let list_len = list.len() as u64;

    let mut recvlist: Vec<Vec<u64>> = Vec::new();
    let mut list_accum: Vec<u64> = Vec::new();
    let (tx,rx) = channel();
    //let mut list_accum = Arc::new(Mutex::new(Vec::<u64>::new()));
    for t_num in 0..max_threads {
        let tx = tx.clone();
        let list_len = list_len.clone();
        let list = list.clone();
        let (start,mut end) = get_indices(t_num, max_threads, list_len);
        if t_num == max_threads - 1 {
            end = list_len - 1;
        }
        let list_piece: Vec<u64> = vector_cut(list, start, end);
        thread::spawn(move || {
            let newlist: Vec<u64> = list_piece.into_iter().filter(|&n| is_prime(n)).collect();
            tx.send(newlist.clone()).unwrap();
        });
    }
    for _ in 0..max_threads {
        recvlist.push(rx.recv().unwrap());
    }
    for item in recvlist {
        list_accum.append(&mut item.clone());
    }
    
    list.clone_from(&list_accum);
    primes.append(&mut list.clone());
    return primes;
}

fn main() {
    println!("Sum: {}", sieve(2000000 as u64).iter().sum::<u64>());
}

#![feature(step_by)]
#![feature(iter_arith)]
#![feature(append)]
use std::thread;
use std::sync::{Arc, Mutex};
extern crate num_cpus;

fn get_indices(thread_n: u64, total_proc: u64, list_length: u64) -> (u64, u64) {
    let start: u64 = thread_n * list_length/total_proc;
    let end: u64 = ((thread_n + 1) * list_length/total_proc) - 1;
    return (start, end) 
} 

fn vector_cut(vec: Vec<u64>, start: u64, end: u64) -> Vec<u64> {
    let mut new_vec: Vec<u64> = Vec::new();
    for i in (start as usize) ..(end as usize) {
        new_vec.push(vec.get(i).unwrap().clone());
    }
    return new_vec;
}

fn sieve(limit: u64) -> Vec<u64> {
    let max_threads: u64 = num_cpus::get() as u64;
    let mut list: Vec<u64> = (3..limit).step_by(2).filter(|&x| x % 3 != 0).collect();
    let mut newlist = Arc::new(Mutex::new(list.clone()));
    let mut primes: Vec<u64> = vec![2,3];
    let mut prime: u64 = 3;

    let mut list_len = list.len();
    while list_len >= max_threads as usize {
        for num in 0..max_threads {
            let newlist = newlist.clone();
            let list = list.clone();
            
            //spawn threads and partition off pieces of the main list (index and clone)
            thread::spawn(move || {
                let (start, end) = get_indices(num, max_threads, list_len as u64);
                let mut mylist: Vec<u64> = vector_cut(list,start,end);
                println!("{:?}",mylist.first());
                newlist.lock().unwrap().append(&mut mylist
                                               .into_iter()
                                               .filter(|&n| n % prime != 0)
                                               .collect()); 
            });
        }
        list.clone_from(&newlist.lock().unwrap().clone());
        list.sort();
        prime = list.remove(0);
        primes.push(prime.clone());
        list_len = list.len();
    }

    //while !list.is_empty() {
        //list = list.into_iter().filter(|&item| item % prime != 0).collect();
        //prime = list.pop().unwrap();
        //primes.push(prime.clone());
    //} 
    return primes;
}

fn main() {
    println!("Sum: {}", sieve(2000000 as u64).iter().sum::<u64>());
    //let mut primes: Vec<u64> = vec![2,3,4,5,6,7];
    //for x in 0..8 {
        //println!("{:?}",get_indices(x,8,8));
    //}
    //println!("{:?}",vector_cut(primes,0,2));
}

/*Given a positive integer, N, define the ’3N+1’ sequence star
ting from N as follows: If N is an even number, then divide N by two; but if N is odd, then multiply N by 3 and add 1. Continue
to generate numbers in this way until N becomes equal to 1. For
example, starting from N = 3, which is odd, we multiply by 3 and
add 1, giving N = 3*3+1 = 10. Then, since N is even, we divide
by 2, giving N = 10/2 = 5. We continue in this way, stopping
when we reach 1. The complete sequence is: 3, 10, 5, 16, 8, 4, 2,
1.
Write a program that will read a positive integer from the
user and will print out the 3N+1 sequence starting from that
integer. The program should also count and print out the numb
er of terms in the sequence*/ 
use std::io;

fn collatz(n : u64, history : &mut Vec<u64>){
    if n <= 1 {
        println!("Collatz sequence length: {}", history.len());
        print!("Collatz sequnce:");
        for item in history{
            print!(" {},", item);
        }
    }else{
        let new_n: u64;
        if n % 2 == 0{
            //even
            new_n = n / 2;
        }else{
            //odd
            new_n = n * 3 + 1;
        }
        history.push(new_n);
        collatz(new_n, history);
    }
}

fn main() {
    let mut raw_input = String::new();
    let input: u64;

    println!("Enter a positive integer.");

    io::stdin()
        .read_line(&mut raw_input)
        .ok()
        .expect("Readline Failed.");

    input = raw_input
        .trim()
        .parse()
        .ok()
        .expect("Cannot convert input to Integer.");

    collatz(input,&mut vec![input]);

}

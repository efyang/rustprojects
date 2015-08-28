fn fib_list_to_max(max: i64) -> Vec<i64>{
    let mut fib_list: Vec<i64> = vec![1,1];
    let mut index;
    let mut next_fib: i64;
    loop{
        index = fib_list.len() - 1;
        next_fib = fib_list[index] + fib_list[index - 1];
        if next_fib >= max{
            break
        }
        else{
            fib_list.push(next_fib);
        }
    }
    return fib_list;
}

fn even_sum(values: Vec<i64>) -> i64{
    return values
        .iter()
        .filter(|&x| x%2 == 0 )
        .fold(0i64, |acc, &item| acc + item);
}

fn main() {
    println!("{}",even_sum(fib_list_to_max(4000000)));
}

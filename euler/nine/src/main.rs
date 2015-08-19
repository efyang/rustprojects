fn main() {
    for x in 1..999 as u64{
        for y in 1..1000 as u64{
             for z in 1..1001 as u64{
                if x.pow(2) + y.pow(2) == z.pow(2) && x + y + z == 1000 {
                    println!("{}", x * y * z);
                    return;
                }
             }
        }
    }
}

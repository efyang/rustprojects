use std::io;



fn main() {
    let mut guess = String::new();
    println!("Enter something.");

    io::stdin().read_line(&mut guess)
        .ok()
        .expect("Readline failed.");

    println!("You entered \"{}\".", guess.trim());
}

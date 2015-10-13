#![feature(custom_derive)]
#![feature(convert)]
mod parser;

#[derive(Debug)]
pub enum Expr {
    Expr(String),
    Exprs(Box<Vec<Expr>>),
}

fn main() {
    match parser::parse(&"(+ 1 2 3 4)".to_string()) {
        Expr::Exprs(ref xs) => {println!("{:?}", xs)},
        _ => {},
    }
}

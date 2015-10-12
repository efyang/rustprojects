#![feature(custom_derive)]
#![feature(convert)]
mod parser;

pub enum Expr {
    Expr(Vec<String>),
    Exprs(Box<Expr>),
}

fn main() {
    match parser::parse(&"Hello World".to_string())[0] {
        Expr::Expr(ref xs) => {println!("{:#?}", xs)},
        _ => {},
    }
}

#![feature(custom_derive)]
#![feature(convert)]
mod parser;

pub struct Exprs(Vec<Expr>);

#[derive(debug)]
pub enum Expr {
    Vec(Vec<String>),
    Exprs,
}

fn main() {
    match parser::parse(&"Hello World".to_string())[0] {
        Expr::Vec(ref xs) => {println!("{:#?}", xs)},
        _ => {},
    }
}

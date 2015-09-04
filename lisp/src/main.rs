mod parse;

pub struct Exprs(Vec<Expr>);

pub enum Expr {
    Vec(Vec<String>),
    Exprs,
}

fn main() {
    println!("Hello, world!");
}

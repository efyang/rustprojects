#[derive(Debug)]
pub enum Number {
    Int(isize),
    Float(f64),
}

#[derive(Debug)]
pub enum Object {
    Symbol(String),
    Number(Number),
}

#[derive(Debug)]
pub enum Expr {
    Expr(Object),
    Exprs(Box<Vec<Expr>>),
}

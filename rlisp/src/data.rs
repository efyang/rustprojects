use std::collections::Hashmap;
use stdlisp::BASE_FUNCTIONS;

#[derive(Debug)] pub enum Expr {
    Expr(Object),
    Exprs(Box<Vec<Expr>>),
}

#[derive(Debug)]
pub enum Object {
    Symbol(String),
    Number(Number),
    List(Box<Vec<Object>>),
}

#[derive(Debug)]
pub enum Number {
    Int(isize),
    Float(f64),
}

#[derive(Debug)]
pub struct Env {
    functions: Vec<Function>,
    variables: Hashmap<(String, Object)>,
}

impl Env {
    pub fn new() -> Env {
        Env {
            methods: BASE_FUNCTIONS,
            vars: Hashmap::new(),
        }
    }
    pub fn with_functions(functions: Vec<Function>) -> Env {
        Env {
            methods: BASE_FUNCTIONS.concat(functions),
            vars: Hashmap::new(),
        }
    }
}

#[derive(Debug)]
pub struct Function {
    name: String,
    procedure: fn(Vec<Object>) -> Object,
}



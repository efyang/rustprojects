use std::collections::HashMap;
use stdlisp::BASE_FUNCTIONS;

#[derive(Debug, Clone)] 
pub enum Expr {
    Expr(Object),
    Exprs(Box<Vec<Expr>>),
}

#[derive(Debug, Clone)]
pub enum Object {
    Symbol(String),
    Number(Number),
    List(Box<Vec<Object>>),
}

#[derive(Debug, Clone)]
pub enum Number {
    Int(isize),
    Float(f64),
}

#[derive(Debug, Clone)]
pub struct Env<'a> {
    functions: Vec<Function<'a>>,
    variables: HashMap<String, Object>,
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            functions: BASE_FUNCTIONS,
            variables: HashMap::new(),
        }
    }
    pub fn with_functions(functions: Vec<Function<'a>>) -> Env {
        let mut funcs: Vec<Function> = BASE_FUNCTIONS.clone();
        funcs.clone_from_slice(&functions);
        Env {
            functions: funcs,
            variables: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Function<'a> {
    name: &'static str,
    procedure: &'a (fn(Vec<Object>) -> Object),
}



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

#[derive(Clone)]
pub struct Env<'a> {
    pub functions: Vec<Function<'a>>,
    pub variables: HashMap<String, Object>,
}

impl<'a> Env<'a> {
    pub fn new() -> Env<'a> {
        Env {
            functions: BASE_FUNCTIONS.to_vec(),
            variables: HashMap::new(),
        }
    }
    pub fn with_functions(functions: Vec<Function<'a>>) -> Env {
        let mut funcs: Vec<Function> = BASE_FUNCTIONS.to_vec().clone();
        funcs.clone_from_slice(&functions);
        Env {
            functions: funcs,
            variables: HashMap::new(),
        }
    }
    pub fn functions(&self) -> Vec<Function> {
        self.functions.clone()
    }
}

#[derive(Clone)]
pub struct Function<'a> {
    pub name: &'static str,
    pub procedure: &'a (fn(Vec<Object>, &mut Env) -> Object),
}



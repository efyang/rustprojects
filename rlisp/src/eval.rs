use data::*;
use stdlisp::BASE_FUNCTIONS;
use std::thread;

impl Expr {
    pub fn eval_no_panic(&self, env: Env) -> Expr {
        unimplemented!()
    }

    pub fn eval(&self, env: &mut Env) -> Object {
        if let &Expr::Exprs(objects) = self {
            let evaluated = objects
                .iter()
                .map(|&x| x.eval(&mut env))
                .collect::<Vec<Object>>();
            let splitted: (&Object, &[Object]) = evaluated.split_first().unwrap();
            let function_name = splitted.0;
            let args = splitted.1.to_vec();
            if let &Object::Symbol(ref fn_name) = function_name {
                eval_function(fn_name, args, env)
            } else {
                panic!("Invalid function name {:?}", function_name);
            }
        } else {
            if let &Expr::Expr(object) = self {
                object
            } else {
                panic!("Failed to eval {:?}", self);
            }
        }
    }
}

fn eval_function(function_name: &String, args: Vec<Object>, env: &mut Env) -> Object {
    let function = match_first_function(function_name, env.functions());
    (function.procedure)(args, &mut env)
}

fn match_first_function<'a>(function_name: &String, functions: Vec<Function<'a>>) -> Function<'a> {
    if functions.is_empty() {
        panic!("No such function {:?}", function_name);    
    }
    let splitted: (&Function, &[Function]) = functions.split_first().unwrap();
    let current = splitted.0;
    if &current.name.to_string() == function_name {
        current.clone()
    } else {
        match_first_function(function_name, splitted.1.to_vec())  
    }
}

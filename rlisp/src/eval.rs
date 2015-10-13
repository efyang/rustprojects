use data::*;
use stdlisp::BASE_FUNCTIONS;

trait Eval {
    fn eval(self, env: Env) -> Expr;
}

impl Eval for Expr::Expr {
    fn eval(self, env: Env) -> Expr {
        self
    }
}

impl Eval for Expr::Exprs {
    fn eval(self, env: Env) -> Expr {
        let evaluated = unbox(self)
            .iter()
            .map(|x| x.eval())
            .collect::<Vec<Expr>>();
        let splitted = evaluated.split_first().unwrap();
        let function_name = splitted.0;
        let args = splitted.1;
        if let Expr::Expr(Object::Symbol(fn_name)) = function_name {
            eval_method(fn_name, args)
        } else {
            panic!("Invalid function nae {:?}", function_name);
        }
    } 
}

fn eval_method(function_name: String, args: Vec<Object>, env: Env) -> Expr {
    function = match_first_function(function_name, env.functions);
    function.procedure(args)
}

fn match_first_function(function_name: String, functions: Vec<Method>, env: Env) -> Function {
    if functions.is_empty() {
        panic!("No such function {:?}", function_name);    
    }
    let splitted = functions.split_first().unwrap();
    let current = splitted.0;
    if current.name == function_name {
        current
    } else {
        match_first_function(function_name, splitted.1, env)  
    }
}

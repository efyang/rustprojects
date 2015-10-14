use data::*;

pub static BASE_FUNCTIONS: &'static [Function<'static>] = &[
    Function {name: "+", procedure: &(add as fn(Vec<Object>, &mut Env) -> Object)}
    //Function {name: "-", procedure: &(subtract as fn(Vec<Object>) -> Object)}
];

fn add(args: Vec<Object>, env: &mut Env) -> Object {
    unimplemented!();
}

fn subtract(args: Vec<Object>, env: &mut Env) -> Object {
    unimplemented!();
}


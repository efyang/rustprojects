use data::*;

pub const BASE_FUNCTIONS: Vec<Function<'static>> = vec![
    Function {name: "+", procedure: &(add as fn(Vec<Object>) -> Object)},
    Function {name: "-", procedure: &(subtract as fn(Vec<Object>) -> Object)}
];

fn add(args: Vec<Object>) -> Object {
    unimplemented!();
}

fn subtract(args: Vec<Object>) -> Object {
    unimplemented!();
}

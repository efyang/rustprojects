use data::*;

pub static BASE_FUNCTIONS: &'static [Function<'static>] = &[
    Function {name: "+", procedure: &(add as fn(Vec<Object>, &mut Env) -> Object)},
    //Function {name: "-", procedure: &(subtract as fn(Vec<Object>, &mut Env) -> Object)},
    Function {name: "list", procedure: &(list as fn(Vec<Object>, &mut Env) -> Object)}
];

fn add(args: Vec<Object>, env: &mut Env) -> Object {
    let numbers = args.iter()
        .map(|x| get_number(x))
        .collect::<Vec<Number>>();

    Object::Number(numbers.first().unwrap().clone())
}

//fn subtract(args: Vec<Object>, env: &mut Env) -> Object {
    //for object in args.iter() {
    
    //}
//}

fn get_number(object: &Object) -> Number {
    if let &Object::Number(ref n) = object {
        n.clone()
    } else {
        panic!("Object {:?} is not a number.", object);
    }
}

fn list(args: Vec<Object>, env: &mut Env) -> Object {
    Object::List(Box::new(args))
}


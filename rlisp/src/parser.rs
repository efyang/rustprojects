use data::*;
use std::fs::File;
use std::io::prelude::*;

pub fn parse_file(filename: &str) -> Expr {
    let mut f = File::open(filename).expect("Failed to open file.");
    let mut s = String::new();
    f.read_to_string(&mut s).expect("Failed to read file.");
    parse(&s)
}

pub fn parse(data: &String) -> Expr {
    let mut tokens = tokenize(&lines_to_spaces(&data))
        .iter()
        .rev()
        .map(|t| t.clone())
        .collect::<Vec<String>>();
    tokens_to_expr(&mut tokens)
}

fn tokens_to_expr(tokens: &mut Vec<String>) -> Expr {
    if tokens.is_empty() {
        panic!("No tokens to parse.");
    }
    let token: String;
    token = tokens.pop().unwrap();
    if token == "(" {
        let mut l = Vec::new();
        while tokens.last().unwrap().as_str() != ")" {
            l.push(tokens_to_expr(tokens));
        }
        tokens.pop().unwrap();
        Expr::Exprs(Box::new(l))
    } else if token == ")" {
        panic!("Unexpected )");
    } else {
        Expr::Expr(atomize(token))
    }
}

fn atomize(token: String) -> Object {
    if token.contains('.') {
        match token.parse::<f64>() {
            Ok(f) => Object::Number(Number::Float(f)),
            _ => atomize_string(token),
        }
    } else {
        match token.parse::<isize>() {
            Ok(i) => Object::Number(Number::Int(i)),
            _ => atomize_string(token),
        }
    }
}
//detecting strings with whitespace needs work in parser
fn atomize_string(token: String) -> Object {
    if token.char_at(0) == '\"' && token.char_at(token.len() - 1) == '\"' {
        Object::String((&token[1..(token.len() - 1)]).to_string())
    } else {
        Object::Symbol(token)
    }
}

fn tokenize(data: &String) -> Vec<String> {
    data.replace("(", " ( ")
        .replace(")", " ) ")
        .split_whitespace()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
}

fn lines_to_spaces(data: &String) -> String {
    data.replace("\r\n", " ").replace("\n", " ")
} 

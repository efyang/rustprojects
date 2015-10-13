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
    let parens = count_parens(&tokens);

    if parens.0 != parens.1 || parens.0 == 0 {
        panic!("Missing parentheses, cannot interpret program.");
    }
    
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
            l.push(tokens_to_expr(&mut tokens.to_owned()));
            tokens.pop().unwrap();
        }
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
            _ => Object::Symbol(token),
        }
    } else {
        match token.parse::<isize>() {
            Ok(i) => Object::Number(Number::Int(i)),
            _ => Object::Symbol(token),
        }
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
    data.as_str().replace("\n", " ")
} 

fn count_parens(tokens: &Vec<String>) -> (usize, usize) {
    tokens.iter().fold((0, 0), |acc, ref item| 
                       if item.as_str() == "(" {(acc.0 + 1, acc.1)}
                       else if item.as_str() == ")" {(acc.0, acc.1 + 1)}
                       else {acc})
}

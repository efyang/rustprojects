use super::Expr;

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
        Expr::Expr(token)
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

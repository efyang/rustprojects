use super::Expr;

pub fn parse(data: &String) -> Vec<Expr> {
    
    let data_vec: Vec<Vec<String>> = vec![vec!["quote".to_string(), "test".to_string()]];
    data_vec.iter().map(|x| Expr::Vec(x.to_owned())).collect::<Vec<Expr>>()
}

pub fn parse_single(data: &String) -> Expr {
    for chars in parse_single.iter() {
        chars
    }
}

fn lines_to_spaces(data: &String) -> String {
    data.as_str().replace("\n", " ")
} 

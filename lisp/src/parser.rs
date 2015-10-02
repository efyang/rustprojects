use super::Expr;

pub fn parse(data: &String) -> Vec<Expr> {
    
    let data_vec: Vec<Vec<String>> = vec![vec!["quote".to_string(), "test".to_string()]];
    data_vec.iter().map(|x| Expr::Vec(x.to_owned())).collect::<Vec<Expr>>()
}

fn line_to_space(data: &String) -> String {
    data.as_str().replace("\n", " ")
} 

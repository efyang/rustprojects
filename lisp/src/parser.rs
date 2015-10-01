use super::Expr;

pub fn parse(data: &String) -> Vec<Expr> {
    return vec![Expr::Vec(vec!["quote".to_string(), "test".to_string()])] 
}

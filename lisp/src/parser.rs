use super::Expr;

pub fn parse(data: &String) -> Vec<Expr> {
    let dataVec: Vec<Vec<String>> = vec![vec!["quote".to_string(), "test".to_string()]];
    dataVec.iter().map(|x| Expr::Vec(x.to_owned())).collect::<Vec<Expr>>()
}

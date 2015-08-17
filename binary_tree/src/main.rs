struct BinaryTree {
    data: std::Vec,
}

impl BinaryTree{
    fn make_binary(&mut self){
        if self.len() > 1 {
            if self.get(0) != 0{
                self.insert(0,0)
            }
        }
    }

    
}

fn main() {

}

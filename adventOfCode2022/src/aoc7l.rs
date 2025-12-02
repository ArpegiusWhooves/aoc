
 

pub enum Node {
    Empty,
    File(usize),
    Dir(Vec<Node>)
}




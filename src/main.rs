#[derive(Debug, Default)]
struct Node {
  pub value: usize,
  pub leaves: Box<Vec<Node>>
}

impl From<usize> for Node {
  fn from(value: usize) -> Self {
    Node::new(value)
  }
}

impl Node {
  pub fn new(value: usize) -> Self {
    Node {
      value,
      leaves: Box::new(vec![])
    }
  }

  pub fn set_leaves(mut self, leaves: Vec<Node>) -> Self  {
    self.leaves = Box::new(leaves);
    self
  }
}

fn root(value: usize) -> Node {
  Node::new(value)
}

fn main() {
  let tree = root(5)
    .set_leaves(vec![root(2).set_leaves(vec![root(3).set_leaves(vec![])]), root(7).set_leaves(vec![])]);
  println!("Tree {:?}", tree);
}

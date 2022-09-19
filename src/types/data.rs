use super::node::Node;

pub type Literal = i32;
pub type Clause = Vec<Literal>;
pub type Cnf = Vec<Clause>;
pub type Assignment = Vec<Node>;

use std::rc::{Rc, Weak};

use crate::elements::{Div, Span};




// enum NodeValue {
//   Text(String), // 文本
//   Div(Div), // div元素
//   Span(Span),
// }


#[derive(Default)]
struct Node {
  node_type: u8,
  node_name: String,
  node_value: Option<String>,

  parent_node: Option<Weak<Node>>,
  child_node: Option<Rc<Node>>,
  next_sibling: Option<Rc<Node>>,
  previous_sibling: Option<Rc<Node>>,
}

impl Node {
  fn new(node_name: String) -> Node {
    let mut node = Node::default();
    node.node_name = node_name;
    node
  }
}
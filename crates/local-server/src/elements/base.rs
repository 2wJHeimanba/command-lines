use crate::elements::{Div, Span};




enum NodeValue {
  Text(String), // 文本
  Div(Div), // div元素
  Span(Span),
}


struct BaseNode {
  node_type: u8,
  node_name: String,
  node_value: NodeValue,
}
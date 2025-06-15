// use std::fmt::Write;
// use std::{
//   cell::RefCell,
//   collections::HashMap,
//   fmt::{Debug, Display},
//   rc::{Rc, Weak},
// };

// use crate::tools::{escape_html_text, is_void_element};

// #[allow(dead_code)]
// #[derive(Default)]
// enum NodeType {
//   Document,
//   #[default]
//   Element, // 元素节点
//   Attr, // 节点属性
//   Text, // 文本节点
// }

// #[derive(Default)]
// enum NodeName {
//   Html,
//   Head,
//   #[default]
//   Div,
//   Span,
//   A,
//   Body,
//   P,
//   Meta,
//   Title,
// }

// impl Debug for NodeName {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     write!(f, "{self}")
//   }
// }
// impl Display for NodeName {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     write!(
//       f,
//       "{}",
//       match self {
//         NodeName::Div => "div",
//         NodeName::A => "a",
//         NodeName::Span => "span",
//         NodeName::Body => "body",
//         NodeName::P => "p",
//         NodeName::Head => "head",
//         NodeName::Html => "html",
//         NodeName::Meta => "meta",
//         NodeName::Title => "title",
//       }
//     )
//   }
// }

// impl Debug for NodeType {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     write!(
//       f,
//       "{}",
//       match self {
//         NodeType::Document => "文档节点",
//         NodeType::Element => "元素节点",
//         NodeType::Attr => "属性",
//         NodeType::Text => "文本节点",
//       }
//     )
//   }
// }

// #[allow(dead_code)]
// mod tools {
//   use super::Result;
//   use std::{env::current_dir, io::Write};

//   pub fn escape_html_text(s: &str) -> String {
//     s.chars()
//       .map(|c| match c {
//         '<' => "&lt;".to_string(),
//         '>' => "&gt;".to_string(),
//         '&' => "&amp;".to_string(),
//         '"' => "&quot;".to_string(),
//         '\'' => "&#39;".to_string(),
//         _ => c.to_string(),
//       })
//       .collect()
//   }

//   pub fn escape_html_attr(s: &str) -> String {
//     s.chars()
//       .map(|c| match c {
//         '"' => "&quot;".to_string(),
//         '\'' => "&#39;".to_string(),
//         '&' => "&amp;".to_string(),
//         _ => c.to_string(),
//       })
//       .collect()
//   }

//   pub fn is_void_element(tag: &str) -> bool {
//     matches!(
//       tag.to_lowercase().as_str(),
//       "area"
//         | "base"
//         | "br"
//         | "col"
//         | "embed"
//         | "hr"
//         | "img"
//         | "input"
//         | "link"
//         | "meta"
//         | "param"
//         | "source"
//         | "track"
//         | "wbr"
//     )
//   }

//   pub fn save(content: String) -> Result<()> {
//     let mut curr_path = current_dir()?;
//     curr_path.push("index.html");
//     let mut file = std::fs::OpenOptions::new()
//       .create(true)
//       .write(true)
//       .truncate(true)
//       .read(true)
//       .open(&curr_path)
//       .expect("文件创建失败");
//     // if curr_path.exists() {
//     //     println!("文件已经存在");
//     // } else {
//     file.write_all(content.as_bytes()).expect("文件写入失败");
//     println!("文件写入成功");
//     // }
//     Ok(())
//   }
// }

// #[allow(dead_code)]
// #[derive(Default)]
// struct Node {
//   node_type: NodeType,
//   node_name: NodeName,
//   node_value: Option<String>,
//   attributes: Option<HashMap<String, String>>,
//   is_single_tag: bool,

//   parent_node: Option<Weak<RefCell<Node>>>,
//   child_node: Option<Rc<RefCell<Node>>>,
//   next_sibling: Option<Rc<RefCell<Node>>>,
//   previous_sibling: Option<Weak<RefCell<Node>>>,
// }

// impl Debug for Node {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     f.debug_struct("node")
//       .field("node_type", &self.node_type)
//       .field("node_name", &self.node_name)
//       .field("attributes", &self.attributes)
//       .field("is_single_tag", &self.is_single_tag)
//       .field("child_node", &self.child_node)
//       .field("previous_node", &self.previous_sibling)
//       .field("next_sibling", &self.next_sibling)
//       .finish()
//   }
// }

// impl Node {
//   fn new() -> Self {
//     Node::default()
//   }

//   fn new_node(node_name: NodeName) -> Self {
//     let mut node = Node::new();
//     node.node_name = node_name;
//     node
//   }

//   fn new_text(node_value: String) -> Self {
//     let mut node = Self::new();
//     node.node_type = NodeType::Text;
//     node.node_value = Some(node_value);
//     node
//   }

//   fn last_child(&self) -> Option<Rc<RefCell<Self>>> {
//     // if self.child_node.is_none() {
//     //   return None;
//     // }
//     self.child_node.as_ref()?;

//     let mut current = self.child_node.as_ref().map(|v| v.clone());
//     while let Some(val) = current {
//       if val.borrow().next_sibling.is_none() {
//         return Some(val);
//       }
//       current = val.borrow().next_sibling.as_ref().map(|v| v.clone());
//     }
//     current
//   }

//   fn attr_insert<T: Into<String>>(&mut self, key: T, value: T) -> Result<()> {
//     if self.attributes.is_none() {
//       let mut store = HashMap::new();
//       store.insert(key.into(), value.into());
//       self.attributes = Some(store);
//     } else {
//       self
//         .attributes
//         .as_mut()
//         .unwrap()
//         .insert(key.into(), value.into());
//     }
//     Ok(())
//   }

//   fn append_child(&mut self, node: Node) -> Result<()> {
//     let node = Rc::new(RefCell::new(node));
//     let last_child = self.last_child();
//     if last_child.is_some() {
//       if let Some(ref child) = last_child {
//         let _ = node
//           .borrow_mut()
//           .previous_sibling
//           .insert(Rc::downgrade(child));
//         let _ = child.borrow_mut().next_sibling.insert(node);
//       }
//     } else {
//       self.child_node = Some(node);
//     }
//     Ok(())
//   }

//   fn to_html(&self) -> Result<String> {
//     let mut content = String::new();
//     self.write_html(&mut content)?;
//     Ok(content)
//   }

//   fn write_html(&self, output: &mut String) -> Result<()> {
//     match self.node_type {
//       NodeType::Element => self.write_element(output),
//       NodeType::Text => self.write_text(output),
//       _ => Ok(()),
//     }
//   }

//   fn write_element(&self, output: &mut String) -> Result<()> {
//     write!(output, "<{}", self.node_name)?;
//     if let Some(attr) = &self.attributes {
//       for (key, value) in attr {
//         write!(output, " {key}=\"{value}\"")?;
//       }
//     }

//     if is_void_element(&self.node_name.to_string()) {
//       write!(output, ">")?;
//     } else {
//       write!(output, ">")?;
//       if let Some(child) = &self.child_node {
//         let mut current = Some(Rc::clone(child));
//         while let Some(node) = current {
//           node.borrow().write_html(output)?;
//           current = node.borrow().next_sibling.as_ref().map(Rc::clone);
//         }
//       }
//       write!(output, "</{}>", self.node_name)?;
//     }
//     Ok(())
//   }

//   fn write_text(&self, output: &mut String) -> Result<()> {
//     if let Some(val) = &self.node_value {
//       write!(output, "{}", escape_html_text(val))?;
//     }
//     Ok(())
//   }
// }

// type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

use std::collections::HashMap;

use command_lines::{Node, NodeName, Result, tools};

fn main() -> Result<()> {
  let mut root_node = Node::new_node(NodeName::Html);

  let mut head_node = Node::new_node(NodeName::Head);

  let mut meta_node = Node::new_node(NodeName::Meta);
  meta_node.attr_insert("charset", "UTF-8")?;

  head_node.append_child(meta_node)?;

  let mut meta_node = Node::new_node(NodeName::Meta);
  meta_node.attr_insert("name", "viewport")?;
  meta_node.attr_insert("content", "width=device-width, initial-scale=1.0")?;
  head_node.append_child(meta_node)?;

  let mut title_node = Node::new_node(NodeName::Title);
  let title_content = Node::new_text("测试环境".into());
  title_node.append_child(title_content)?;
  head_node.append_child(title_node)?;
  root_node.append_child(head_node)?;
  let mut body_node = Node::new_node(NodeName::Body);
  let son_node_1 = Node::new_node(NodeName::Span);
  let mut son_node_2 = Node::new_node(NodeName::Div);
  let son_node_3 = Node::new_node(NodeName::P);
  let mut a_element = Node::new_node(NodeName::A);

  {
    let mut attrs = HashMap::new();
    attrs.insert("href".into(), "https://www.baidu.com".into());
    attrs.insert("target".into(), "_blank".into());
    let _ = a_element.attributes.insert(attrs);
  }

  let a_element_text = Node::new_text("点击跳转百度浏览器".into());
  a_element.append_child(a_element_text)?;

  son_node_2.append_child(a_element)?;

  body_node.append_child(son_node_1)?;
  body_node.append_child(son_node_2)?;
  body_node.append_child(son_node_3)?;

  root_node.append_child(body_node)?;

  println!("{}", root_node.to_html()?);
  tools::save(root_node.to_html()?)?;

  Ok(())
}

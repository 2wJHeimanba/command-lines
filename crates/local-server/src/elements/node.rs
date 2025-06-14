use std::{cell::RefCell, collections::HashMap, fmt::Debug, rc::{Rc, Weak}};

use crate::Result;

enum NodeType {
    DOCUMENT,
    ELEMENT, // 元素节点
    ATTR, // 节点属性
    TEXT, // 文本节点
}

impl Default for NodeType {
    fn default() -> Self {
        NodeType::ELEMENT
    }
}

impl Debug for NodeType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            NodeType::DOCUMENT => "文档节点",
            NodeType::ELEMENT => "元素节点",
            NodeType::ATTR => "属性",
            NodeType::TEXT => "文本节点"
        })
    }
}


#[derive(Default)]
struct Node {
    node_type: NodeType,
    node_name: String,
    node_value: Option<String>,
    attributes: Option<HashMap<String, String>>,

    parent_node: Option<Weak<Node>>,
    // child_nodes: Option<Vec<Rc<RefCell<Node>>>>,
    child_node: Option<Rc<RefCell<Node>>>,
    next_sibling: Option<Rc<RefCell<Node>>>,
    previous_sibling: Option<Weak<RefCell<Node>>>,
}

impl Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("node")
            .field("node_type", &self.node_type)
            .field("node_name", &self.node_name)
            .field("attributes", &self.attributes)
            .field("child_node", &self.child_node)
            .field("previous_node", &self.previous_sibling)
            .field("next_sibling", &self.next_sibling)
            .finish()
    }
}

impl Node {
    fn new<T: ToString>(node_name: T) -> Result<Self> {
        let mut node = Node::default();
        node.node_name = node_name.to_string().to_uppercase();
        Ok(node)
    }

    fn last_child(&self) -> Option<Rc<RefCell<Self>>> {
        if self.child_node.is_none() {
            return None;
        }
        let mut current = self.child_node.as_ref().map(|v| v.clone());
        while let Some(val) = current {
            if val.borrow().next_sibling.is_none() {
                return Some(val);
            }
            current = val.borrow().next_sibling.as_ref().map(|v| v.clone());
        }
        current
    }

    fn append_child(&mut self, node: Node) -> Result<()> {
        let node = Rc::new(RefCell::new(node));
        let last_child = self.last_child();
        if last_child.is_some() {
            (*node.borrow_mut()).previous_sibling = Some(Rc::downgrade(last_child.as_ref().unwrap()));
            (*last_child.as_ref().unwrap().borrow_mut()).next_sibling = Some(node);
        } else {
            self.child_node = Some(node);
        }
        Ok(())
    }
}



fn main() -> Result<()> {
    
    let mut root_node = Node::new("div")?;
    let son_node_1 = Node::new("span")?;
    let mut son_node_2 = Node::new("div")?;
    let son_node_3 = Node::new("p")?;

    son_node_2.append_child(Node::new("a")?)?;
    let mut attrs = HashMap::new();
    attrs.insert("class".to_string(), "name".to_string());
    attrs.insert("style".into(), "unknown".into());

    son_node_2.attributes = Some(attrs);

    root_node.append_child(son_node_1)?;
    root_node.append_child(son_node_2)?;
    root_node.append_child(son_node_3)?;




    println!("node: {:#?}", root_node);

    Ok(())
}

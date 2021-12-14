use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    pub children: Vec<Node>,
    pub nodetype: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Text(String),
    ElementType(Element),
}

#[derive(Debug)]
pub struct Element {
    tag: String,
    attr: Map,
}

pub type Map = HashMap<String, String>;

pub fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        nodetype: NodeType::Text(data),
    }
}

pub fn element(tag: String, attr: Map, children: Vec<Node>) -> Node {
    Node {
        children: children,
        nodetype: NodeType::ElementType(Element {
            tag: tag,
            attr: attr,
        }),
    }
}

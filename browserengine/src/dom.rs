use std::collections::HashMap;

struct Node {
    children: Vec<Node>,
    nodetype: NodeType,
}

enum NodeType {
    Text(String),
    ElementType(Element),
}

struct Element {
    tag: String,
    attr: Map,
}

type Map = HashMap<String, String>;

fn text(data: String) -> Node {
    Node {
        children: Vec::new(),
        nodetype: NodeType::Text(data),
    }
}

fn element(tag: String, attr: Map, children: Vec<Node>) -> Node {
    Node {
        children: children,
        nodetype: NodeType::ElementType(Element {
            tag: tag,
            attr: attr,
        }),
    }
}

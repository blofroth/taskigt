use self::NodeKind::*;
use slab::Slab;

// assume copy
pub type NodeIdx = usize;

pub enum NodeKind {
    Planned,
    Doing,
    Done,
    Info,
    Verbatim(Option<String>)
}

impl NodeKind {
    pub fn symbol(&self) -> &'static str {
        match self {
            Planned => "?",
            Doing => "*",
            Done => "#",
            Info => "-",
            Verbatim(_syntax) => "|"
        }
    }

    pub fn parse(text: &str) -> NodeKind {
        match &text[..1] {
            "?" => Planned,
            "*" => Doing,
            "#" => Done,
            "-" => Info,
            "|" => Verbatim(None),
            // should not really happen?
            _ => Info
        }
    }
}

pub struct Node {
    pub kind: NodeKind,
    pub text: String,
    pub children_ids: Vec<NodeIdx>,
    pub parent: Option<NodeIdx>
}

impl Node {
    pub fn display(&self) -> String {
        let mut out = self.kind.symbol().to_string();
        out.push(' ');
        out.push_str(&self.text);
        out
    }
}

pub struct Model {
    pub root_node_id: NodeIdx,
    pub nodes: Slab<Node>,
    pub title: String
}
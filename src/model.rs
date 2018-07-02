use self::NodeKind::*;
use std::collections::HashMap;

// assume copy
pub type NodeIdx = usize;

const INDENT_SZ: usize = 2;

#[derive(Clone, Debug, PartialEq)]
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

    pub fn parse(text: &str) -> Result<NodeKind, usize> {
        let first = text.chars().next();
        match first {
            Some('?') => Ok(Planned),
            Some('*') => Ok(Doing),
            Some('#') => Ok(Done),
            Some('-') => Ok(Info),
            Some('|') => Ok(Verbatim(None)),
            // should not really happen?
            None => Err(0),
            _ => Err(1)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Node {
    pub kind: NodeKind,
    pub text: String,
    pub children_ids: Vec<NodeIdx>
}


impl Node {

    pub fn leaf(kind: NodeKind, text: &str) -> Self {
        Node {
            kind,
            text: text.to_string(),
            children_ids: vec![]
        }
    }

    pub fn display(&self) -> String {
        let mut out = self.kind.symbol().to_string();
        out.push(' ');
        out.push_str(&self.text);
        out
    }

    pub fn parse(line: &str) -> Node {
        let maybe_kind = NodeKind::parse(&line);
        let skip = if let Err(skip) = maybe_kind {
            skip
        } else {
            2
        };

        println!("skip: {}, line: {}", skip, line);

        let kind = maybe_kind.unwrap_or(Info);

        let text: String = line.chars().skip(skip).collect();
        Node::leaf(kind, &text)
    }
}

// TODO: Eq not really true since nodes
// could have different ids but be the same tree
#[derive(Clone, Debug, PartialEq)]
pub struct Model {
    pub nodes: Vec<Node>,
    pub parents: Vec<Option<NodeIdx>>
}

impl Model {
    fn parse_line(line: &str) -> (usize, Node) {
        let nonspace_idx = line.find(|c| !char::is_whitespace(c));
        let spaces = nonspace_idx.unwrap_or(0);
        let indent = spaces / INDENT_SZ;

        let node = Node::parse(&line[spaces..]);
        println!("indent: {:?}, node: {:?}", indent, node);
        (indent, node)
    }

    pub fn parse(title: &str, content: &str) -> Self {
        // hålla koll på indentering?
        let mut model = Model::new(title);

        let mut last_at_indent = HashMap::new();
        let root = model.root();

        for line in content.lines() {
            let (indent, child) = Model::parse_line(line);
            let parent_id = *last_at_indent.get(&indent).unwrap_or(&root);
            let id = model.add_child(parent_id, child);
            last_at_indent.insert(indent + 1, id);
        }

        model
    }

    pub fn new(title: &str) -> Self {
        Model {
            nodes: vec![Node::leaf(Info, title)],
            parents: vec![None]
        }
    }

    pub fn root(&self) -> NodeIdx { 0 }

    pub fn title(&self) -> String {
        self.nodes[self.root()].text.clone()
    }

    pub fn add_child(&mut self, parent: NodeIdx, child: Node) -> NodeIdx {
        let idx = self.nodes.len();
        self.nodes.push(child);
        self.parents.push(Some(parent));
        self.nodes[parent].children_ids.push(idx);
        idx
    }

    pub fn add_child_at(&mut self, parent: NodeIdx, idx: usize, child: Node) -> NodeIdx {
        let id = self.nodes.len();
        self.nodes.push(child);
        self.parents.push(Some(parent));
        self.nodes[parent].children_ids.insert(idx, id);
        id
    }

    pub fn append(&mut self, parent: NodeIdx, kind: NodeKind, other: &mut Model) {
        let first_new_id = self.nodes.len();

        other.bump_indices(0, first_new_id);

        self.nodes.append(&mut other.nodes);
        self.parents.append(&mut other.parents.iter()
            .map(|maybe_id| maybe_id.map(|id| id + first_new_id ))
            .collect());
        self.parents[first_new_id] = Some(parent);
        self.nodes[parent].children_ids.push(first_new_id);

        self.nodes[first_new_id].kind = kind;
    }

    fn bump_indices(&mut self, parent: NodeIdx, bump: usize) {
        let children = {
            &self.nodes[parent].children_ids.clone()
        };
        for child_idx in children {
            Model::bump_indices(self, *child_idx, bump);
        }
        let old_ids = {
            self.nodes[parent].children_ids.clone()
        };
        self.nodes[parent].children_ids = old_ids.iter()
            .map(|id| id + bump)
            .collect();
    }

    pub fn parent(&self, id: NodeIdx) -> Option<NodeIdx> {
        self.parents[id]
    }
}

pub fn child(text: &str, children: &mut [(NodeKind, Model)]) -> Model {
    let mut curr = Model::new(text);
    let root = curr.root();

    for (kind, child) in children.into_iter() {
        curr.append(root, kind.clone(), child);
    }

    curr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_empty() {
        let doc = "";
        let model = Model::parse("the doc", doc);
        assert_eq!(
            child("the doc",&mut []),
            model
        );
    }

    #[test]
    fn parse_document() {
        let doc = "- dude\n  * sweet\n  ? what";
        let model = Model::parse("the doc", doc);
        assert_eq!(
            child("the doc",&mut [
                (Info, child("dude", &mut [
                    (Doing, child("sweet", &mut []) ),
                    (Planned, child("what", &mut []) )
                ]))
            ]),
            model
        );
    }
}
use self::ItemKind::*;
use std::collections::HashMap;

// assume copy
pub type ItemId = usize;

const INDENT_SZ: usize = 2;

#[derive(Clone, Debug, PartialEq)]
pub enum ItemKind {
    Planned,
    Doing,
    Done,
    Info,
    Verbatim(Option<String>)
    // possibly add empty/none?
}

impl ItemKind {
    pub fn symbol(&self) -> &'static str {
        match self {
            Planned => "?",
            Doing => "*",
            Done => "#",
            Info => "-",
            Verbatim(_syntax) => "|"
        }
    }

    pub fn parse(text: &str) -> (ItemKind, &str) {
        let first = text.chars().next();
        match first {
            Some('?') => (Planned, &text[1..]),
            Some('*') => (Doing, &text[1..]),
            Some('#') => (Done, &text[1..]),
            Some('-') => (Info, &text[1..]),
            Some('|') => (Verbatim(None), &text[1..]),
            // default to info
            None => (Info, text),
            _ => (Info, text)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Item {
    pub kind: ItemKind,
    pub text: String,
    pub children_ids: Vec<ItemId>
}


impl Item {

    pub fn leaf(kind: ItemKind, text: &str) -> Self {
        Item {
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

    pub fn parse(line: &str) -> Item {
        let (kind, rest) = ItemKind::parse(&line);
        let rest = if rest.chars().next() == Some(' ') {
            &rest[1..]
        } else {
            rest
        };

        Item::leaf(kind, rest)
    }
}

// TODO: Eq not really true since nodes
// could have different ids but be the same tree
#[derive(Clone, Debug, PartialEq)]
pub struct ItemTree {
    pub nodes: Vec<Item>,
    pub parents: Vec<Option<ItemId>>
}

impl ItemTree {
    fn parse_line(line: &str) -> (usize, Item) {
        let nonspace_idx = line.find(|c| !char::is_whitespace(c));
        let spaces = nonspace_idx.unwrap_or(0);
        let indent = spaces / INDENT_SZ;

        let node = Item::parse(&line[spaces..]);
        println!("indent: {:?}, node: {:?}", indent, node);
        (indent, node)
    }

    pub fn parse(title: &str, content: &str) -> Self {
        // hålla koll på indentering?
        let mut tree = ItemTree::new(title);

        let mut last_at_indent = HashMap::new();
        let root = tree.root();

        for line in content.lines() {
            let (indent, child) = ItemTree::parse_line(line);
            let parent_id = *last_at_indent.get(&indent).unwrap_or(&root);
            let id = tree.add_child(parent_id, child);
            last_at_indent.insert(indent + 1, id);
        }

        tree
    }

    pub fn new(title: &str) -> Self {
        ItemTree {
            nodes: vec![Item::leaf(Info, title)],
            parents: vec![None]
        }
    }

    pub fn root(&self) -> ItemId { 0 }

    pub fn title(&self) -> String {
        self.nodes[self.root()].text.clone()
    }

    pub fn add_child(&mut self, parent: ItemId, child: Item) -> ItemId {
        let id = self.nodes.len();
        self.nodes.push(child);
        self.parents.push(Some(parent));
        self.nodes[parent].children_ids.push(id);
        id
    }

    pub fn add_child_at(&mut self, parent: ItemId, pos: usize, child: Item) -> ItemId {
        let new_id = self.nodes.len();
        self.nodes.push(child);
        self.parents.push(Some(parent));
        if pos >= self.nodes[parent].children_ids.len() {
            self.nodes[parent].children_ids.push(new_id)
        } else {
            self.nodes[parent].children_ids.insert(new_id, pos);
        }
        new_id
    }

    pub fn append(&mut self, parent: ItemId, kind: ItemKind, other: &mut ItemTree) {
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

    fn bump_indices(&mut self, parent: ItemId, bump: usize) {
        let children = {
            &self.nodes[parent].children_ids.clone()
        };
        for child_id in children {
            ItemTree::bump_indices(self, *child_id, bump);
        }
        let old_ids = {
            self.nodes[parent].children_ids.clone()
        };
        self.nodes[parent].children_ids = old_ids.iter()
            .map(|id| id + bump)
            .collect();
    }

    pub fn parent(&self, id: ItemId) -> Option<ItemId> {
        self.parents[id]
    }
}

pub fn child(text: &str, children: &mut [(ItemKind, ItemTree)]) -> ItemTree {
    let mut curr = ItemTree::new(text);
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
        let tree = ItemTree::parse("the doc", doc);
        assert_eq!(
            child("the doc",&mut []),
            tree
        );
    }

    #[test]
    fn parse_document() {
        let doc = "- dude\n  * sweet\n  ? what";
        let tree = ItemTree::parse("the doc", doc);
        assert_eq!(
            child("the doc",&mut [
                (Info, child("dude", &mut [
                    (Doing, child("sweet", &mut []) ),
                    (Planned, child("what", &mut []) )
                ]))
            ]),
            tree
        );
    }

    #[test]
    fn parse_slim_item() {
        assert_eq!(Item::leaf(Info, "myitem"), Item::parse("-myitem"));
    }
}
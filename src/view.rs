use yew::prelude::*;
use yew::services::console::{ConsoleService};
use itemtree::{ItemTree, Item, ItemId, ItemKind};
use itemtree::ItemKind::*;
use storage::LocalDocumentStorage;
use std::mem;
use std::collections::HashSet;

pub struct Context {
    pub console: ConsoleService,
    pub storage: LocalDocumentStorage
}

pub enum Msg {
    // item tree manipulation
    Edit(ItemId, String),
    Delete(ItemId),
    Add(ItemId, usize),
    EditTitle(String),

    // folding
    ToggleFold(ClickEvent, ItemId),
    FoldOffspring(ItemId,bool),
    ExpandOffspring(ItemId,bool),
    // save/restore
    Save,
    EditRestoreDocument(String),
    Restore,

    // pasting
    EditPastedDocument(String),
    LoadFromPasted,

    Noop
}

/// The Taskigt-format consists of items, at different indentations
/// that form a tree, subject to:
/// * Each item is preceded by a bullet that indicates its type ('-','*','?','#','!', '|')
/// * In a well formatted document, each bullet is at char column divisible by 2
///   this means the first bullet should be in column 2 (0 indexed)
/// * In a well formatted document, each bullet is followed by a space
/// * A line can be all whitespace
///
/// Any text file can be parsed into the Taskigt format, and transformed to a well formatted
/// document. Such a transformation should only mean a few possible changes:
///  * Each line can be indented (in or out) by 1 space (or 2 if it starts at col 0)
///  * The content of a line can be prefixed with a bullet and a space
pub const README: &'static str =
r#"  - Hierarchical item based note taking
  - Different item types
    - Informational
    * Task (Doing)
    ? Task (Planned)
    # Task (Done)
    ! Task (Blocked/Waiting)
    | Verbatim/quote

  - Controls
    | <ctrl/cmd> + *left-click*
      - toggle item visibility (including sub item)
    | <enter>
      - Create new sub item (last of children, informational)
    | <tab>
      - Move to next item (horizontally)
    | <shift> + <tab>
      - Move to previous item (horizontally)
    | *clear content of item*
      - Deletes the item, if there are no sub-items
    | <ctrl> + <z>
      - Undo textual edits (note: not item removals/additions!)

  - Persistence
    - [Save document]: saves the document to local web storage, using the current title as the document name
      | https://developer.mozilla.org/en-US/docs/Web/API/Storage/LocalStorage
    - [Restore document]: Restore the document with the supplied name from local storage

  - Export/import from text
    - A textual representation of the current document is given under the 'As text' section
    - Import to the current document (overwriting it!) by pasting into the 'Paste document' area, and clicking [Load pasted]
"#;

pub struct Model {
    curr_tree: ItemTree,
    restore_document_name: String,
    pasted_document: String,
    hidden_node_ids: HashSet<ItemId>
}

impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        let curr_tree = ItemTree::parse("My items", README);
        Model {
            curr_tree,
            restore_document_name: "".to_string(),
            pasted_document: "".to_string(),
            hidden_node_ids: HashSet::new()
        }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        let root = self.curr_tree.root();
        match msg {
            Msg::Edit(id, new_value) => {
                if new_value.len() > 0 {
                    context.console.log(&format!("changed {} to {}", id, new_value));
                    let mut new_item = Item::parse(&new_value);
                    // use new item, old children_ids
                    mem::swap(&mut new_item, &mut self.curr_tree.nodes[id]);
                    mem::swap(&mut new_item.children_ids,
                              &mut self.curr_tree.nodes[id].children_ids);
                } else {
                    self.update(Msg::Delete(id), context);
                }
            },
            Msg::EditTitle(title) => { self.curr_tree.nodes[root].text = title; },
            Msg::Delete(child_id) => {
                let removed = self.curr_tree.remove_if_leaf(child_id);
                if removed {
                    context.console.log(&format!("del - {}", child_id));
                }
            }
            Msg::Add(parent_id, child_pos) => {
                context.console.log(&format!("add - at pos {} in node {}",
                                             child_pos, parent_id));
                self.curr_tree.add_child_at(parent_id, child_pos,
                                  Item::leaf(Info, ""));
            },
            Msg::ToggleFold(click, id) => {
                if click.meta_key() {
                    context.console.log(&format!("toggle node - {}", id));
                    let was_hidden = self.hidden_node_ids.contains(&id);
                    if was_hidden {
                        self.hidden_node_ids.remove(&id);
                    } else {
                        self.hidden_node_ids.insert(id);
                    }
                }
            }
            Msg::FoldOffspring(id, and_self) => {
                if and_self {
                    self.hidden_node_ids.insert(id);
                }
                for child_id in self.curr_tree.nodes[id].children_ids.clone() {
                    self.update(Msg::FoldOffspring(child_id, true), context);
                }
            },
            Msg::ExpandOffspring(id, and_self) => {
                if and_self {
                    self.hidden_node_ids.remove(&id);
                }
                for child_id in self.curr_tree.nodes[id].children_ids.clone() {
                    self.update(Msg::ExpandOffspring(child_id, true), context);
                }
            },
            Msg::Save => {
                context.storage.save(&self.curr_tree.title(),
                                     build_text(self.curr_tree.root(),
                                                &self.curr_tree.nodes));
            },
            Msg::EditRestoreDocument(doc_name) => {
                self.restore_document_name = doc_name;
            },
            Msg::Restore => {
                let mut parsed_tree = context.storage.restore(&self.restore_document_name)
                    .map(|doc| ItemTree::parse(&self.restore_document_name, &doc))
                    .expect("load document failure");

                mem::swap(&mut self.curr_tree, &mut parsed_tree);
            },
            Msg::EditPastedDocument(content) => {
                self.pasted_document = content;
            },
            Msg::LoadFromPasted => {
                let mut parsed_tree =
                    ItemTree::parse("Pasted", &self.pasted_document);
                mem::swap(&mut self.curr_tree, &mut parsed_tree);
            }
            Msg::Noop => {}
        }
        true
    }
}

fn kind_class(kind: &ItemKind) -> &'static str {
    match *kind {
        Verbatim(_) => "node-value-verbatim",
        _ => "node-value"
    }
}

fn view_item(id: ItemId, item: &Item) -> Html<Context, Model> {
    let new_pos = item.children_ids.len();
    html! {
        <input class=kind_class(&item.kind),
                oninput=|e| Msg::Edit(id, e.value),
                onclick=|e| Msg::ToggleFold(e, id),
                value=&item.display(),
                onkeypress=|e| {
                       if e.key() == "Enter" { Msg::Add(id, new_pos) } else { Msg::Noop }
               }, />
    }
}

fn view_node(node: ItemId, nodes: &Vec<Item>, hidden: &HashSet<ItemId>, display_item: bool) -> Html<Context, Model> {
    let hide_ya_kids = hidden.contains(&node);
    let num_children = nodes[node].children_ids.len();
    html! {
        <li>
            {
                if display_item {
                    view_item(node, &nodes[node])
                } else {
                    // hack for missing tag
                    html!{ <input type="hidden", />}
                }
            }

            {
                if hide_ya_kids {
                    if num_children > 0 {
                        html!{
                            <ul class="nodes",>
                                <li class="node-value", >{"[...]"}</li>
                            </ul>
                        }
                    } else {
                        // hack for missing tag
                        html!{ <input type="hidden", />}
                    }
                } else {
                    html!{
                        <ul class="nodes",>
                        { for nodes[node].children_ids.iter().map(|child_id| {
                            view_node(child_id.clone(), nodes, hidden, true)
                        })}
                        </ul>
                    }
                }
            }

        </li>
    }
}

fn build_text_rec(level: usize, buffer: &mut String, node: ItemId, nodes: &Vec<Item>,
                  display_item: bool) {
    if display_item {
        // assumes starts at level 1
        if nodes[node].kind != BlankLine {
            buffer.push_str(&" ".repeat((level-1) * 2));
            buffer.push_str(&nodes[node].display());
        }
        buffer.push('\n');
    }

    for child_id in &nodes[node].children_ids {
        build_text_rec(level + 1, buffer, *child_id, nodes, true);
    }
}

fn build_text(start: ItemId, nodes: &Vec<Item>) -> String {
    let mut buffer = String::new();
    build_text_rec(1, &mut buffer, start, nodes, false);
    buffer
}

fn view_as_text(node: ItemId, nodes: &Vec<Item>) -> Html<Context, Model> {
    html! {
        <div>
            <h1>{ "As text" }</h1>
            <pre>{ build_text(node, nodes) }</pre>
        </div>
    }
}

fn paste_area(content: &str) -> Html<Context, Model> {
    html! {
        <div>
            <h2>{ "Paste document" }</h2>
            <button onclick=|_| Msg::LoadFromPasted,>
                { "Load pasted" }
            </button>
            <br />
            <textarea rows=40, cols=120,
                value=content,
                oninput=|e| Msg::EditPastedDocument(e.value),
                placeholder="pasted document",>
            </textarea>

        </div>
    }
}

impl Renderable<Context, Model> for Model {
    fn view(&self) -> Html<Context, Self> {
        html! {
            <div>
                <nav class="menu",>
                </nav>
                <div>
                    <div>
                        <input class="document-title",
                            oninput=|e| Msg::EditTitle(e.value),
                            onclick=|e| Msg::ToggleFold(e, 0),
                            value=&self.curr_tree.title(), />
                    </div>
                    <div>
                        <button onclick=|_| Msg::Save,>
                            { "Save document" }
                        </button>
                        <br />
                        <button onclick=|_| Msg::FoldOffspring(0, false),>
                            { "Fold all" }
                        </button>
                        <button onclick=|_| Msg::ExpandOffspring(0, false),>
                            { "Expand all" }
                        </button>
                        <br />
                        <input
                            oninput=|e| Msg::EditRestoreDocument(e.value),
                            value=&self.restore_document_name, />
                        <button onclick=|_| Msg::Restore, >
                            { "Restore document" }
                        </button>
                    </div>
                    <ul class="nodes",>
                        { view_node(self.curr_tree.root(), &self.curr_tree.nodes, &self.hidden_node_ids, false) }
                    </ul>
                </div>
                { view_as_text(self.curr_tree.root(), &self.curr_tree.nodes) }
                { paste_area(&self.pasted_document) }
            </div>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn readme_is_well_formatted() {
        let tree = ItemTree::parse("readme", README);
        let tree_as_text = build_text(0, &tree.nodes);
        assert_eq!(README, tree_as_text);
    }
}
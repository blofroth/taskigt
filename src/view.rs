use yew::prelude::*;
use yew::services::console::{ConsoleService};
use yew::virtual_dom::vtext::VText;
use yew::virtual_dom::vnode::VNode;
use itemtree::{ItemTree, Item, ItemId, ItemKind};
use itemtree::ItemKind::*;
use storage::LocalDocumentStorage;
use std::mem;

pub struct Context {
    pub console: ConsoleService,
    pub storage: LocalDocumentStorage
}

pub enum Msg {
    Edit(ItemId, String),
    EditTitle(String),
    EditRestoreDocument(String),
    Delete(ItemId),
    Add(ItemId, usize),
    Save,
    Restore,
    EditPastedDocument(String),
    LoadFromPasted,
    Noop
}

pub struct Model {
    curr_tree: ItemTree,
    restore_document_name: String,
    pasted_document: String
}

impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        let mut curr_tree = ItemTree::new("My items");
        let root = curr_tree.root();
        curr_tree.add_child(root, Item::leaf(Info, ""));
        Model {
            curr_tree,
            restore_document_name: "".to_string(),
            pasted_document: "".to_string()
        }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        let root = self.curr_tree.root();
        match msg {
            Msg::Edit(id, new_value) => {
                if new_value.len() > 0 {
                    context.console.log(&format!("changed {} to {}", id, new_value));
                    self.curr_tree.nodes[id] = Item::parse(&new_value);
                } else {
                    self.update(Msg::Delete(id), context);
                }
            },
            Msg::EditTitle(title) => { self.curr_tree.nodes[root].text = title; },
            Msg::Delete(child_id) => {
                let parent_id = self.curr_tree.parent(child_id);
                let num_children = self.curr_tree.nodes[child_id].children_ids.len();
                parent_id.map(|id| {
                    if num_children == 0 {
                        context.console.log(&format!("del - {} from {}", child_id, id));
                        let index = self.curr_tree.nodes[id].children_ids.iter()
                            .position(|child| *child == child_id).unwrap();
                        self.curr_tree.nodes[id].children_ids
                            .remove(index);
                    }
                });
            }
            Msg::Add(parent_id, child_pos) => {
                context.console.log(&format!("add - at pos {} in node {}",
                                             child_pos, parent_id));
                self.curr_tree.add_child_at(parent_id, child_pos,
                                  Item::leaf(Info, ""));
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
                value=&item.display(),
                onkeypress=|e| {
                       if e.key() == "Enter" { Msg::Add(id, new_pos) } else { Msg::Noop }
               }, />
    }
}

fn view_node(node: ItemId, nodes: &Vec<Item>, display_item: bool) -> Html<Context, Model> {
    html! {
        <li>
            {
                if display_item {
                    view_item(node, &nodes[node])
                } else {
                    // hack for missing tag
                    VNode::VText(VText::new("".to_string()))
                }
            }
            <ul class="nodes",>
            { for nodes[node].children_ids.iter().map(|child_id| {
                view_node(child_id.clone(), nodes, true)
            })}
            </ul>
        </li>
    }
}

fn build_text_rec(level: usize, buffer: &mut String, node: ItemId, nodes: &Vec<Item>,
                  display_item: bool) {
    if display_item {
        buffer.push_str(&" ".repeat(level * 2));
        buffer.push_str(&nodes[node].display());
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
            <h1>{ "Export" }</h1>
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
                    <input class="document-title",
                        oninput=|e| Msg::EditTitle(e.value),
                        value=&self.curr_tree.title(), />
                    <ul class="nodes",>
                        { view_node(self.curr_tree.root(), &self.curr_tree.nodes, false) }
                    </ul>
                </div>
                <button onclick=|_| Msg::Save,>
                    { "Save document" }
                </button>
                <br />
                <input
                    oninput=|e| Msg::EditRestoreDocument(e.value),
                    value=&self.restore_document_name, />
                <button onclick=|_| Msg::Restore,>
                    { "Restore document" }
                </button>
                { view_as_text(self.curr_tree.root(), &self.curr_tree.nodes) }
                { paste_area(&self.pasted_document) }
            </div>
        }
    }
}
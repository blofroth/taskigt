use yew::prelude::*;
use yew::services::console::{ConsoleService};
use model::{Model, Node, NodeIdx, NodeKind};
use slab::Slab;
use model::NodeKind::*;
use storage::LocalDocumentStorage;

pub struct Context {
    pub console: ConsoleService,
    pub storage: LocalDocumentStorage
}

pub enum Msg {
    Edit(NodeIdx, String),
    EditTitle(String),
    Delete(NodeIdx),
    Add(NodeIdx, usize),
    Save,
    Restore(String),
    Noop
}

impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        let mut nodes = Slab::new();
        let root_node_id = nodes.insert(Node {
            kind: Info,
            text: "Task".to_string(),
            children_ids: vec![],
            parent: None
        });
        let title = "".to_string();
        Model { root_node_id, nodes, title }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Edit(idx, new_value) => {
                if new_value.len() > 0 {
                    context.console.log(&format!("changed {} to {}", idx, new_value));
                    self.nodes[idx].kind = NodeKind::parse(&new_value);
                    if new_value.len() < 3 {
                        self.nodes[idx].text = "".to_string();
                    } else {
                        self.nodes[idx].text = new_value[2..].to_string();
                    }
                } else {
                    self.update(Msg::Delete(idx), context);
                }
            },
            Msg::EditTitle(title) => { self.title = title; },
            Msg::Delete(child_idx) => {
                let parent_idx = self.nodes[child_idx].parent;
                let num_children = self.nodes[child_idx].children_ids.len();
                parent_idx.map(|idx| {
                    if num_children == 0 {
                        context.console.log(&format!("del - {} from {}", child_idx, idx));
                        self.nodes[idx].children_ids
                            .remove_item(&child_idx);
                    }
                });
            }
            Msg::Add(parent_idx, child_pos) => {
                context.console.log(&format!("add - at pos {} in node {}",
                                             child_pos, parent_idx));
                let new_child = self.nodes.insert(Node {
                    kind: Info,
                    text: "".to_string(),
                    children_ids: vec![],
                    parent: Some(parent_idx)
                });
                self.nodes[parent_idx].children_ids
                    .insert(child_pos, new_child);
            },
            Msg::Save => {
                context.storage
                    .save(&self.title, build_text_root(&self.root_node_id, &self.nodes));
            }
            Msg::Restore(_title) => {
                // TODO, be able to parse document to tree
            }
            Msg::Noop => {}
        }
        true
    }
}

fn view_node(node: NodeIdx, nodes: &Slab<Node>) -> Html<Context, Model> {
    let new_idx = nodes[node].children_ids.len();
    html! {
        <li>
            <input class="node-value",
                oninput=|e| Msg::Edit(node, e.value),
                value=&nodes[node].display(),
                onkeypress=|e| {
                       if e.key() == "Enter" { Msg::Add(node, new_idx) } else { Msg::Noop }
               }, />
            <ul class="nodes",>
            { for nodes[node].children_ids.iter().map(|child_id| {
                view_node(child_id.clone(), nodes)
            })}
            </ul>
        </li>
    }
}

fn build_text(level: usize, buffer: &mut String, node: &NodeIdx, nodes: &Slab<Node>) {
    buffer.push_str(&" ".repeat(level * 2));
    buffer.push_str(&nodes[*node].display());
    buffer.push('\n');
    for child_id in &nodes[*node].children_ids {
        build_text(level + 1, buffer, child_id, nodes);
    }
}

fn build_text_root(node: &NodeIdx, nodes: &Slab<Node>) -> String {
    let mut buffer = String::new();
    build_text(1, &mut buffer, node, nodes);
    buffer
}

fn view_as_text(node: NodeIdx, nodes: &Slab<Node>) -> Html<Context, Model> {
    html! {
        <div>
            <h1>{ "Export" }</h1>
            <pre>{ build_text_root(&node, nodes) }</pre>
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
                    <ul class="nodes",>
                        { view_node(self.root_node_id, &self.nodes) }
                    </ul>
                </div>
                <input
                    oninput=|e| Msg::EditTitle(e.value),
                    value=&self.title, />
                <button onclick=|_| Msg::Save,>
                    { "Save document" }
                </button>
                { view_as_text(self.root_node_id, &self.nodes) }
            </div>
        }
    }
}
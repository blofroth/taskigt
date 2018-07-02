use yew::prelude::*;
use yew::services::console::{ConsoleService};
use model::{Model, Node, NodeIdx};
use model::NodeKind::*;
use storage::LocalDocumentStorage;
use std::mem;

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
    Restore,
    Noop
}

impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        Model::new("")
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        let root = self.root();
        match msg {
            Msg::Edit(idx, new_value) => {
                if new_value.len() > 0 {
                    context.console.log(&format!("changed {} to {}", idx, new_value));
                    self.nodes[idx] = Node::parse(&new_value);
                } else {
                    self.update(Msg::Delete(idx), context);
                }
            },
            Msg::EditTitle(title) => { self.nodes[root].text = title; },
            Msg::Delete(child_idx) => {
                let parent_idx = self.parent(child_idx);
                let num_children = self.nodes[child_idx].children_ids.len();
                parent_idx.map(|idx| {
                    if num_children == 0 {
                        context.console.log(&format!("del - {} from {}", child_idx, idx));
                        let index = self.nodes[idx].children_ids.iter()
                            .position(|child| *child == child_idx).unwrap();
                        self.nodes[idx].children_ids
                            .remove(index);
                    }
                });
            }
            Msg::Add(parent_idx, child_pos) => {
                context.console.log(&format!("add - at pos {} in node {}",
                                             child_pos, parent_idx));
                self.add_child_at(parent_idx, child_pos,
                                  Node::leaf(Info, ""));
            },
            Msg::Save => {
                context.storage.save(&self.title(),
                                     build_text(self.root(), &self.nodes));
            }
            Msg::Restore => {
                let title = self.title();
                let mut model = context.storage.restore(&title)
                    .map(|doc| Model::parse(&title, &doc))
                    .expect("load document failure");

                mem::swap(self, &mut model);
            }
            Msg::Noop => {}
        }
        true
    }
}

fn view_node(node: NodeIdx, nodes: &Vec<Node>) -> Html<Context, Model> {
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

fn build_text_rec(level: usize, buffer: &mut String, node: NodeIdx, nodes: &Vec<Node>) {
    buffer.push_str(&" ".repeat(level * 2));
    buffer.push_str(&nodes[node].display());
    buffer.push('\n');
    for child_id in &nodes[node].children_ids {
        build_text_rec(level + 1, buffer, *child_id, nodes);
    }
}

fn build_text(start: NodeIdx, nodes: &Vec<Node>) -> String {
    let mut buffer = String::new();
    build_text_rec(1, &mut buffer, start, nodes);
    buffer
}

fn view_as_text(node: NodeIdx, nodes: &Vec<Node>) -> Html<Context, Model> {
    html! {
        <div>
            <h1>{ "Export" }</h1>
            <pre>{ build_text(node, nodes) }</pre>
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
                        { view_node(self.root(), &self.nodes) }
                    </ul>
                </div>
                <input
                    oninput=|e| Msg::EditTitle(e.value),
                    value=self.title(), />
                <button onclick=|_| Msg::Save,>
                    { "Save document" }
                </button>
                <button onclick=|_| Msg::Restore,>
                    { "Restore document" }
                </button>
                { view_as_text(self.root(), &self.nodes) }
            </div>
        }
    }
}
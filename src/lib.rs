#![feature(vec_remove_item)]

#[macro_use]
extern crate yew;
extern crate slab;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use NodeKind::*;
use slab::Slab;

// assume copy
type NodeIdx = usize;

pub struct Context {
    pub console: ConsoleService
}

pub enum NodeKind {
    Planned,
    Doing,
    Done,
    Info,
    Verbatim(Option<String>)
}

pub struct Node {
    kind: NodeKind,
    text: String,
    children_ids: Vec<NodeIdx>,
    parent: Option<NodeIdx>
}

pub struct Model {
    root_node_id: NodeIdx,
    nodes: Slab<Node>
}

pub enum Msg {
    Edit(NodeIdx, String),
    Delete(NodeIdx),
    Add(NodeIdx, usize),
    Noop
}

impl Component<Context> for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<Context, Self>) -> Self {
        let mut nodes = Slab::new();
        let root_node_id = nodes.insert(Node {
            kind: Info,
            text: "Root".to_string(),
            children_ids: vec![],
            parent: None
        });
        Model { root_node_id, nodes }
    }

    fn update(&mut self, msg: Self::Message, context: &mut Env<Context, Self>) -> ShouldRender {
        match msg {
            Msg::Edit(idx, new_value) => {
                if new_value.len() > 0 {
                    context.console.log(&format!("changed {} to {}", idx, new_value));
                    self.nodes[idx].text = new_value;
                } else {
                    self.update(Msg::Delete(idx), context);
                }
            }
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
                    text: "Child".to_string(),
                    children_ids: vec![],
                    parent: Some(parent_idx)
                });
                self.nodes[parent_idx].children_ids
                    .insert(child_pos, new_child);
            },
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
                value=&nodes[node].text,
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
            </div>
        }
    }
}

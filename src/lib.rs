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
    children_ids: Vec<NodeIdx>
}

pub struct Model {
    root_node_id: NodeIdx,
    nodes: Slab<Node>
}

pub enum Msg {
    Edit(NodeIdx, String),
    Delete(NodeIdx, NodeIdx),
    Add(NodeIdx, usize),
    Noop
}

impl<CTX> Component<CTX> for Model
    where
        CTX: AsMut<ConsoleService>,
{
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        let mut nodes = Slab::new();
        let root_node_id = nodes.insert(Node {
            kind: Info,
            text: "Root".to_string(),
            children_ids: vec![]
        });
        Model { root_node_id, nodes }
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::Edit(idx, new_value) => {
                if new_value.len() > 0 {
                    context.as_mut().log(&format!("changed {} to {}", idx, new_value));
                    self.nodes[idx].text = new_value;
                } else {
                    // TODO should delete, need parent index
                    context.as_mut().log(&format!("TODO delete {}", idx));
                }
            }
            Msg::Delete(parent_idx, child_idx) => {
                context.as_mut().log(&format!("del - {} from {}", child_idx, parent_idx));
                self.nodes[parent_idx].children_ids
                    .remove_item(&child_idx);
            }
            Msg::Add(parent_idx, child_pos) => {
                context.as_mut().log(&format!("add - at pos {} in node {}",
                                              child_pos, parent_idx));
                let new_child = self.nodes.insert(Node {
                    kind: Info,
                    text: "Child".to_string(),
                    children_ids: vec![]
                });
                self.nodes[parent_idx].children_ids
                    .insert(child_pos, new_child);
            },
            Msg::Noop => {}
        }
        true
    }
}

fn view_node<CTX>(node: NodeIdx, nodes: &Slab<Node>) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static,
{
    let new_idx = nodes[node].children_ids.len();
    html! {
        <li>
            <input class="node-value", value=&nodes[node].text,
                oninput=move |e: InputData| Msg::Edit(node, e.value),
                onkeypress=move |e: KeyData| {
                       if e.key == "Enter" { Msg::Add(node, new_idx) } else { Msg::Noop }
               }, />
            <ul class="nodes",>
            { for nodes[node].children_ids.iter().map(|child_id| {
                view_node(child_id.clone(), nodes)
            })}
            </ul>
        </li>
    }
}

impl<CTX> Renderable<CTX, Model> for Model
    where
        CTX: AsMut<ConsoleService> + 'static,
{
    fn view(&self) -> Html<CTX, Self> {
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

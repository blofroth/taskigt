extern crate stdweb;
#[macro_use]
extern crate yew;

use stdweb::web::Date;
use yew::prelude::*;
use yew::services::console::ConsoleService;
use NodeKind::*;

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
    children: Vec<Node>
}

pub struct Model {
    root_node: Node
}

pub enum Msg {
    Edit(String),
    Delete(usize),
    Add(usize),
}

impl<CTX> Component<CTX> for Model
    where
        CTX: AsMut<ConsoleService>,
{
    type Msg = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: &mut Env<CTX, Self>) -> Self {
        Model { root_node: Node {
            kind: Info,
            text: "Root".to_string(),
            children: vec![]
        }}
    }

    fn update(&mut self, msg: Self::Msg, context: &mut Env<CTX, Self>) -> ShouldRender {
        match msg {
            Msg::Edit(new_value) => {
                context.as_mut().log(&format!("changed root to {}", new_value));
                self.root_node.text = new_value;
            }
            Msg::Delete(_) => {
                context.as_mut().log("del - noop for now");
            }
            Msg::Add(_) => {
                context.as_mut().log("add - noop for now");
            },
        }
        true
    }
}

fn view_node<CTX>(node: &Node) -> Html<CTX, Model>
    where
        CTX: AsMut<ConsoleService> + 'static,
{
    html! {
        <li>
            <input class="node-value", value=node.text.clone(), />
            <ul class="nodes",>
            { for node.children.iter().map(|child| {
                view_node(child)
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
                    <button onclick=|_| Msg::Edit("dude".to_string()),>{ "Dude!" }</button>
                    <button onclick=|_| Msg::Add(0),>{ "Add" }</button>
                    <button onclick=|_| Msg::Delete(0),>{ "Delete" }</button>
                </nav>
                <div>
                    <ul class="nodes",>
                        { view_node(&self.root_node) }
                    </ul>
                </div>
            </div>
        }
    }
}

extern crate yew;
extern crate taskigt;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use taskigt::Model;
use taskigt::Context;

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService::new(),
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();

    yew::run_loop();
}
extern crate yew;
extern crate taskigt;

use yew::prelude::*;
use yew::services::console::ConsoleService;
use taskigt::model::Model;
use taskigt::view::Context;
use taskigt::storage::LocalDocumentStorage;

fn main() {
    yew::initialize();
    let context = Context {
        console: ConsoleService::new(),
        storage: LocalDocumentStorage::new()
    };
    let app: App<_, Model> = App::new(context);
    app.mount_to_body();

    yew::run_loop();
}
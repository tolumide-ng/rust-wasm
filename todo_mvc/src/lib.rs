use wasm_bindgen::prelude::*;
use std::rc::Rc;

/// Stores items into localstorage
pub mod store;
/// Element wrapper to the DOM
pub mod element;
/// Controller of the program
pub mod controller;
/// Presentation layer
pub mod view;
/// Handles constructing template strings from data
pub mod template;
pub mod scheduler;

use crate::scheduler::Scheduler;
use view::{View, ViewMessage};
use controller::{ControllerMessage};
use store::Store;
use controller::Controller;

/// Message wrapper enum used to pass through the scheduler to the Controller or View
pub enum Message {
    /// Message wrapper to send to the controller
    Controller(ControllerMessage),
    /// Message wrapper to send to the view
    View(ViewMessage),
}

/// Used for debugging to the console
pub fn exit(message: &str) {
    let v = wasm_bindgen::JsValue::from_str(&message.to_string());
    web_sys::console::exception_1(&v);
    std::process::abort();
}

fn app(name: &str) {
    let sched = Rc::new(Scheduler::new());
    let store = match Store::new(name) {
        Some(s) => s,
        None => return,
    };
    let controller = Controller::new(store, Rc::downgrade(&sched));
    if let Some(mut view) = View::new(Rc::clone(&sched)) {
        let sch: &Rc<Scheduler> = &sched;
        view.init();
        sch.set_view(view);
        sch.set_controller(controller);
        sched.add_message(Message::Controller(ControllerMessage::SetPage("".to_string(),)));
    }
}

/// Entry point into the program from Javascript
#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    app("todos-wasmbindgen");

    Ok(())
}
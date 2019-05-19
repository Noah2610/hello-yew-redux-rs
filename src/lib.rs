extern crate redux_rs;
extern crate shred;
extern crate yew;

mod yew_prelude {
    pub use yew::{
        html,
        Callback,
        Component,
        ComponentLink,
        Html,
        IKeyboardEvent,
        Renderable,
        ShouldRender,
    };
}

mod components;
mod entry_data;
mod store;

use std::sync::mpsc::Sender;

use yew::services::ConsoleService;

use components::prelude::*;
use entry_data::prelude::*;
use store::prelude::*;
use yew_prelude::*;

/// Entry point
pub fn run() {
    /// Start root model
    yew::start_app::<Model>();
}

/// Debug function; print to JS console
pub fn consolelog<T: ToString>(msg: T) {
    yew::services::ConsoleService::new().log(&msg.to_string())
}

pub enum Msg {
    InputNewTodo(String),
    AddNewTodo,
    UpdateEntry(EntryData),
    Noop,
}

/// Root component
pub struct Model {
    store_handle:   StoreHandle,
    console:        ConsoleService,
    input_new_todo: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        /// Initialize redux store
        let store_handle = store::initialize_store();
        /// Create actual root component
        Self {
            store_handle:   store_handle,
            console:        ConsoleService::new(),
            input_new_todo: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::InputNewTodo(input) => {
                let changed = self.input_new_todo != input;
                self.input_new_todo = input;
                changed
            }
            Msg::AddNewTodo => {
                self.store_handle.store.dispatch(Action::AddEntryWithName(
                    self.input_new_todo.clone(),
                ));
                // self.entries.push(EntryData {
                //     id: self.entries.len(),
                //     name: self.input_new_todo.clone(),
                //     ..Default::default()
                // });
                self.input_new_todo = String::new();
                true
            }
            Msg::UpdateEntry(data) => {
                false
                // TODO
                // if let Some(entry) =
                //     self.entries.iter_mut().find(|entry| data.id == entry.id)
                // {
                //     *entry = data;
                //     true
                // } else {
                //     false
                // }
            }
            Msg::Noop => false,
        }
    }
}

impl Renderable<Self> for Model {
    fn view(&self) -> Html<Self> {
        html! {
            <div class="todoapp",>
                <header class="header",>
                    <h1>{ "Todos..." }</h1>
                    { self.view_create_entry() }
                </header>
                { self.view_entry_list() }
            </div>
        }
    }
}

impl Model {
    fn view_entry_list(&self) -> Html<Self> {
        html! {
            <EntryList: store=self.store_handle.store, />
        }
    }

    fn view_create_entry(&self) -> Html<Self> {
        html! {
            <input
                type="text",
                class="new-todo",
                value=&self.input_new_todo,
                oninput=|e| Msg::InputNewTodo(e.value),
                onkeypress=|e| if e.key() == "Enter" { Msg::AddNewTodo } else { Msg::Noop }, />
        }
    }
}

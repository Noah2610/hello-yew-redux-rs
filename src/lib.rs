extern crate yew;

mod yew_prelude {
    pub use yew::{
        html,
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

use yew::services::ConsoleService;

use components::prelude::*;
pub use entry_data::EntryData;
use yew_prelude::*;

/// Debug function; print to JS console
pub fn consolelog<T: ToString>(msg: T) {
    yew::services::ConsoleService::new().log(&msg.to_string())
}

pub enum Msg {
    UpdateInputNewTodo(String),
    NewTodo,
    Noop,
}

/// Root component
#[derive(Default)]
pub struct Model {
    console:        ConsoleService,
    entries:        Vec<EntryData>,
    input_new_todo: String,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            entries: Vec::new(),
            ..Default::default()
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::UpdateInputNewTodo(input) => {
                let changed = self.input_new_todo != input;
                self.input_new_todo = input;
                changed
            }
            Msg::NewTodo => {
                self.entries.push(EntryData {
                    id: self.entries.len(),
                    name: self.input_new_todo.clone(),
                    ..Default::default()
                });
                true
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
                <EntryList: entries=&self.entries, />
            </div>
        }
    }
}

impl Model {
    fn view_create_entry(&self) -> Html<Self> {
        html! {
            <input
                type="text",
                class="new-todo",
                value=&self.input_new_todo,
                oninput=|e| Msg::UpdateInputNewTodo(e.value),
                onkeypress=|e| if e.key() == "Enter" { Msg::NewTodo } else { Msg::Noop }, />
        }
    }
}

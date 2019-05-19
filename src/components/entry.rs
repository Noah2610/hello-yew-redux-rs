//! A single Todo entry item

use crate::yew_prelude::*;
use crate::EntryData;

#[derive(Default)]
pub struct Entry {
    id:        usize,
    name:      String,
    completed: bool,
}

pub enum Msg {
    Toggle,
}

#[derive(Default, Clone, PartialEq)]
pub struct Props {
    pub entry: EntryData,
}

impl Component for Entry {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            id:        props.entry.id,
            name:      props.entry.name,
            completed: props.entry.completed,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.id = props.entry.id;
        self.name = props.entry.name;
        self.completed = props.entry.completed;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                // TODO
                self.completed = !self.completed;
                true
            }
        }
    }
}

impl Renderable<Self> for Entry {
    fn view(&self) -> Html<Self> {
        let mut class = "todo".to_string();
        if self.completed {
            class.push_str(" completed");
        }
        html! {
            <li class=class,>
                <input class="toggle", type="checkbox", />
                <label>{ &self.name }</label>
                <button class="destroy", checked=self.completed, onclick=|_| Msg::Toggle, />
            </li>
        }
    }
}

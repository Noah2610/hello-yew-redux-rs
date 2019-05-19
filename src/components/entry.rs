//! A single Todo entry item

use crate::entry_data::prelude::*;
use crate::yew_prelude::*;

#[derive(Default)]
pub struct Entry {
    data:      EntryData,
    on_update: Option<Callback<EntryData>>,
}

pub enum Msg {
    Toggle,
}

#[derive(Default, Clone, PartialEq)]
pub struct Props {
    pub data:      EntryData,
    pub on_update: Option<Callback<EntryData>>,
}

impl Component for Entry {
    type Message = Msg;
    type Properties = Props;

    fn create(
        Props { data, on_update }: Self::Properties,
        _: ComponentLink<Self>,
    ) -> Self {
        Self { data, on_update }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.data != props.data;
        if changed {
            self.data = props.data;
        }
        changed
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                self.data.completed = !self.data.completed;
                self.on_update
                    .as_ref()
                    .map(|callback| callback.emit(self.data.clone()));
                true
            }
        }
    }
}

impl Renderable<Self> for Entry {
    fn view(&self) -> Html<Self> {
        let mut class = "todo".to_string();
        if self.data.completed {
            class.push_str(" completed");
        }
        html! {
            <li class=class,>
                <input class="toggle", type="checkbox", />
                <label>{ &self.data.name }</label>
                <button class="destroy", checked=self.data.completed, onclick=|_| Msg::Toggle, />
            </li>
        }
    }
}

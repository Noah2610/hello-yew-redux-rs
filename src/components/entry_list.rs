//! A list of Todo entry items

use crate::yew_prelude::*;

use super::prelude::{Entry, EntryProps};
use crate::EntryData;

#[derive(Default)]
pub struct EntryList {
    entries: Vec<EntryData>,
}

pub enum Msg {}

#[derive(Default, Clone, PartialEq)]
pub struct Props {
    pub entries: Vec<EntryData>,
}

impl Component for EntryList {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {
            entries: props.entries,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.entries != props.entries;
        self.entries = props.entries;
        changed
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {}
    }
}

impl Renderable<Self> for EntryList {
    fn view(&self) -> Html<Self> {
        html! {
            <ul class="todo-list",>
                { for self.entries
                        .iter()
                        .map(|e| self.view_entry(e)) }
            </ul>
        }
    }
}

impl EntryList {
    fn view_entry(&self, entry: &EntryData) -> Html<Self> {
        html! {
            <Entry: entry=entry, />
        }
    }
}

//! A list of Todo entry items

use crate::yew_prelude::*;

use super::prelude::{Entry, EntryProps};
use crate::entry_data::prelude::*;

pub struct EntryList {
    entries:         Vec<EntryData>,
    on_entry_toggle: Option<Callback<EntryId>>,
}

pub enum Msg {}

#[derive(Default, Clone, PartialEq)]
pub struct Props {
    pub entries:         Vec<EntryData>,
    pub on_entry_toggle: Option<Callback<EntryId>>,
}

impl Component for EntryList {
    type Message = Msg;
    type Properties = Props;

    fn create(
        Props {
            entries,
            on_entry_toggle,
        }: Self::Properties,
        _: ComponentLink<Self>,
    ) -> Self {
        Self {
            entries,
            on_entry_toggle,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        let changed = self.entries != props.entries;
        if changed {
            self.entries = props.entries;
        }
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
            <Entry: data=entry, on_toggle=self.on_entry_toggle.clone(), />
        }
    }
}

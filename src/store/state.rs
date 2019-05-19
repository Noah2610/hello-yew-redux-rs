//! Redux store state data structure

pub mod prelude {
    pub use super::State;
    pub use super::StateBuilder;
}

// use std::iter::IntoIterator;

use crate::entry_data::prelude::*;

#[derive(Default, Clone)]
pub struct State {
    pub entries: Vec<EntryData>,
}

impl State {
    pub fn builder() -> StateBuilder {
        StateBuilder::default()
    }
}

impl From<&Self> for State {
    fn from(state_ref: &Self) -> Self {
        state_ref.clone()
    }
}

impl From<StateBuilder> for State {
    fn from(builder: StateBuilder) -> Self {
        Self {
            entries: builder.entries,
        }
    }
}

/// A builder struct for `State`
#[derive(Default)]
pub struct StateBuilder {
    entries: Vec<EntryData>,
}

impl StateBuilder {
    pub fn add_entry(mut self, entry: EntryData) -> Self {
        self.entries.push(entry);
        self
    }

    pub fn add_entries<T>(mut self, entries: T) -> Self
    where
        T: IntoIterator<Item = EntryData>,
    {
        for entry in entries {
            self.entries.push(entry);
        }
        self
    }

    pub fn build(self) -> State {
        self.into()
    }
}

impl From<&State> for StateBuilder {
    fn from(state_ref: &State) -> Self {
        Self {
            entries: state_ref.entries.clone(),
        }
    }
}

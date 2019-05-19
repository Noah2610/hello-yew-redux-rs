//! Redux stuff

pub mod prelude {
    pub use super::state::prelude::*;
    pub use super::Action;
    pub use super::StoreHandle;
}

pub mod state;

use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use redux_rs::Store;

use crate::entry_data::prelude::*;
use state::prelude::*;

pub struct StoreHandle {
    pub store:  Arc<Mutex<Store<State, Action>>>,
    pub tx:     Sender<Action>,
    pub rx:     Arc<Mutex<Receiver<Action>>>,
    pub thread: JoinHandle<()>,
}

/// Redux actions, carrying payloads for reducer logic
pub enum Action {
    /// Add a new entry, with only the name given
    AddEntryWithName(String),
    Noop, // TODO
}

/// `initialize_store` creates a new thread, in which it sets up the redux store, etc.
/// and returns a `sync::mpsc::Sender` from the thread.
pub fn initialize_store() -> StoreHandle {
    let store = Arc::new(Mutex::new(Store::new(reducer, State::default())));
    let (tx, rx) = channel::<Action>();
    let rx = Arc::new(Mutex::new(rx));
    let thread = spawn_thread(Arc::clone(&rx), Arc::clone(&store));
    StoreHandle {
        store,
        tx,
        rx,
        thread,
    }
}

fn spawn_thread(
    rx: Arc<Mutex<Receiver<Action>>>,
    store: Arc<Mutex<Store<State, Action>>>,
) -> JoinHandle<()> {
    thread::spawn(move || match rx.lock().unwrap().recv() {
        Ok(action) => (*store).lock().unwrap().dispatch(action),
        Err(err) => panic!(format!(
            "Store thread got error from `rx.recv()`: {:?}",
            err
        )),
    })
}

fn reducer(state: &State, action: &Action) -> State {
    match action {
        Action::AddEntryWithName(name) => StateBuilder::from(state)
            .add_entry(EntryData {
                id: state.entries.len(),
                name: name.clone(),
                ..Default::default()
            })
            .build(),
        Action::Noop => state.clone(),
    }
}

//! Redux stuff

use std::sync::mpsc::{channel, Receiver, RecvError, Sender};
use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

use redux_rs::Store;

pub mod prelude {
    pub use super::State;
    pub use super::StoreAction;
    pub use super::StoreHandle;
}

#[derive(Default, Clone)]
pub struct State;

pub struct StoreHandle {
    pub store:  Arc<Mutex<Store<State, StoreAction>>>, // TODO
    pub tx:     Sender<StoreAction>,
    pub rx:     Arc<Mutex<Receiver<StoreAction>>>,
    pub thread: JoinHandle<()>,
}

/// Redux actions, carrying payloads for reducer logic
pub enum StoreAction {
    Noop, // TODO
}

/// `initialize_store` creates a new thread, in which it sets up the redux store, etc.
/// and returns a `sync::mpsc::Sender` from the thread.
pub fn initialize_store() -> StoreHandle {
    let store = Arc::new(Mutex::new(Store::new(reducer, State::default())));
    let (tx, rx) = channel::<StoreAction>();
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
    rx: Arc<Mutex<Receiver<StoreAction>>>,
    store: Arc<Mutex<Store<State, StoreAction>>>,
) -> JoinHandle<()> {
    thread::spawn(move || match rx.lock().unwrap().recv() {
        Ok(action) => (*store).lock().unwrap().dispatch(action),
        Err(err) => panic!(format!(
            "Store thread got error from `rx.recv()`: {:?}",
            err
        )),
    })
}

fn reducer(state: &State, action: &StoreAction) -> State {
    match action {
        StoreAction::Noop => state.clone(),
        _ => state.clone(),
    }
}

pub mod prelude {
    pub use super::entry::Entry;
    pub use super::entry::Msg as EntryMsg;
    pub use super::entry::Props as EntryProps;
    pub use super::entry_list::EntryList;
    pub use super::entry_list::Msg as EntryListMsg;
    pub use super::entry_list::Props as EntryListProps;
}

mod entry;
mod entry_list;

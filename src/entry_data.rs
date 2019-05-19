pub mod prelude {
    pub use super::EntryData;
    pub use super::EntryId;
}

pub type EntryId = usize;

#[derive(Default, Clone, PartialEq)]
pub struct EntryData {
    pub id:        EntryId,
    pub name:      String,
    pub completed: bool,
}

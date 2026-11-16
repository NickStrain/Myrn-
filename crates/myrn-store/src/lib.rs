//! Storage for Myrn: the SQLite file (code graph + index) and the Markdown notes.
//!
//! Notes are the source of truth for earned knowledge; the database is always
//! rebuildable from the code and the notes. Write order: note first, index second.
//!
//! Design doc: "Storage layout", "Code graph and index", "Notes".

pub mod db;
pub mod episodes;
pub mod fts;
pub mod graph;
pub mod notes;
pub mod paths;
pub mod state;
pub mod sync;
pub mod vector;

pub use db::Db;
pub use graph::{Edge, Node};
pub use paths::Layout;
pub use state::State;

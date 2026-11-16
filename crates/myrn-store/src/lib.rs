//! Storage for Myrn: the SQLite file (code graph + index) and the Markdown notes.
//!
//! Notes are the source of truth for earned knowledge; the database is always
//! rebuildable from the code and the notes. Write order: note first, index second.
//!
//! Design doc: "Storage layout", "Code graph and index", "Notes".

pub mod paths;
pub mod db;
pub mod graph;
pub mod fts;
pub mod vector;
pub mod notes;
pub mod sync;
pub mod episodes;

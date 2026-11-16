//! Domain model for the Myrn knowledge graph. Pure types, no I/O.
//!
//! Design doc: "Knowledge graph model".

pub mod event;
pub mod id;
pub mod node;
pub mod record;
pub mod relation;

pub use node::{NodeType, Origin};
pub use record::{EdgeStatus, Status, Visibility};
pub use relation::Relation;

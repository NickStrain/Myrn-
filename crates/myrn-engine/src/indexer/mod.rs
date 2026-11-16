//! Indexer: code and Git history into derived nodes. No model involved.
//! Re-parses a file only when its content hash changes.

pub mod walk;
pub mod parse;
pub mod symbols;
pub mod body_hash;
pub mod git;
pub mod lang;
pub mod scip;

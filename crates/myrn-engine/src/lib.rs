//! The Myrn core engine. Every read and write from adapters, the MCP server and
//! the CLI goes through [`api`]; nothing else touches the stores.
//!
//! Design doc: "System architecture".

pub mod api;
pub mod config;

pub mod capture;
pub mod distill;
pub mod embeddings;
pub mod indexer;
pub mod knowledge;
pub mod learn;
pub mod pack;
pub mod retrieve;
pub mod review;
pub mod summarize;
pub mod validity;

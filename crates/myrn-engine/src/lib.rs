//! The Myrn core engine. Every read and write from adapters, the MCP server and
//! the CLI goes through [`api`]; nothing else touches the stores.
//!
//! Design doc: "System architecture".

pub mod api;
pub mod config;

pub mod indexer;
pub mod capture;
pub mod distill;
pub mod validity;
pub mod retrieve;
pub mod pack;

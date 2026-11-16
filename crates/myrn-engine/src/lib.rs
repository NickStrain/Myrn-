//! The Myrn core engine. Every read and write from adapters, the MCP server and
//! the CLI goes through [`api`]; nothing else touches the stores.
//!
//! Design doc: "System architecture".

pub mod api;
pub mod config;

pub mod capture;
pub mod distill;
pub mod indexer;
pub mod pack;
pub mod retrieve;
pub mod validity;

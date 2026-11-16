//! Optional embedding helper. The only long-lived process; off the hot path.
//! The model is downloaded only on explicit request. Nothing depends on it.

pub mod helper;
pub mod client;

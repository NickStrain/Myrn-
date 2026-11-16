//! Distillation: the agent writes records through `record`; the engine checks them.

pub mod checks;
pub mod dedupe;
pub mod contradiction;
pub mod mechanical;

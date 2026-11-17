//! Model-free episode summary when no agent distillation happens.
//!
//! Every recorded session already keeps a mechanical record without any model:
//! files read and edited, symbols touched, commands, errors, test outcome and
//! commits (`capture::episode::Episode::summary`). It is stored whether or not
//! the agent calls `record` at session end, so nothing is lost when it doesn't.

pub use crate::capture::episode::Episode;

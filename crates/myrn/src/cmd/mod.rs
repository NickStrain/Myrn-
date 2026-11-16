//! One module per subcommand.

pub mod init;
pub mod connect;
pub mod status;
pub mod log;
pub mod show;
pub mod diff;
pub mod trace;
pub mod verify;
pub mod forget;
pub mod gc;
pub mod rebuild;
pub mod publish;
pub mod doctor;

// Internal entry points, not for direct use.
pub mod hook;
pub mod mcp;

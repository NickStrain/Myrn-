//! Normalized agent events. Every adapter translates its agent's hook
//! payloads into these, so the engine never sees agent-specific formats.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub session_id: String,
    /// Working directory reported by the agent (absolute).
    pub cwd: Option<String>,
    pub kind: EventKind,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "event", rename_all = "snake_case")]
pub enum EventKind {
    /// `source`: startup, resume, clear, compact, fork.
    SessionStart {
        source: Option<String>,
    },
    PromptSubmitted {
        prompt: String,
    },
    ToolResult(ToolResult),
    /// End of a session, or of a turn when the agent stops (`reason` says which).
    SessionEnd {
        reason: Option<String>,
        /// The agent is already continuing because a stop hook asked it to
        /// (`stop_hook_active`); never ask again, or it would loop.
        #[serde(default)]
        continued: bool,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ToolOutcome {
    Failure,
    FileEdit,
    Other,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ToolResult {
    pub tool: String,
    pub outcome: ToolOutcome,
    /// Files the tool edited or read, as absolute or agent-reported paths.
    pub files: Vec<String>,
    /// Shell command, for shell tools.
    pub command: Option<String>,
    /// Error text, for failures.
    pub error: Option<String>,
}

impl EventKind {
    pub fn name(&self) -> &'static str {
        match self {
            EventKind::SessionStart { .. } => "session_start",
            EventKind::PromptSubmitted { .. } => "prompt_submitted",
            EventKind::ToolResult(_) => "tool_result",
            EventKind::SessionEnd { .. } => "session_end",
        }
    }
}

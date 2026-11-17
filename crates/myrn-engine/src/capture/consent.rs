//! Consent state for session recording. Given once in `myrn connect`, stored in
//! the private `state.json`; nothing from a session is written without it.

use myrn_store::State;

/// May this agent's session be recorded?
pub fn may_record(state: &State, agent: &str) -> bool {
    state.capture_consent && state.agents.get(agent).is_some_and(|a| a.mode == "hooks")
}

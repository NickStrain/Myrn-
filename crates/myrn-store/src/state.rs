//! Private per-project state that must survive `myrn rebuild`: consent and
//! connected agents. Lives next to memory.db as `state.json`, never in the repo.

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct State {
    /// The developer agreed to session recording (`myrn connect`).
    #[serde(default)]
    pub capture_consent: bool,
    /// Unix time consent was given.
    #[serde(default)]
    pub consent_time: Option<i64>,
    /// Connected agents by adapter name.
    #[serde(default)]
    pub agents: BTreeMap<String, AgentState>,
    /// Recent hook timings, newest last (at most 50).
    #[serde(default)]
    pub hook_timings: Vec<HookTiming>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentState {
    /// `hooks` or `pull_only`.
    pub mode: String,
    pub connected_time: i64,
    /// Files myrn wrote or changed for this agent.
    pub written: Vec<String>,
    /// Instruction files myrn created (rather than edited), removed on disconnect if left empty.
    #[serde(default)]
    pub created: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookTiming {
    pub agent: String,
    pub event: String,
    pub millis: u64,
    pub time: i64,
}

impl State {
    pub fn load(path: &Path) -> Result<State> {
        match std::fs::read_to_string(path) {
            Ok(s) => {
                serde_json::from_str(&s).with_context(|| format!("reading {}", path.display()))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(State::default()),
            Err(e) => Err(e.into()),
        }
    }

    /// Write atomically (agents may run several hooks at once).
    pub fn save(&self, path: &Path) -> Result<()> {
        if let Some(dir) = path.parent() {
            std::fs::create_dir_all(dir)?;
        }
        let tmp = path.with_extension(format!("json.{}.tmp", std::process::id()));
        std::fs::write(&tmp, serde_json::to_string_pretty(self)?)?;
        std::fs::rename(&tmp, path)?;
        Ok(())
    }

    pub fn record_timing(&mut self, t: HookTiming) {
        self.hook_timings.push(t);
        let excess = self.hook_timings.len().saturating_sub(50);
        self.hook_timings.drain(..excess);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_missing_file() {
        let dir = std::env::temp_dir().join(format!("myrn-state-{}", std::process::id()));
        let p = dir.join("state.json");
        assert!(!State::load(&p).unwrap().capture_consent);
        let mut s = State {
            capture_consent: true,
            ..Default::default()
        };
        for i in 0..60 {
            s.record_timing(HookTiming {
                agent: "a".into(),
                event: "e".into(),
                millis: i,
                time: 0,
            });
        }
        s.save(&p).unwrap();
        let back = State::load(&p).unwrap();
        assert!(back.capture_consent);
        assert_eq!(back.hook_timings.len(), 50);
        assert_eq!(back.hook_timings[0].millis, 10);
        std::fs::remove_dir_all(dir).unwrap();
    }
}

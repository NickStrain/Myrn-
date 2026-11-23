//! The learning loop: what was injected, whether the agent then used it, and
//! how the session ended. Ranking uses it to move items that keep helping up
//! and items that keep being ignored down.
//!
//! "Used" means: after the injection, in the same session, the agent read or
//! edited the file of the injected code (or of the code a record is anchored
//! to), asked `explain`/`impact`/`recall` about it, or cited the record's id
//! (`record` relations, `confirm`, `supersede`).

use std::collections::HashMap;

use anyhow::Result;
use rusqlite::{params, Connection};
use serde::Serialize;

use crate::api::Engine;
use crate::config::RankingConfig;

/// Use counted within this many seconds of an injection when the session is
/// not known (MCP tool calls do not carry the session id).
const RECENT_SECS: i64 = 2 * 3600;
/// Beta prior on "an injected item gets used": mean 1/3, weight of three injections.
const PRIOR_USED: f64 = 1.0;
const PRIOR_UNUSED: f64 = 2.0;

/// Past use of one item, from sessions that have ended (an outcome is
/// attached at the end of each turn); a session still running is not judged yet.
#[derive(Debug, Clone, Default, Serialize, PartialEq)]
pub struct Usage {
    pub injected: i64,
    pub used: i64,
    /// Used in a session that ended well.
    pub used_good: i64,
}

impl Usage {
    /// Ranking factor in 0.5..1.5: the posterior chance of being used,
    /// relative to the prior, raised to `weight`. Use in good sessions
    /// counts one and a half. No history: 1.
    pub fn factor(&self, weight: f64) -> f64 {
        if self.injected == 0 || weight <= 0.0 {
            return 1.0;
        }
        let used = self.used as f64 + 0.5 * self.used_good as f64;
        let n = self.injected as f64 + 0.5 * self.used_good as f64;
        let mean = (used + PRIOR_USED) / (n + PRIOR_USED + PRIOR_UNUSED);
        let prior = PRIOR_USED / (PRIOR_USED + PRIOR_UNUSED);
        (mean / prior).powf(weight).clamp(0.5, 1.5)
    }
}

pub fn usage_of(conn: &Connection, ids: &[&str]) -> Result<HashMap<String, Usage>> {
    let mut stmt = conn.prepare_cached(
        "SELECT COUNT(*), coalesce(SUM(used), 0),
                coalesce(SUM(CASE WHEN used = 1 AND outcome IN ('tests_passed', 'committed') THEN 1 ELSE 0 END), 0)
         FROM usage WHERE note_id = ?1 AND injected = 1 AND outcome IS NOT NULL",
    )?;
    let mut out = HashMap::new();
    for id in ids {
        let u = stmt.query_row([id], |r| {
            Ok(Usage {
                injected: r.get(0)?,
                used: r.get(1)?,
                used_good: r.get(2)?,
            })
        })?;
        if u.injected > 0 {
            out.insert(id.to_string(), u);
        }
    }
    Ok(out)
}

/// Ranking factor for a record's age (note file time): 1 when new, down to
/// `1 - weight` when very old.
pub fn freshness(cfg: &RankingConfig, age_days: f64) -> f64 {
    if cfg.freshness_weight <= 0.0 || cfg.freshness_half_life_days <= 0.0 {
        return 1.0;
    }
    let decay = 0.5f64.powf(age_days.max(0.0) / cfg.freshness_half_life_days);
    1.0 - cfg.freshness_weight * (1.0 - decay)
}

impl Engine {
    /// The agent read or edited these files (repo-relative): injected items
    /// about them in this session were used.
    pub fn mark_used_files(&self, session: &str, files: &[String]) -> Result<usize> {
        let c = &self.db.conn;
        let mut n = 0;
        for f in files {
            n += c.execute(
                "UPDATE usage SET used = 1
                 WHERE session_id = ?1 AND injected = 1 AND used = 0 AND (
                   note_id IN (SELECT id FROM nodes WHERE path = ?2)
                   OR note_id IN (SELECT a.note_id FROM anchors a JOIN nodes s ON s.id = a.symbol_id
                                  WHERE s.path = ?2))",
                params![session, f],
            )?;
        }
        Ok(n)
    }

    /// The agent asked about or cited these items (by node id): recent
    /// injections of them were used.
    pub fn mark_used_ids(&self, ids: &[String]) -> Result<usize> {
        let c = &self.db.conn;
        let mut n = 0;
        for id in ids {
            n += c.execute(
                "UPDATE usage SET used = 1
                 WHERE note_id = ?1 AND injected = 1 AND used = 0
                   AND coalesce(t, 0) >= unixepoch() - ?2",
                params![id, RECENT_SECS],
            )?;
        }
        Ok(n)
    }

    /// Attach the session's outcome to everything injected in it.
    pub fn attach_outcome(&self, session: &str) -> Result<()> {
        let Some((ep, _)) = self.episode_log(session)? else {
            return Ok(());
        };
        self.db.conn.execute(
            "UPDATE usage SET outcome = ?2 WHERE session_id = ?1",
            params![session, ep.outcome()],
        )?;
        Ok(())
    }

    /// Past use of items, for `myrn status` and tests.
    pub fn usage(&self, ids: &[&str]) -> Result<HashMap<String, Usage>> {
        usage_of(&self.db.conn, ids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn usefulness_moves_with_evidence_and_stays_bounded() {
        let none = Usage::default();
        assert_eq!(none.factor(0.5), 1.0);
        let ignored = Usage {
            injected: 6,
            used: 0,
            used_good: 0,
        };
        let helpful = Usage {
            injected: 6,
            used: 5,
            used_good: 4,
        };
        let once = Usage {
            injected: 1,
            used: 0,
            used_good: 0,
        };
        assert!(ignored.factor(0.5) < once.factor(0.5));
        assert!(once.factor(0.5) < 1.0);
        assert!(helpful.factor(0.5) > 1.0);
        assert!(ignored.factor(5.0) >= 0.5 && helpful.factor(5.0) <= 1.5);
        assert_eq!(helpful.factor(0.0), 1.0);
    }

    #[test]
    fn old_records_rank_down_a_little() {
        let cfg = RankingConfig::default();
        assert_eq!(freshness(&cfg, 0.0), 1.0);
        assert!((freshness(&cfg, 180.0) - 0.9).abs() < 1e-9);
        assert!(freshness(&cfg, 10_000.0) > 0.79);
    }
}

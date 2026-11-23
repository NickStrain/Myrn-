//! Log what was injected, so later usage and outcomes can be attributed to it.

use anyhow::Result;
use rusqlite::{params, Connection};

pub fn record(conn: &Connection, session: &str, ids: &[String]) -> Result<()> {
    let mut stmt = conn.prepare_cached(
        "INSERT INTO usage (note_id, session_id, injected, used, status, t)
         VALUES (?1, ?2, 1, 0, (SELECT status FROM nodes WHERE id = ?1), unixepoch())",
    )?;
    for id in ids {
        stmt.execute(params![id, session])?;
    }
    Ok(())
}

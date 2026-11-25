//! Opening `memory.db` and running migrations.

use std::path::Path;

use anyhow::{bail, Context, Result};
use rusqlite::{Connection, OptionalExtension};

const MIGRATIONS: &[&str] = &[
    include_str!("../migrations/0001_init.sql"),
    include_str!("../migrations/0002_notes.sql"),
    include_str!("../migrations/0003_summary_search.sql"),
    include_str!("../migrations/0004_usage_status.sql"),
    include_str!("../migrations/0005_learning.sql"),
    include_str!("../migrations/0006_sharing.sql"),
    include_str!("../migrations/0007_summary_details.sql"),
    include_str!("../migrations/0008_commit_adds.sql"),
];

/// Schema version this binary writes.
pub const SCHEMA_VERSION: i64 = MIGRATIONS.len() as i64;

pub struct Db {
    pub conn: Connection,
}

impl Db {
    pub fn open(path: &Path) -> Result<Db> {
        let conn = Connection::open(path).with_context(|| format!("opening {}", path.display()))?;
        Self::setup(conn)
    }

    pub fn open_in_memory() -> Result<Db> {
        Self::setup(Connection::open_in_memory()?)
    }

    fn setup(conn: Connection) -> Result<Db> {
        conn.busy_timeout(std::time::Duration::from_secs(5))?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        conn.pragma_update(None, "synchronous", "NORMAL")?;
        let db = Db { conn };
        db.migrate()?;
        Ok(db)
    }

    pub fn schema_version(&self) -> Result<i64> {
        Ok(self
            .conn
            .query_row("PRAGMA user_version", [], |r| r.get(0))?)
    }

    fn migrate(&self) -> Result<()> {
        let current = self.schema_version()?;
        if current > SCHEMA_VERSION {
            bail!(
                "memory.db has schema version {current}, newer than this myrn ({SCHEMA_VERSION}); \
                 upgrade myrn or run `myrn rebuild`"
            );
        }
        for (i, sql) in MIGRATIONS.iter().enumerate().skip(current as usize) {
            self.conn.execute_batch(&format!(
                "BEGIN; {sql}\nPRAGMA user_version = {}; COMMIT;",
                i + 1
            ))?;
        }
        Ok(())
    }

    pub fn get_meta(&self, key: &str) -> Result<Option<String>> {
        Ok(self
            .conn
            .query_row("SELECT value FROM meta WHERE key = ?1", [key], |r| r.get(0))
            .optional()?)
    }

    pub fn set_meta(&self, key: &str, value: &str) -> Result<()> {
        self.conn.execute(
            "INSERT INTO meta(key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            [key, value],
        )?;
        Ok(())
    }

    pub fn integrity_ok(&self) -> Result<bool> {
        let s: String = self
            .conn
            .query_row("PRAGMA quick_check", [], |r| r.get(0))?;
        Ok(s == "ok")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrates_and_stores_meta() {
        let db = Db::open_in_memory().unwrap();
        assert_eq!(db.schema_version().unwrap(), SCHEMA_VERSION);
        db.set_meta("k", "v1").unwrap();
        db.set_meta("k", "v2").unwrap();
        assert_eq!(db.get_meta("k").unwrap().as_deref(), Some("v2"));
        assert!(db.integrity_ok().unwrap());
    }
}

//! Semantic search, when turned on with `myrn embeddings enable`: every code
//! symbol, file and learned record gets a vector of what it is and does (name,
//! signature, summary); a task's vector finds them by meaning, as one more
//! kind of seed next to names and full-text matches. Off, or with the model
//! missing, nothing here runs and retrieval is unchanged.

use std::path::PathBuf;

use anyhow::{bail, Result};
use myrn_store::fts;
use serde::Serialize;

use crate::api::Engine;
use crate::retrieve::seed::Seed;

/// Most texts per embedding request.
const BATCH: usize = 256;
const MAX_TEXT: usize = 1500;

#[derive(Debug, Clone, Default, Serialize)]
pub struct VectorReport {
    pub embedded: usize,
    pub removed: usize,
    pub total: usize,
    pub millis: u128,
}

#[derive(Debug, Clone, Serialize)]
pub struct EmbeddingsStatus {
    pub supported: bool,
    pub enabled: bool,
    pub installed: bool,
    pub model: String,
    pub model_dir: PathBuf,
    pub vectors: i64,
    pub helper_running: bool,
}

impl Engine {
    /// `<data home>`, the parent of every project's data directory.
    pub fn data_home(&self) -> PathBuf {
        self.layout
            .data_dir
            .parent()
            .and_then(|p| p.parent())
            .map(PathBuf::from)
            .unwrap_or_else(|| self.layout.data_dir.clone())
    }

    /// Embeddings are on for this project and the model is here.
    pub fn embeddings_on(&self) -> bool {
        myrn_embed::supported()
            && myrn_embed::installed(&self.data_home())
            && self.state().map(|s| s.embeddings).unwrap_or(false)
    }

    /// The `myrn` binary, to start the helper with; `None` inside other
    /// programs (tests, the benchmark), which then load the model themselves.
    fn helper_exe() -> Option<PathBuf> {
        let exe = std::env::current_exe().ok()?;
        let stem = exe.file_stem()?.to_str()?;
        (stem == "myrn").then_some(exe)
    }

    pub fn embed(&self, texts: &[String]) -> Option<Vec<Vec<f32>>> {
        myrn_embed::client::embed(&self.data_home(), texts, Self::helper_exe().as_deref())
    }

    /// Embed what is new or changed, drop vectors of nodes that are gone.
    pub fn update_vectors(&mut self) -> Result<VectorReport> {
        let started = std::time::Instant::now();
        let mut r = VectorReport::default();
        let c = &self.db.conn;
        let removed = c.execute(
            "DELETE FROM vectors WHERE id NOT IN (SELECT id FROM nodes)",
            [],
        )?;
        r.removed = removed;
        // The map's Behaviour and Terms lines: how people describe the code,
        // which is what a task repeats (the chosen model ranks best with them).
        let mut details: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        {
            let mut d = c.prepare("SELECT id, behaviour, terms FROM summary_search")?;
            let rows = d.query_map([], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, Option<String>>(1)?.unwrap_or_default(),
                    r.get::<_, Option<String>>(2)?.unwrap_or_default(),
                ))
            })?;
            for row in rows {
                let (id, b, t) = row?;
                let text = format!("{} {}", b.replace('\n', " "), t);
                if !text.trim().is_empty() {
                    details.insert(id, text.trim().to_string());
                }
            }
        }
        let mut stmt = c.prepare(
            "SELECT n.id, n.type, n.kind, n.title, n.qname, n.path, n.signature, n.summary, n.doc, v.hash
             FROM nodes n LEFT JOIN vectors v ON v.id = n.id
             WHERE (n.type = 'symbol' AND n.kind NOT IN ('const', 'static'))
                OR n.type = 'file'
                OR (n.origin = 'earned' AND n.type != 'episode')",
        )?;
        let rows = stmt.query_map([], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, Option<String>>(3)?,
                row.get::<_, Option<String>>(4)?,
                row.get::<_, Option<String>>(5)?,
                row.get::<_, Option<String>>(6)?,
                row.get::<_, Option<String>>(7)?,
                row.get::<_, Option<String>>(8)?,
                row.get::<_, Option<String>>(9)?,
            ))
        })?;
        let mut todo: Vec<(String, String, String)> = Vec::new(); // (id, hash, text)
        for row in rows {
            let (id, ntype, kind, title, qname, path, sig, summary, doc, have) = row?;
            r.total += 1;
            let words =
                |s: &Option<String>| fts::split_identifier(s.as_deref().unwrap_or("")).join(" ");
            let about = summary.or(doc).unwrap_or_default();
            let extra = details.get(&id).map(String::as_str).unwrap_or("");
            let text = match ntype.as_str() {
                "symbol" => format!(
                    "{} {}. {} {} {extra}",
                    kind.unwrap_or_default(),
                    words(&qname),
                    sig.unwrap_or_default(),
                    about
                ),
                "file" => format!("file {}. {about}", words(&path)),
                _ => format!("{ntype} {}. {about}", title.unwrap_or_default()),
            };
            let text: String = text.chars().take(MAX_TEXT).collect();
            let hash = blake3::hash(format!("{}\n{text}", myrn_embed::MODEL_REPO).as_bytes())
                .to_hex()[..16]
                .to_string();
            if have.as_deref() != Some(hash.as_str()) {
                todo.push((id, hash, text));
            }
        }
        drop(stmt);
        for chunk in todo.chunks(BATCH) {
            let texts: Vec<String> = chunk.iter().map(|(_, _, t)| t.clone()).collect();
            let Some(vecs) = self.embed(&texts) else {
                bail!("the embedding model is not available; run `myrn embeddings enable`");
            };
            let tx = self.db.conn.transaction()?;
            {
                let mut ins = tx.prepare(
                    "INSERT OR REPLACE INTO vectors (id, hash, model, vec) VALUES (?1, ?2, ?3, ?4)",
                )?;
                for ((id, hash, _), v) in chunk.iter().zip(vecs) {
                    ins.execute(rusqlite::params![
                        id,
                        hash,
                        myrn_embed::MODEL_REPO,
                        myrn_embed::to_bytes(&v)
                    ])?;
                }
            }
            tx.commit()?;
            r.embedded += chunk.len();
        }
        r.millis = started.elapsed().as_millis();
        Ok(r)
    }

    /// Nodes whose meaning is close to the task's, as seeds.
    pub fn vector_seeds(&self, task: &str) -> Result<Vec<Seed>> {
        if !self.embeddings_on() {
            return Ok(Vec::new());
        }
        let cfg = &self.config.embeddings;
        let query = format!("{}{task}", myrn_embed::QUERY_PREFIX);
        let Some(q) = self.embed(&[query]).and_then(|mut v| v.pop()) else {
            return Ok(Vec::new());
        };
        if q.iter().all(|x| *x == 0.0) {
            return Ok(Vec::new());
        }
        let mut stmt = self.db.conn.prepare_cached("SELECT id, vec FROM vectors")?;
        let mut hits: Vec<(String, f32)> = stmt
            .query_map([], |r| {
                Ok((r.get::<_, String>(0)?, r.get::<_, Vec<u8>>(1)?))
            })?
            .filter_map(|r| r.ok())
            .map(|(id, b)| {
                let v = myrn_embed::from_bytes(&b);
                (id, myrn_embed::dot(&q, &v))
            })
            .collect();
        hits.sort_by(|a, b| b.1.total_cmp(&a.1));
        let Some(background) = hits
            .get(19.min(hits.len().saturating_sub(1)))
            .map(|h| h.1 as f64)
        else {
            return Ok(Vec::new());
        };
        let floor = cfg.min_similarity.max(background + cfg.margin);
        hits.retain(|(_, s)| *s as f64 >= floor);
        hits.truncate(cfg.top_k);
        // Just above the floor: a weak seed (MIN_SCORE); clearly above: up to a
        // full-text match's strength. Names in the task still win.
        Ok(hits
            .into_iter()
            .map(|(id, s)| Seed {
                id,
                score: (3.0 + 20.0 * (s as f64 - floor)).min(6.0),
                why: "similar meaning",
            })
            .collect())
    }

    /// Download the model if needed, turn embeddings on for this project and
    /// embed everything.
    pub fn enable_embeddings(&mut self, progress: &dyn Fn(&str)) -> Result<VectorReport> {
        if !myrn_embed::supported() {
            bail!(
                "this build of myrn has no embedding support (built without the `model` feature)"
            );
        }
        let home = self.data_home();
        if !myrn_embed::installed(&home) {
            progress(&format!(
                "downloading {} ({}) into {}",
                myrn_embed::MODEL_REPO,
                myrn_embed::MODEL_SIZE,
                myrn_embed::model_dir(&home).display()
            ));
            download(&home, progress)?;
        }
        let mut state = self.state()?;
        state.embeddings = true;
        self.save_state(&state)?;
        progress("embedding the code and records");
        self.update_vectors()
    }

    pub fn disable_embeddings(&mut self) -> Result<()> {
        let mut state = self.state()?;
        state.embeddings = false;
        self.save_state(&state)?;
        self.db.conn.execute("DELETE FROM vectors", [])?;
        Ok(())
    }

    pub fn embeddings_status(&self) -> Result<EmbeddingsStatus> {
        let home = self.data_home();
        Ok(EmbeddingsStatus {
            supported: myrn_embed::supported(),
            enabled: self.state()?.embeddings,
            installed: myrn_embed::installed(&home),
            model: myrn_embed::MODEL_REPO.into(),
            model_dir: myrn_embed::model_dir(&home),
            vectors: self
                .db
                .conn
                .query_row("SELECT COUNT(*) FROM vectors", [], |r| r.get(0))?,
            helper_running: myrn_embed::client::request(&home, &[String::new()]).is_some(),
        })
    }
}

#[cfg(feature = "embeddings")]
fn download(home: &std::path::Path, progress: &dyn Fn(&str)) -> Result<()> {
    myrn_embed::model::download(&myrn_embed::model_dir(home), progress)
}

#[cfg(not(feature = "embeddings"))]
fn download(_home: &std::path::Path, _progress: &dyn Fn(&str)) -> Result<()> {
    bail!("this build of myrn has no embedding support")
}

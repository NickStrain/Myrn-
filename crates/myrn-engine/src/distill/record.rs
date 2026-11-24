//! `record(item)`: check a record, merge it with what is known, write it as a
//! private note, index it. Also the session-end request that asks the agent
//! to record what it learned.

use std::collections::HashSet;
use std::path::PathBuf;

use anyhow::Result;
use myrn_model::Relation;
use myrn_store::episodes::{self, Entry};
use myrn_store::notes::{self, frontmatter, layout, Anchor, Link, Note};
use myrn_store::{graph, paths, sync};

use super::checks::{self, Ref};
use super::{contradiction, dedupe, signature, RecordItem, RecordOutcome};
use crate::api::Engine;
use crate::capture::episode::clip;

/// Shortest body accepted: a record states something.
const MIN_BODY: usize = 12;

impl Engine {
    /// `record` from raw JSON: schema errors become rejection reasons.
    pub fn record_json(&mut self, v: &serde_json::Value) -> Result<RecordOutcome> {
        match serde_json::from_value::<RecordItem>(v.clone()) {
            Ok(item) => self.record(item),
            Err(e) => Ok(RecordOutcome::reject(vec![format!(
                "the record does not match the schema: {e}"
            )])),
        }
    }

    /// `record` with either one item or `{"items": [...]}`: one call can store
    /// everything a session learned, so the agent re-reads its context once,
    /// not once per item. Each item is checked on its own; rejected ones say why.
    pub fn record_any_json(&mut self, v: &serde_json::Value) -> Result<Vec<RecordOutcome>> {
        match v.get("items") {
            Some(serde_json::Value::Array(items)) => {
                if items.is_empty() {
                    return Ok(vec![RecordOutcome::reject(vec![
                        "`items` is empty; nothing to record".into(),
                    ])]);
                }
                let session = v.get("session").cloned();
                items
                    .iter()
                    .map(|item| {
                        let mut item = item.clone();
                        if let (Some(s), Some(o)) = (&session, item.as_object_mut()) {
                            o.entry("session").or_insert(s.clone());
                        }
                        self.record_json(&item)
                    })
                    .collect()
            }
            Some(_) => Ok(vec![RecordOutcome::reject(vec![
                "`items` must be a list of records".into(),
            ])]),
            None => Ok(vec![self.record_json(v)?]),
        }
    }

    /// Check, merge and store one record.
    pub fn record(&mut self, item: RecordItem) -> Result<RecordOutcome> {
        if let Some(p) = checks::check_type(&item.kind) {
            return Ok(RecordOutcome::reject(vec![p]));
        }
        let kind = item.kind.as_str();
        let mut problems = Vec::new();
        let body = item.body.trim();
        if body.chars().count() < MIN_BODY {
            problems.push("`body` must state what was learned, in at least a sentence".into());
        }

        let mut resolve_all = |names: &[String], field: &str, notes: bool| -> Result<Vec<Ref>> {
            let mut out = Vec::new();
            for n in names {
                match self.resolve_ref(n, notes)? {
                    Ok(r) => out.push(r),
                    Err(e) => problems.push(format!("{field}: {e}")),
                }
            }
            Ok(out)
        };
        let about = resolve_all(&item.about, "about", true)?;
        let located = resolve_all(&item.located_in, "located_in", false)?;
        let verified = resolve_all(&item.verified_by, "verified_by", false)?;
        let mut relations: Vec<(Relation, Ref)> = Vec::new();
        for r in &item.relations {
            if let Some(p) = checks::check_relation(&r.rel) {
                problems.push(p);
                continue;
            }
            match self.resolve_ref(&r.target, true)? {
                Ok(t) => relations.push((Relation::parse(&r.rel).expect("checked"), t)),
                Err(e) => problems.push(format!("relation {}: {e}", r.rel)),
            }
        }
        let fixed_in = match item.fixed_in.as_deref().filter(|s| !s.trim().is_empty()) {
            Some(sha) => match self.resolve_ref(sha, false)? {
                Ok(r) if r.ntype == "commit" => Some(r),
                _ => {
                    problems.push(format!(
                        "fixed_in: `{sha}` is not a commit in this repository's indexed history"
                    ));
                    None
                }
            },
            None => None,
        };
        for r in &verified {
            if !matches!(r.ntype.as_str(), "test" | "commit" | "file") {
                problems.push(format!(
                    "verified_by: `{}` is a {}; cite a test or a commit",
                    r.header, r.ntype
                ));
            }
        }

        // Fields that belong to one type only.
        let only = |field: &str, set: bool, types: &[&str], problems: &mut Vec<String>| {
            if set && !types.contains(&kind) {
                problems.push(format!(
                    "`{field}` is only for {} records",
                    types.join(" and ")
                ));
            }
        };
        only("error", item.error.is_some(), &["bug"], &mut problems);
        only(
            "root_cause",
            item.root_cause.is_some(),
            &["bug"],
            &mut problems,
        );
        only(
            "located_in",
            !item.located_in.is_empty(),
            &["bug"],
            &mut problems,
        );
        only("fix", item.fix.is_some(), &["bug"], &mut problems);
        only("fixed_in", item.fixed_in.is_some(), &["bug"], &mut problems);
        only(
            "steps",
            !item.steps.is_empty(),
            &["workflow"],
            &mut problems,
        );
        match kind {
            "bug" if item.root_cause.is_some() && located.is_empty() => problems
                .push("a bug with a `root_cause` needs `located_in`: the code responsible".into()),
            "workflow" if item.steps.is_empty() => {
                problems.push("a workflow needs `steps`, in order".into())
            }
            "summary" => {
                let code: Vec<&Ref> = about
                    .iter()
                    .filter(|r| matches!(r.ntype.as_str(), "symbol" | "test" | "file" | "module"))
                    .collect();
                if code.len() != 1 || about.len() != 1 {
                    problems.push(
                        "a summary is `about` exactly one symbol or file (one record per symbol)"
                            .into(),
                    );
                }
            }
            _ => {}
        }

        let (evidence, evidence_refs, evidence_problems) = self.check_evidence(&item.evidence)?;
        let mut evidence = evidence;
        if kind == "summary" && evidence.is_empty() {
            // The code it describes is its evidence.
            if let Some(r) = about.first() {
                evidence.push(myrn_store::notes::Evidence {
                    kind: "symbol".into(),
                    value: r.header.clone(),
                });
            }
        }
        if evidence.is_empty() {
            problems.push(
                "`evidence` must cite at least one thing that exists: {\"symbol\": name}, \
                 {\"commit\": sha}, {\"test\": test name or file} or {\"test_run\": command you ran}"
                    .into(),
            );
            problems.extend(evidence_problems.iter().cloned());
        }
        if !problems.is_empty() {
            return Ok(RecordOutcome::reject(problems));
        }
        let mut outcome = RecordOutcome {
            accepted: true,
            messages: evidence_problems
                .into_iter()
                .map(|p| format!("ignored: {p}"))
                .collect(),
            ..Default::default()
        };

        let source = self.current_session(item.session.as_deref());
        // Provenance, so a shared copy can say who wrote it and when.
        let written_by = self.session_agent(source.as_deref());
        let created = crate::sharing::today();
        let stamp = |n: &mut Note| {
            if let Some(a) = &written_by {
                n.set_extra("written_by", a);
            }
            n.set_extra("created", &created);
        };
        let anchors_of = |refs: &[&Ref]| -> Vec<Anchor> {
            let mut out: Vec<Anchor> = Vec::new();
            for a in refs.iter().filter_map(|r| r.anchor.clone()) {
                if !out.iter().any(|o| o.target == a.target) {
                    out.push(a);
                }
            }
            out
        };
        let mut fresh: HashSet<String> = HashSet::new();

        // Root cause first, so the bug can link to it.
        let mut cause_id = None;
        if let Some(cause) = item.root_cause.as_deref().filter(|c| !c.trim().is_empty()) {
            let mut rc = Note::new("root_cause", &format!("{}\n", cause.trim()));
            rc.source = source.clone();
            stamp(&mut rc);
            rc.evidence = evidence.clone();
            rc.links = located
                .iter()
                .map(|r| link(Relation::LocatedIn, &r.header))
                .collect();
            rc.anchors = anchors_of(&located.iter().collect::<Vec<_>>());
            let id = layout::make_id("root_cause", &rc.body, 6);
            let (id, updated) = match dedupe::find_id(&self.db.conn, &id)? {
                Some(mut ex) => {
                    dedupe::merge(&mut ex.note, &rc, false);
                    self.put_note(&mut ex.note, Some(&ex.key), &fresh)?;
                    (ex.note.id, true)
                }
                None => {
                    self.put_note(&mut rc, None, &fresh)?;
                    (rc.id, false)
                }
            };
            outcome.updated |= updated;
            fresh.insert(id.clone());
            cause_id = Some(id);
        }

        // The main note.
        let mut text = format!("{body}\n");
        if let Some(err) = item.error.as_deref().filter(|e| !e.trim().is_empty()) {
            let first = err
                .lines()
                .map(str::trim)
                .find(|l| !l.is_empty())
                .unwrap_or("");
            text.push_str(&format!("\nError: `{}`\n", clip(first, 200)));
        }
        if let Some(fix) = item.fix.as_deref().filter(|f| !f.trim().is_empty()) {
            text.push_str(&format!("\nFix: {}\n", fix.trim()));
        }
        if !item.steps.is_empty() {
            text.push('\n');
            for (i, s) in item.steps.iter().enumerate() {
                text.push_str(&format!("{}. {}\n", i + 1, s.trim()));
            }
        }
        let mut note = Note::new(kind, &text);
        note.title = item.title.clone().filter(|t| !t.trim().is_empty());
        note.source = source.clone();
        stamp(&mut note);
        note.evidence = evidence;
        note.signature = item
            .error
            .as_deref()
            .and_then(signature::normalize)
            .map(|s| s.text());
        for r in &about {
            note.links.push(link(Relation::About, &r.header));
        }
        for r in &verified {
            note.links.push(link(Relation::VerifiedBy, &r.header));
        }
        if let Some(c) = &cause_id {
            note.links.push(link(Relation::CausedBy, c));
        }
        if let Some(f) = &fixed_in {
            note.links.push(Link {
                rel: Relation::Fixes,
                target: f.header.clone(),
                inverse: true,
            });
        }
        for (rel, t) in &relations {
            note.links.push(link(*rel, &t.header));
        }
        let anchored: Vec<&Ref> = about
            .iter()
            .chain(&evidence_refs)
            .chain(if kind == "bug" {
                located.iter()
            } else {
                [].iter()
            })
            .collect();
        note.anchors = anchors_of(&anchored);

        // Contradictions with known facts and decisions about the same code.
        let new_id = layout::make_id(kind, &note.body, 6);
        if matches!(kind, "fact" | "decision") {
            let targets: Vec<String> = about.iter().map(|r| r.id.clone()).collect();
            for (id, title) in contradiction::find(&self.db.conn, &targets, body)? {
                if id == new_id || note.links.iter().any(|l| l.target == id) {
                    continue;
                }
                note.links.push(link(Relation::Contradicts, &id));
                outcome.messages.push(format!(
                    "conflicts with {id} (\"{title}\"): both are ranked down until one is superseded. \
                     If this replaces it, record again with relations [{{\"rel\": \"supersedes\", \"target\": \"{id}\"}}]"
                ));
            }
        }

        // Is this already known?
        let existing = match kind {
            "bug" => {
                let symbols: HashSet<String> = about
                    .iter()
                    .chain(&located)
                    .filter(|r| r.anchor.is_some())
                    .map(|r| r.id.clone())
                    .collect();
                let by_signature = match note.signature.as_deref().map(dedupe::parse_signature) {
                    Some(sig) => dedupe::find_bug(&self.db.conn, &sig, &symbols)?,
                    None => None,
                };
                match by_signature {
                    Some(e) => Some(e),
                    None => dedupe::find_id(&self.db.conn, &new_id)?,
                }
            }
            "summary" => dedupe::find_summary(&self.db.conn, &about[0].id)?,
            _ => dedupe::find_id(&self.db.conn, &new_id)?,
        };
        let id = match existing {
            Some(mut ex) => {
                dedupe::merge(&mut ex.note, &note, kind == "summary");
                self.put_note(&mut ex.note, Some(&ex.key), &fresh)?;
                outcome.updated = true;
                ex.note.id
            }
            None => {
                self.put_note(&mut note, None, &fresh)?;
                note.id
            }
        };
        outcome.ids.push(id.clone());
        if let Some(c) = cause_id {
            outcome.ids.push(c);
        }

        // A newer record replaces the ones it supersedes.
        for (rel, t) in &relations {
            if *rel != Relation::Supersedes {
                continue;
            }
            if let Some(mut old) = dedupe::find_id(&self.db.conn, &t.id)? {
                if old.note.status != "superseded" {
                    old.note.status = "superseded".into();
                    self.put_note(&mut old.note, Some(&old.key), &fresh)?;
                    outcome.messages.push(format!("{} is now superseded", t.id));
                }
            }
        }

        let roots = self.note_roots();
        sync::sync(&mut self.db.conn, &roots, false)?;
        Ok(outcome)
    }

    /// Write a note: back to its own file when it exists, else into the
    /// private notes. File first; the caller indexes afterwards.
    pub(crate) fn put_note(
        &self,
        note: &mut Note,
        key: Option<&str>,
        fresh: &HashSet<String>,
    ) -> Result<PathBuf> {
        let c = &self.db.conn;
        let is_note = |id: &str| {
            fresh.contains(id)
                || c.query_row("SELECT 1 FROM notes WHERE id = ?1", [id], |_| Ok(()))
                    .is_ok()
        };
        match key {
            Some(key) => {
                let path = self.note_file(key);
                let tmp = path.with_extension(format!("md.{}.tmp", std::process::id()));
                std::fs::write(&tmp, frontmatter::render(note, &is_note))?;
                std::fs::rename(&tmp, &path)?;
                Ok(path)
            }
            None => notes::write(&self.layout.notes_dir(), note, &is_note),
        }
    }

    /// The file behind a notes-table key.
    pub(crate) fn note_file(&self, key: &str) -> PathBuf {
        let shared = format!("{}/knowledge/", paths::REPO_DIR);
        match key.strip_prefix(&shared) {
            Some(rest) => self.layout.knowledge_dir().join(rest),
            None => self
                .layout
                .notes_dir()
                .join(key.strip_prefix("notes/").unwrap_or(key)),
        }
    }

    /// `episode:<session>` for the given session, or the most recently recorded one.
    fn current_session(&self, given: Option<&str>) -> Option<String> {
        let key = match given.filter(|s| !s.trim().is_empty()) {
            Some(s) => episodes::file_name(s)
                .trim_end_matches(".jsonl")
                .to_string(),
            None => {
                episodes::list(&self.layout.episodes_dir())
                    .ok()?
                    .into_iter()
                    .next()?
                    .0
            }
        };
        Some(myrn_model::id::episode_id(&key))
    }

    /// At the end of a turn that edited code, the request asking the agent to
    /// record what it learned. Asked once per batch of edits, never while the
    /// agent is already continuing because of a stop hook, and only for
    /// sessions being recorded (with consent).
    pub fn records_request(&mut self, session: &str, continued: bool) -> Result<Option<String>> {
        if continued || !self.config.records.ask_at_session_end {
            return Ok(None);
        }
        let dir = self.layout.episodes_dir();
        let path = episodes::path_for(&dir, session);
        if !path.exists() {
            return Ok(None);
        }
        let entries = episodes::read(&path)?;
        let since = entries
            .iter()
            .rposition(|e| e.event == "record_request")
            .map(|i| i + 1)
            .unwrap_or(0);
        let recent = &entries[since..];
        let edits: Vec<&Entry> = recent
            .iter()
            .filter(|e| e.outcome.as_deref() == Some("file_edit"))
            .collect();
        if edits.is_empty() {
            return Ok(None);
        }
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        episodes::append(
            &dir,
            session,
            &Entry {
                t: now,
                agent: entries.first().map(|e| e.agent.clone()).unwrap_or_default(),
                event: "record_request".into(),
                ..Default::default()
            },
        )?;

        let mut files: Vec<&str> = edits
            .iter()
            .flat_map(|e| e.files.iter().map(String::as_str))
            .collect();
        files.sort();
        files.dedup();
        let mut symbols: Vec<String> = Vec::new();
        for id in edits.iter().flat_map(|e| e.symbols.iter()) {
            if let Some(n) = graph::node(&self.db.conn, id)? {
                if n.ntype == "symbol" && !self.summary_is_current(&n)? {
                    let name = n.qname.clone().unwrap_or(n.id.clone());
                    if !symbols.contains(&name) {
                        symbols.push(name);
                    }
                }
            }
        }
        symbols.truncate(4);
        let mut errors: Vec<String> = recent
            .iter()
            .filter_map(|e| e.error.as_deref())
            .filter_map(signature::normalize)
            .map(|s| s.text())
            .collect();
        errors.dedup();
        errors.truncate(3);
        let tests: Vec<&str> = recent
            .iter()
            .filter(|e| e.test.is_some())
            .filter_map(|e| e.command.as_deref())
            .collect();

        // Agents connected without the MCP server (the default with hooks)
        // record through the CLI instead of the `record` tool.
        let agent = entries.first().map(|e| e.agent.clone()).unwrap_or_default();
        let has_tool = self
            .state()?
            .agents
            .get(&agent)
            .is_some_and(|a| a.mode == "pull_only" || a.mcp);
        let how = if has_tool {
            "Make ONE call to the myrn `record` tool with every item in `items`".to_string()
        } else {
            let bin = std::env::current_exe()
                .ok()
                .map(|p| p.to_string_lossy().into_owned())
                .unwrap_or_else(|| "myrn".into());
            format!(
                "Run ONE shell command with every item in `items` as JSON on stdin: \
                 `\"{bin}\" record <<'EOF'` then {{\"items\": [...]}} then `EOF`"
            )
        };
        let mut out = format!(
            "Before finishing: record what this session learned so later sessions don't \
             rediscover it. {how}, then reply only \"recorded\". Include a bug (with root cause \
             and fix), a decision and why, a workflow that worked, or a non-obvious fact; skip \
             anything plain from the code. If nothing is worth keeping, reply \"nothing to \
             record\" without recording.\n",
        );
        out.push_str(&format!("\nEdited: {}.\n", files.join(", ")));
        if !errors.is_empty() {
            out.push_str(&format!("Errors seen: {}.\n", errors.join("; ")));
        }
        if let Some(t) = tests.last().and_then(|t| checks::test_step(t)) {
            out.push_str(&format!("Test command: `{t}`.\n"));
        }
        out.push_str(
            "\nItem: {\"type\": \"bug\"|\"decision\"|\"workflow\"|\"fact\"|\"summary\", \
             \"body\": \"...\", \"about\": [\"qualified::name\"], \
             \"evidence\": [{\"symbol\"|\"commit\"|\"test\"|\"test_run\": \"...\"}]}; \
             bug adds \"error\", \"root_cause\", \"located_in\", \"fix\"; workflow adds \
             \"steps\". Evidence must exist: a symbol, commit, test, or test command you ran.\n",
        );
        if !symbols.is_empty() {
            out.push_str(&format!(
                "Also include a summary item (about = the symbol; what it does, inputs, outputs, \
                 edge cases, gotchas) for: {}.\n",
                symbols.join(", ")
            ));
        }
        Ok(Some(out))
    }

    /// Does this node have a summary written for its current body?
    pub(crate) fn summary_is_current(&self, n: &graph::Node) -> Result<bool> {
        if n.summary.is_none() {
            return Ok(false);
        }
        Ok(!self.summary_outdated(n)?)
    }

    /// The node's summary was written for a different body hash.
    pub fn summary_outdated(&self, n: &graph::Node) -> Result<bool> {
        let (Some(_), Some(current)) = (&n.summary, &n.body_hash) else {
            return Ok(false);
        };
        let mut stmt = self.db.conn.prepare_cached(
            "SELECT a.body_hash FROM anchors a JOIN notes s ON s.id = a.note_id
             WHERE a.symbol_id = ?1 AND s.type = 'summary' AND a.body_hash IS NOT NULL",
        )?;
        let hashes: Vec<String> = stmt
            .query_map([&n.id], |r| r.get(0))?
            .collect::<rusqlite::Result<_>>()?;
        Ok(!hashes.is_empty() && !hashes.iter().any(|h| current.starts_with(h.as_str())))
    }
}

fn link(rel: Relation, target: &str) -> Link {
    Link {
        rel,
        target: target.to_string(),
        inverse: false,
    }
}

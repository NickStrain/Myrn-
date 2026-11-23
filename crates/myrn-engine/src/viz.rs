//! Data for `myrn ui`: the project map, the code graph, the notes graph,
//! learned records and sessions, as JSON-ready structs. Read-only.

use std::collections::{BTreeMap, HashMap, HashSet};

use anyhow::Result;
use myrn_model::Relation;
use myrn_store::graph::{self, Node};
use myrn_store::notes::Note;
use myrn_store::sync;
use serde::Serialize;

use crate::api::Engine;
use crate::knowledge::RecordFilter;
use crate::summarize;

/// A graph for the browser: nodes and weighted links.
#[derive(Debug, Default, Serialize)]
pub struct Graph {
    pub nodes: Vec<GNode>,
    pub links: Vec<GLink>,
    /// Nodes left out to keep the drawing readable.
    pub truncated: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct GNode {
    pub id: String,
    pub label: String,
    /// file, module, symbol, test, commit, fact, bug, summary, code, ...
    #[serde(rename = "type")]
    pub ntype: String,
    pub kind: Option<String>,
    pub path: Option<String>,
    /// Records: unverified, verified, suspect, superseded.
    pub status: Option<String>,
    /// Notes: private or shared.
    pub visibility: Option<String>,
    /// Top-level folder, for colouring the code graph.
    pub group: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct GLink {
    pub s: String,
    pub t: String,
    pub rel: String,
    /// How many underlying edges this link stands for.
    pub w: u32,
}

/// One entry in the project map tree.
#[derive(Debug, Serialize)]
pub struct MapEntry {
    pub id: String,
    pub name: String,
    /// module (directory), file or symbol
    #[serde(rename = "type")]
    pub ntype: String,
    pub kind: Option<String>,
    pub path: Option<String>,
    pub line: Option<i64>,
    pub summary: Option<String>,
    /// fresh, stale (the code changed since) or missing
    pub state: &'static str,
    pub children: Vec<MapEntry>,
}

#[derive(Debug, Serialize)]
pub struct MapData {
    pub root: Option<MapEntry>,
    /// (fresh, stale, missing) over files and symbols the map covers.
    pub fresh: usize,
    pub stale: usize,
    pub missing: usize,
}

/// Details of any node or note, for the side panel.
#[derive(Debug, Serialize)]
pub struct Details {
    pub node: Option<Node>,
    /// The note's Markdown body (notes only).
    pub body: Option<String>,
    pub note_file: Option<String>,
    pub out: Vec<Neighbor>,
    pub inc: Vec<Neighbor>,
}

#[derive(Debug, Serialize)]
pub struct Neighbor {
    pub rel: String,
    pub id: String,
    pub label: String,
    #[serde(rename = "type")]
    pub ntype: String,
}

#[derive(Debug, Serialize)]
pub struct SessionRow {
    #[serde(flatten)]
    pub episode: crate::capture::episode::Episode,
    pub outcome: &'static str,
    pub summary: String,
    pub injected: usize,
    pub used: usize,
}

#[derive(Debug, Serialize)]
pub struct RecordsData {
    pub records: Vec<RecordView>,
    pub conflicts: Vec<sync::Conflict>,
}

#[derive(Debug, Serialize)]
pub struct RecordView {
    #[serde(flatten)]
    pub row: crate::knowledge::RecordRow,
    pub injected: u32,
    pub used: u32,
}

fn top_group(path: Option<&str>) -> String {
    let p = path.unwrap_or("");
    let mut it = p.split('/');
    match (it.next(), it.next()) {
        (Some("crates"), Some(c)) => c.to_string(),
        (Some(a), Some(_)) => a.to_string(),
        _ => ".".to_string(),
    }
}

fn gnode(n: &Node) -> GNode {
    let label = match n.ntype.as_str() {
        "file" | "module" => n
            .path
            .clone()
            .filter(|p| !p.is_empty())
            .unwrap_or_else(|| n.title.clone().unwrap_or_else(|| n.id.clone())),
        "commit" => format!(
            "{} {}",
            &n.qname.as_deref().unwrap_or(&n.id)
                [..7.min(n.qname.as_deref().unwrap_or(&n.id).len())],
            n.title.as_deref().unwrap_or("")
        ),
        _ => n.display_name().to_string(),
    };
    let visibility = n
        .extra
        .as_deref()
        .and_then(|e| serde_json::from_str::<serde_json::Value>(e).ok())
        .and_then(|e| e["visibility"].as_str().map(str::to_string));
    GNode {
        id: n.id.clone(),
        label,
        ntype: n.ntype.clone(),
        kind: n.kind.clone(),
        path: n.path.clone(),
        status: n.status.clone(),
        visibility,
        group: top_group(n.path.as_deref()),
    }
}

impl Engine {
    /// The code graph. `level` is `files` (imports and calls between files),
    /// `dirs` (directories and the imports between them) or `focus` (one node,
    /// `focus`, and everything within `depth` edges of it).
    pub fn viz_code_graph(&self, level: &str, focus: Option<&str>, depth: usize) -> Result<Graph> {
        let c = &self.db.conn;
        match (level, focus) {
            ("focus", Some(id)) => self.viz_neighborhood(id, depth.clamp(1, 3), 400),
            ("dirs", _) => {
                let mods = graph::nodes_where(c, "type = 'module'", &[])?;
                let file_dir: HashMap<String, String> = edges(
                    c,
                    "SELECT e.src, e.dst FROM edges e JOIN nodes n ON n.id = e.src
                     WHERE e.rel = 'part_of' AND n.type = 'file'",
                )?
                .into_iter()
                .collect();
                let mut links: BTreeMap<(String, String, &str), u32> = BTreeMap::new();
                for (s, t) in edges(
                    c,
                    "SELECT e.src, e.dst FROM edges e JOIN nodes n ON n.id = e.src
                     WHERE e.rel = 'part_of' AND n.type = 'module'",
                )? {
                    *links.entry((s, t, "part_of")).or_default() += 1;
                }
                for (s, t) in edges(c, "SELECT src, dst FROM edges WHERE rel = 'imports'")? {
                    if let (Some(a), Some(b)) = (file_dir.get(&s), file_dir.get(&t)) {
                        if a != b {
                            *links.entry((a.clone(), b.clone(), "imports")).or_default() += 1;
                        }
                    }
                }
                Ok(Graph {
                    nodes: mods.iter().map(gnode).collect(),
                    links: links
                        .into_iter()
                        .map(|((s, t, rel), w)| GLink {
                            s,
                            t,
                            rel: rel.into(),
                            w,
                        })
                        .collect(),
                    truncated: 0,
                })
            }
            _ => {
                let files = graph::nodes_where(c, "type = 'file'", &[])?;
                let sym_file: HashMap<String, String> =
                    edges(c, "SELECT src, dst FROM edges WHERE rel = 'defined_in'")?
                        .into_iter()
                        .collect();
                let mut links: BTreeMap<(String, String, &str), u32> = BTreeMap::new();
                for (s, t) in edges(c, "SELECT src, dst FROM edges WHERE rel = 'imports'")? {
                    if s != t {
                        *links.entry((s, t, "imports")).or_default() += 1;
                    }
                }
                for (rel, sql) in [
                    ("calls", "SELECT src, dst FROM edges WHERE rel = 'calls'"),
                    ("tests", "SELECT src, dst FROM edges WHERE rel = 'tests'"),
                ] {
                    for (s, t) in edges(c, sql)? {
                        if let (Some(a), Some(b)) = (sym_file.get(&s), sym_file.get(&t)) {
                            if a != b {
                                *links.entry((a.clone(), b.clone(), rel)).or_default() += 1;
                            }
                        }
                    }
                }
                Ok(Graph {
                    nodes: files.iter().map(gnode).collect(),
                    links: links
                        .into_iter()
                        .map(|((s, t, rel), w)| GLink {
                            s,
                            t,
                            rel: rel.into(),
                            w,
                        })
                        .collect(),
                    truncated: 0,
                })
            }
        }
    }

    /// One node and its neighbors out to `depth` edges, at most `max` nodes.
    fn viz_neighborhood(&self, id: &str, depth: usize, max: usize) -> Result<Graph> {
        let c = &self.db.conn;
        let mut seen: HashSet<String> = HashSet::from([id.to_string()]);
        let mut frontier = vec![id.to_string()];
        let mut links: BTreeMap<(String, String, String), u32> = BTreeMap::new();
        let mut truncated = 0;
        for _ in 0..depth {
            let mut next = Vec::new();
            for n in &frontier {
                for e in graph::edges_of(c, n)? {
                    let other = if e.src == *n { &e.dst } else { &e.src };
                    if !seen.contains(other) {
                        if seen.len() >= max {
                            truncated += 1;
                            continue;
                        }
                        seen.insert(other.clone());
                        next.push(other.clone());
                    }
                    links.insert((e.src.clone(), e.dst.clone(), e.rel.clone()), 1);
                }
            }
            frontier = next;
        }
        let mut nodes = Vec::new();
        for n in &seen {
            if let Some(node) = graph::node(c, n)? {
                nodes.push(gnode(&node));
            }
        }
        let have: HashSet<&str> = nodes.iter().map(|n| n.id.as_str()).collect();
        let links = links
            .into_iter()
            .filter(|((s, t, _), _)| have.contains(s.as_str()) && have.contains(t.as_str()))
            .map(|((s, t, rel), w)| GLink { s, t, rel, w })
            .collect();
        Ok(Graph {
            nodes,
            links,
            truncated,
        })
    }

    /// The Markdown notes as Obsidian draws them: one node per note, linked by
    /// their header links and `[[wikilinks]]`. With `code`, the code each note
    /// points at is drawn too.
    pub fn viz_notes_graph(&self, code: bool) -> Result<Graph> {
        let c = &self.db.conn;
        let mut nodes: Vec<GNode> = Vec::new();
        let mut ids: HashSet<String> = HashSet::new();
        let mut about: Vec<(String, String)> = Vec::new();
        // (from, rel, target as written): header links and body [[wikilinks]].
        let mut written: Vec<(String, String, String)> = Vec::new();
        {
            let mut stmt = c.prepare(
                "SELECT n.path, n.id, n.type, n.visibility, n.data, x.status
                 FROM notes n LEFT JOIN nodes x ON x.id = n.id
                 ORDER BY n.visibility = 'shared' DESC, n.path",
            )?;
            let rows = stmt.query_map([], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?,
                    r.get::<_, String>(4)?,
                    r.get::<_, Option<String>>(5)?,
                ))
            })?;
            for row in rows {
                let (path, id, ntype, visibility, data, status) = row?;
                if !ids.insert(id.clone()) {
                    continue; // duplicate copy
                }
                let note: Option<Note> = serde_json::from_str(&data).ok();
                let label = note
                    .as_ref()
                    .map(|n| n.display_title())
                    .unwrap_or_else(|| id.clone());
                if let Some(n) = &note {
                    for l in &n.links {
                        if l.rel == Relation::About {
                            about.push((id.clone(), l.target.clone()));
                        } else {
                            written.push((id.clone(), l.rel.as_str().into(), l.target.clone()));
                        }
                    }
                    for t in myrn_store::notes::wikilink::extract(&n.body) {
                        written.push((id.clone(), "link".into(), t));
                    }
                }
                nodes.push(GNode {
                    id: id.clone(),
                    label,
                    ntype: ntype.clone(),
                    kind: Some("note".into()),
                    path: Some(path),
                    status: status.or_else(|| note.as_ref().map(|n| n.status.clone())),
                    visibility: Some(visibility),
                    group: ntype,
                });
            }
        }
        let mut links: BTreeMap<(String, String, String), u32> = BTreeMap::new();
        // Links between notes, resolved by id the way Obsidian resolves them.
        let by_lower: HashMap<String, String> =
            ids.iter().map(|i| (i.to_lowercase(), i.clone())).collect();
        for (s, rel, t) in written {
            let key = t.trim().to_lowercase();
            if let Some(dst) = by_lower.get(&key) {
                if *dst != s && !links.contains_key(&(dst.clone(), s.clone(), rel.clone())) {
                    links.insert((s, dst.clone(), rel), 1);
                }
            }
        }
        let mut code_ids: HashSet<String> = HashSet::new();
        for (s, rel, t) in edges3(
            c,
            "SELECT src, rel, dst FROM edges WHERE evidence LIKE 'note:%' OR evidence LIKE 'check:%'",
        )? {
            let (sn, tn) = (ids.contains(&s), ids.contains(&t));
            if sn && tn {
                links.insert((s, t, rel), 1);
            } else if code && (sn || tn) {
                code_ids.insert(if sn { t.clone() } else { s.clone() });
                links.insert((s, t, rel), 1);
            }
        }
        if code {
            // A summary describes its code: draw that too.
            for (s, t) in about {
                if !ids.contains(&t) {
                    code_ids.insert(t.clone());
                    links.insert((s, t, "about".into()), 1);
                }
            }
            for id in &code_ids {
                if let Some(n) = graph::node(c, id)? {
                    let mut g = gnode(&n);
                    g.group = "code".into();
                    nodes.push(g);
                }
            }
        }
        let have: HashSet<&str> = nodes.iter().map(|n| n.id.as_str()).collect();
        let links = links
            .into_iter()
            .filter(|((s, t, _), _)| {
                s != t && have.contains(s.as_str()) && have.contains(t.as_str())
            })
            .map(|((s, t, rel), w)| GLink { s, t, rel, w })
            .collect();
        Ok(Graph {
            nodes,
            links,
            truncated: 0,
        })
    }

    /// The project map: directories, files and symbols with their summaries.
    pub fn viz_map(&self) -> Result<MapData> {
        let c = &self.db.conn;
        // Summary anchor hash by the node it is about.
        let mut anchor: HashMap<String, Option<String>> = HashMap::new();
        {
            let mut stmt = c.prepare("SELECT data FROM notes WHERE type = 'summary'")?;
            let rows = stmt.query_map([], |r| r.get::<_, String>(0))?;
            for data in rows {
                let Ok(note) = serde_json::from_str::<Note>(&data?) else {
                    continue;
                };
                if let Some(a) = note.links.iter().find(|l| l.rel == Relation::About) {
                    anchor.insert(
                        a.target.clone(),
                        note.anchors.first().and_then(|x| x.hash.clone()),
                    );
                }
            }
        }
        let mut counts = (0, 0, 0);
        let state = |n: &Node, counts: &mut (usize, usize, usize)| -> &'static str {
            let has = n.summary.as_deref().is_some_and(|s| !s.trim().is_empty());
            let s = if !has {
                "missing"
            } else if n.ntype == "module" {
                "fresh"
            } else {
                let cur = n.body_hash.as_deref().unwrap_or("");
                match anchor.get(&n.id).cloned().flatten() {
                    Some(h) if !h.is_empty() && cur.starts_with(&h) => "fresh",
                    None => "fresh", // a summary written by hand, no anchor
                    _ => "stale",
                }
            };
            if n.ntype != "module" {
                match s {
                    "fresh" => counts.0 += 1,
                    "stale" => counts.1 += 1,
                    _ => counts.2 += 1,
                }
            }
            s
        };
        let modules = graph::nodes_where(c, "type = 'module' ORDER BY path", &[])?;
        let files = graph::nodes_where(c, "type = 'file' ORDER BY path", &[])?;
        let kinds = [
            "function", "method", "class", "struct", "enum", "trait", "type", "macro",
        ]
        .map(|k| format!("'{k}'"))
        .join(", ");
        let symbols = graph::nodes_where(
            c,
            &format!("type = 'symbol' AND kind IN ({kinds}) ORDER BY path, start_line"),
            &[],
        )?;
        let parent: HashMap<String, String> =
            edges(c, "SELECT src, dst FROM edges WHERE rel = 'part_of'")?
                .into_iter()
                .collect();

        let entry = |n: &Node, st: &'static str| MapEntry {
            id: n.id.clone(),
            name: match n.ntype.as_str() {
                "symbol" => n.display_name().to_string(),
                _ => n.title.clone().unwrap_or_else(|| n.id.clone()),
            },
            ntype: n.ntype.clone(),
            kind: n.kind.clone(),
            path: n.path.clone(),
            line: n.start_line,
            summary: n.summary.clone(),
            state: st,
            children: Vec::new(),
        };
        let mut by_file: HashMap<String, Vec<MapEntry>> = HashMap::new();
        for s in &symbols {
            let st = state(s, &mut counts);
            by_file
                .entry(myrn_model::id::file_id(s.path.as_deref().unwrap_or("")))
                .or_default()
                .push(entry(s, st));
        }
        let mut items: HashMap<String, MapEntry> = HashMap::new();
        for f in &files {
            let st = state(f, &mut counts);
            let mut e = entry(f, st);
            e.children = by_file.remove(&f.id).unwrap_or_default();
            items.insert(f.id.clone(), e);
        }
        for m in &modules {
            let st = state(m, &mut counts);
            items.insert(m.id.clone(), entry(m, st));
        }
        // Attach deepest first, so each directory is complete when it moves.
        let mut order: Vec<String> = items.keys().cloned().collect();
        order.sort_by_key(|id| {
            std::cmp::Reverse(
                items[id]
                    .path
                    .as_deref()
                    .map(|p| p.matches('/').count() + usize::from(!p.is_empty()))
                    .unwrap_or(0)
                    * 2
                    + usize::from(items[id].ntype == "file"),
            )
        });
        for id in order {
            let Some(p) = parent.get(&id) else { continue };
            if !items.contains_key(p) || p == &id {
                continue;
            }
            let child = items.remove(&id).expect("present");
            items.get_mut(p).expect("parent").children.push(child);
        }
        fn sort(e: &mut MapEntry) {
            e.children.sort_by(|a, b| {
                let rank = |x: &MapEntry| match x.ntype.as_str() {
                    "module" => 0,
                    "file" => 1,
                    _ => 2,
                };
                rank(a)
                    .cmp(&rank(b))
                    .then(a.line.cmp(&b.line))
                    .then(a.name.cmp(&b.name))
            });
            for c in &mut e.children {
                sort(c);
            }
        }
        let mut root = items
            .remove(&myrn_model::id::module_id(""))
            .or_else(|| items.remove("module:."));
        if root.is_none() && !items.is_empty() {
            // No root directory node: gather what is left under one.
            let mut r = MapEntry {
                id: "root".into(),
                name: ".".into(),
                ntype: "module".into(),
                kind: Some("directory".into()),
                path: Some(String::new()),
                line: None,
                summary: None,
                state: "missing",
                children: Vec::new(),
            };
            r.children = items.into_values().collect();
            root = Some(r);
        }
        if let Some(r) = root.as_mut() {
            sort(r);
        }
        Ok(MapData {
            root,
            fresh: counts.0,
            stale: counts.1,
            missing: counts.2,
        })
    }

    /// Any node or note, with its links, for the side panel.
    pub fn viz_details(&self, id: &str) -> Result<Details> {
        let c = &self.db.conn;
        let node = graph::node(c, id)?;
        let note: Option<(String, String)> = c
            .query_row(
                "SELECT path, data FROM notes WHERE id = ?1
                 ORDER BY visibility = 'shared' DESC LIMIT 1",
                [id],
                |r| Ok((r.get(0)?, r.get(1)?)),
            )
            .ok();
        let body = note.as_ref().and_then(|(_, d)| {
            serde_json::from_str::<Note>(d).ok().map(|n| {
                crate::pack::render::safe_text(node.as_ref().unwrap_or(&Node::default()), &n.body)
            })
        });
        let label_of = |nid: &str| -> (String, String) {
            match graph::node(c, nid).ok().flatten() {
                Some(n) => (gnode(&n).label, n.ntype),
                None => (nid.to_string(), "note".to_string()),
            }
        };
        let mut out = Vec::new();
        let mut inc = Vec::new();
        for e in graph::edges_of(c, id)?.into_iter().take(400) {
            if e.src == id {
                let (label, ntype) = label_of(&e.dst);
                out.push(Neighbor {
                    rel: e.rel,
                    id: e.dst,
                    label,
                    ntype,
                });
            } else {
                let (label, ntype) = label_of(&e.src);
                inc.push(Neighbor {
                    rel: e.rel,
                    id: e.src,
                    label,
                    ntype,
                });
            }
        }
        Ok(Details {
            node,
            body,
            note_file: note.map(|(p, _)| p),
            out,
            inc,
        })
    }

    /// Learned records with how often each was injected and used.
    pub fn viz_records(&self) -> Result<RecordsData> {
        let rows = self.records(&RecordFilter::default(), 2000)?;
        let mut counts: HashMap<String, (u32, u32)> = HashMap::new();
        {
            let mut stmt = self.db.conn.prepare(
                "SELECT note_id, SUM(injected), SUM(ifnull(used, 0)) FROM usage GROUP BY note_id",
            )?;
            let it = stmt.query_map([], |r| {
                Ok((
                    r.get::<_, String>(0)?,
                    r.get::<_, Option<i64>>(1)?.unwrap_or(0) as u32,
                    r.get::<_, Option<i64>>(2)?.unwrap_or(0) as u32,
                ))
            })?;
            for row in it {
                let (id, i, u) = row?;
                counts.insert(id, (i, u));
            }
        }
        Ok(RecordsData {
            records: rows
                .into_iter()
                .filter(|r| r.source.as_deref() != Some(summarize::SOURCE))
                .map(|row| {
                    let (injected, used) = counts.get(&row.id).copied().unwrap_or((0, 0));
                    RecordView {
                        row,
                        injected,
                        used,
                    }
                })
                .collect(),
            conflicts: sync::conflicts(&self.db.conn)?,
        })
    }

    /// Recorded sessions, newest first, with what was injected and used in each.
    pub fn viz_sessions(&self, limit: usize) -> Result<Vec<SessionRow>> {
        let mut counts: HashMap<String, (usize, usize)> = HashMap::new();
        {
            let mut stmt = self.db.conn.prepare(
                "SELECT session_id, SUM(injected), SUM(ifnull(used, 0)) FROM usage GROUP BY session_id",
            )?;
            let it = stmt.query_map([], |r| {
                Ok((
                    r.get::<_, Option<String>>(0)?.unwrap_or_default(),
                    r.get::<_, Option<i64>>(1)?.unwrap_or(0) as usize,
                    r.get::<_, Option<i64>>(2)?.unwrap_or(0) as usize,
                ))
            })?;
            for row in it {
                let (s, i, u) = row?;
                counts.insert(s, (i, u));
            }
        }
        Ok(self
            .episodes(limit)?
            .into_iter()
            .map(|e| {
                let (injected, used) = counts.get(&e.session).copied().unwrap_or((0, 0));
                SessionRow {
                    outcome: e.outcome(),
                    summary: e.summary(),
                    injected,
                    used,
                    episode: e,
                }
            })
            .collect())
    }
}

fn edges(c: &rusqlite::Connection, sql: &str) -> Result<Vec<(String, String)>> {
    let mut stmt = c.prepare(sql)?;
    let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?)))?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

fn edges3(c: &rusqlite::Connection, sql: &str) -> Result<Vec<(String, String, String)>> {
    let mut stmt = c.prepare(sql)?;
    let rows = stmt.query_map([], |r| Ok((r.get(0)?, r.get(1)?, r.get(2)?)))?;
    Ok(rows.collect::<rusqlite::Result<_>>()?)
}

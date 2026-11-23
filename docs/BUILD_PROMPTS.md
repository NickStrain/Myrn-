# Myrn — step-by-step build prompts

Source: `Persistent Project Brain architecture and design.pdf` (Nov 15, 2026).

These prompts follow the design doc's roadmap (Phase 0 → 5, each behind a gate).
Paste them into Claude Code **one at a time, in order**. Each one is a single
reviewable unit of work.

## How to use

- Start a fresh session for each prompt (or `/clear`), so every step starts with clean context.
  Prompt 0 creates `CLAUDE.md` and `docs/DESIGN.md`, which give every later session the background.
- For the larger steps, press Shift+Tab to enter plan mode first, review the plan, then let it build.
- Commit after each step passes `cargo test` and `cargo clippy`.
- Do not start a new phase until its gate prompt passes. The design doc requires this.

Crates (already built in Phase 0; use these names, do not create new ones):
`crates/myrn` (CLI binary), `crates/myrn-model` (types and events), `crates/myrn-store`
(SQLite and notes), `crates/myrn-engine` (indexer, capture, distill, validity, retrieve, pack,
and the single `api::Engine`), `crates/myrn-mcp` (MCP server), `crates/myrn-adapters`
(agent adapters), `crates/myrn-embed` (optional embeddings), `bench/`.

Defaults I picked where the doc leaves a question open (change them if you disagree):
- First-class languages: **Rust, Python, TypeScript/JavaScript**.
- Session-end summary: requested **only when the session edited code**.
- Licence: keep the existing `LICENSE` file.

---

## Step 0 — Turn the design doc into working context

```
Read "Persistent Project Brain architecture and design.pdf" in the repo root, all 25 pages
(the architecture diagram on page 3 and the roadmap on page 23 are images, so look at them).

Create:
1. docs/DESIGN.md: a faithful Markdown version of the whole document. Keep every table
   (node types, relation types, metadata fields, record states, walks by task signal, budgets,
   hooks, MCP tools, CLI commands, privacy controls, tech choices, platform handling, metrics,
   roadmap phases and gates). Do not summarise it away; later sessions will treat it as the spec.
2. CLAUDE.md: a short guide for agents working on this repo. Include: what Myrn is (2 lines),
   the 7 design principles, the stack (Rust, Tree-sitter, SQLite + FTS5, MCP over stdio, no
   daemon), the roadmap with gates, and these rules: the relation and node vocabularies are
   closed; the core makes no network calls; paths are stored repo-relative with forward
   slashes; hooks never depend on a POSIX shell; every phase must pass its gate before the
   next starts. Point to docs/DESIGN.md as the spec.
3. Replace README.md with a short project description and a "Status: Phase 0 in progress" line.
```

## Phase 0 — Code graph and pull tools (no hooks, no model)

### Step 1 — Scaffold the Rust workspace

```
Following docs/DESIGN.md, scaffold a Cargo workspace for the `myrn` binary:

- crates/myrn         (binary `myrn`, clap derive; subcommands stubbed: init, connect, status,
                         log, show, diff, trace, verify, forget, gc, rebuild, publish, doctor, mcp)
- crates/myrn-engine   (engine API: the six components as modules: indexer, capture, distill,
                         validity, retrieve, pack)
- crates/myrn-model    (model types for nodes/edges/records, events; no I/O)
- crates/myrn-store    (SQLite access and migrations)
- crates/myrn-mcp      (MCP stdio server, empty for now)
- crates/myrn-adapters (agent adapters, empty for now)

Also:
- Project id = hash of the repository's root commit (via gix), not the folder path.
- Per-user data dir via the `directories` crate: ~/.local/share/myrn/projects/<project-id>/
  on Linux, ~/Library/Application Support/myrn on macOS, %LOCALAPPDATA%\myrn on Windows.
- Closed enums for NodeType, Origin (derived/earned), Relation, Status, Visibility, matching
  the tables in the design doc exactly. Add serde with snake_case names.
- Error handling: thiserror in libraries, anyhow in the CLI. Logging: tracing, off by default.
- GitHub Actions CI: build + test + clippy + fmt on ubuntu, macos and windows.
- Use rusqlite with the `bundled` feature so there's no system SQLite dependency.

Keep it compiling with `cargo test` green. No feature logic yet.
```

### Step 2 — SQLite store

```
Implement crates/myrn-store per the "Code graph and index" section of docs/DESIGN.md.

- Schema: nodes, edges, anchors, notes, usage tables exactly as specified, plus an FTS5
  table over title, summary and note bodies, plus a `files` table (path, language,
  content_hash) for incremental indexing and a `meta` table (schema_version, last indexed
  commit). Leave the vector table for Phase 4 but reserve its migration slot.
- Versioned migrations run on open. WAL mode, sensible pragmas for a short-lived process.
- A typed API: upsert_node, upsert_edge, delete_nodes_for_file, neighbours(id, rels, dir),
  walk(seeds, rels, max_hops) implemented with a recursive CTE, fts_search(query, limit).
- Open must be fast; add a test that opening and doing one query is well under 20ms on a
  small db.
- Unit tests for every function, using a temp dir.
```

### Step 3 — Tree-sitter indexer (files, symbols, body hashes)

```
Implement the indexer in myrn-engine per "Capture and indexing" and "Incremental indexing" in
docs/DESIGN.md, for Rust, Python and TypeScript/JavaScript.

- Walk the repo with the `ignore` crate, honouring .gitignore and .myrn/ignore.
- File node: repo-relative path with forward slashes, language, content hash. Re-parse a file
  only when its content hash changes; delete its symbols when it is removed.
- Symbol nodes for functions, classes/structs/enums/traits/interfaces, methods and types:
  qualified name (e.g. auth::session::refresh, pkg.module.Class.method), signature text,
  line span, and a body hash computed from the syntax tree with comments and whitespace
  removed. CRLF vs LF and reformatting must produce the same hash; add tests proving it.
- Symbol identity = qualified name + file.
- Module nodes from directories/packages; edges defined_in (Symbol→File) and part_of
  (File→Module).
- Put each language's queries in its own file so adding a language is one file + one
  registration.
- Fixture repos under tests/fixtures/{rust,python,ts} with integration tests.
```

### Step 4 — Calls, imports and tests edges

```
Extend the indexer with the relation edges calls (Symbol→Symbol), imports (File→File) and
tests (Test→Symbol), per docs/DESIGN.md.

- Resolve calls by name within scope: same file, then imported names, then unique global
  matches. Any edge resolved by guesswork gets status "inferred" so retrieval can rank it
  down (see the "Imprecise code graph" risk).
- Detect tests per language (#[test] / mod tests, test_* / pytest, *.test.ts / describe/it)
  and create Test nodes with tests edges to the symbols they call.
- Leave a clean extension point for optional SCIP/LSP data to replace inferred edges later.
- Incremental: when a file changes, recompute only edges originating from it, and re-resolve
  edges that pointed at symbols that disappeared.
- Integration tests on the fixtures checking exact call and import edges.
```

### Step 5 — Git history

```
Add Git history indexing with gix (no shelling out), per docs/DESIGN.md.

- Commit nodes (SHA, message, author, time) and touches edges (Commit→Symbol) by diffing each
  commit and mapping changed line ranges to symbols.
- Detect renames; when a file is renamed and a symbol's body hash matches, keep its identity
  so later anchors follow the move.
- Detect reverts (message pattern + inverse diff) and store them, since Phase 3 needs
  "landed and not reverted".
- On `init`, index history with a configurable depth (default: last 1,000 commits);
  afterwards, index only commits newer than meta.last_indexed_commit.
```

### Step 6 — CLI: init, status, show, rebuild, doctor

```
Wire up the first CLI commands per the CLI section of docs/DESIGN.md:

- myrn init: create .myrn/project.toml (committed config) and .myrn/ignore, create the
  private store, run a full index of code and history, print a short summary. Index code
  only; it must not install hooks or record anything.
- myrn status: index health (files, symbols, edges, stale files), record counts by state,
  connected agents (none yet).
- myrn show <id|qualified-name>: print a symbol card in the format shown in the
  "Summaries and cards" section (function, signature, summary, calls, called by, tests,
  known, status).
- myrn rebuild: delete memory.db and recreate it from the code (notes come later).
- myrn doctor: check the store, schema version, ignore files; leave hooks for later.
- Every command first refreshes files whose hash changed (the cheap incremental path).
- Snapshot tests for output using insta.
```

### Step 7 — Retriever and context packer

```
Implement the retriever and context packer per "Retrieval and context packing" in
docs/DESIGN.md. For now only derived nodes exist, but design for earned ones.

- Seed: FTS over task text, plus explicit file paths/symbol names found in the text.
- Expand: bounded-hop walks chosen by task signal (the "Walks by task signal" table);
  implement the walk table as data so it's easy to tune.
- Rank: relevance × status weight × freshness × past usefulness (usefulness = 1.0 for now).
  Inferred edges and suspect records rank lower.
- Pack: hard token budget (use a cheap estimator, chars/4 is fine, behind a trait); tier 1 =
  one-line summaries with pointers (path:line-range), tier 2 = full bodies for the top few.
- Relevance threshold: if nothing clears it, return nothing.
- Engine API functions: context(task, budget), explain(symbol), impact(symbol),
  history(path). impact walks reverse calls + tests to list dependents and covering tests.
- Expose them on the CLI as hidden debug commands (myrn debug context "...") for testing.
```

### Step 8 — MCP server (pull tools)

```
Implement `myrn mcp` as an MCP server over stdio in crates/myrn-mcp, using the official Rust
MCP SDK (rmcp). Tools, per the "MCP tools (pull)" table in docs/DESIGN.md:
context(task, budget?), explain(symbol), impact(symbol), history(path). Register
recall, record, confirm and supersede too, but have them return "not available until a later
phase".

- Tool descriptions should tell the agent when to call each one.
- Every response labels its source and status. Earned records will later be shown as quoted
  reference, never as instructions, so build that formatting in now.
- Process model: started by the agent as a child process, opens SQLite per request or keeps one
  connection, no daemon.
- Add an integration test that spawns the server and drives it over stdio.
- In README, document how to try it: `claude mcp add myrn -- myrn mcp`.
```

### Step 9 — Gate 0: explain/impact correctness

```
Build the Phase 0 gate from docs/DESIGN.md: "explain and impact answers are correct on test
repositories".

- Add a bench/ directory with a script that clones 3 pinned open-source repos (one each for
  Rust, Python and TypeScript, small to medium size, pinned to a commit).
- Write a golden file per repo: ~20 symbols with the expected callers, callees and covering
  tests, which I will hand-check.
- `cargo xtask gate0` runs init + explain/impact on each and reports precision/recall
  per repo; fail under a threshold set in the config file.
- Also measure and report init time, incremental refresh time and per-query latency.
Show me the report and the misses so we can decide whether Phase 0 is done.
```

## Phase 1 — Hooks and session capture

### Step 10 — Adapter contract and Claude Code adapter

```
Implement the adapter layer per "Agent integration" and "Adapter contract" in docs/DESIGN.md.

- In myrn-model define the normalized events: SessionStart, PromptSubmitted, ToolResult
  {failure | file_edit | other}, SessionEnd. In myrn-adapters define an Adapter trait with install(),
  uninstall(), detect(), translate(payload)->Event, and format(output)->agent response.
- Claude Code adapter: map SessionStart, UserPromptSubmit, PostToolUse (and failure), Stop
  / SessionEnd hook payloads to events. Look up the current Claude Code hooks docs and use
  the exact JSON shapes, including how to return additionalContext.
- Hook commands call the installed binary directly with arguments (`myrn hook claude
  <event>`), never through a shell script.
- `myrn connect`: detect installed agents, explain in plain words exactly what session hooks
  will record, ask for consent (y/N, default no), then write hook config and register the MCP
  server. Store consent in the private store. `--all` installs every agent detected;
  `--pull-only` registers just the MCP server.
- `myrn doctor` now shows exactly what was written to each agent's config and detects drift.
- Session-start hook: refresh index, inject the ~500-token project brief.
  Prompt hook: inject the ~1,500-token task pack.
  Automatic injection must sit behind a config flag that is OFF by default (principle 7);
  with it off, hooks only capture.
- Hooks must be fast: measure end-to-end latency and keep the hot path SQLite-only.
```

### Step 11 — Codex adapter and pull-only mode

```
Add a Codex adapter using the same Adapter trait. Check the current Codex hooks and MCP
configuration docs for exact formats. Then add pull-only mode for MCP clients without hooks:
`connect` adds a short line to that agent's instruction file (AGENTS.md / CLAUDE.md) telling
it to call `context` first, and removes it cleanly on disconnect. Add tests that run each
adapter against recorded sample payloads.
```

### Step 12 — Episode capture and secret redaction

```
Implement session capture per "Session capture" and "Privacy and security" in
docs/DESIGN.md.

- Append normalized events to an episode log in the private store
  (episodes/<session-id>.jsonl): task text, files read/edited, commands run, errors, outcome.
  Record what the agent did, not the model transcript.
- Only when consent was given; .myrn/ignore paths are excluded from capture.
- Redact secrets before any write: known token patterns (AWS, GitHub, Slack, JWT, private
  keys, .env-style KEY=VALUE for sensitive names) plus a Shannon-entropy check for long
  random strings. Thorough unit tests, including false-positive cases like hashes in paths.
- Outcomes: test runs pass/fail from tool results, commits made during the session.
- Record Episode nodes and touches of symbols for edited files.
- Log every injection in the usage table (note/node ids, session, injected=1).
- `myrn log` lists episodes.
```

### Step 13 — Gate 1: paired benchmark

```
Build the benchmark from the "Evaluation" section of docs/DESIGN.md, reusing bench/ repos.

- Task set per repo: bug fixes with known solutions (break a test and record the fix), small
  features, and code questions. YAML task files with setup, prompt and success check.
- Runner: runs each task N times with and without myrn (injection on vs off), using Claude
  Code headless (`claude -p` with --output-format stream-json) so token usage and tool calls
  can be parsed.
- Metrics: total tokens, file reads/searches before first correct edit, wall-clock time to
  passing tests, task success, injection use, hook latency.
- Output a markdown report with per-task and aggregate numbers and confidence intervals.
Gate: fewer total tokens and no drop in task success. Don't make injection default-on unless
this passes; report results to me either way.
```

## Phase 2 — Distillation

### Step 14 — Context packs that answer, not just point

> **Status (2026-11-16): mostly done; remaining: re-run Gate 1.** Done: code in the pack
> (top hit up to 80 lines, the next up to 40), relevance cutoff (40% of the best score,
> at most 5 items), no "most-called" brief, tests summarised in one line, commits and
> test-file symbols only when asked for, stubs merged, new header, backticked names as
> seeds, test callers no longer boost ranking, Python `import x as y` and TS
> `export * from` / `import * as R` resolution, shell edits counted in the benchmark
> metrics (`myrn-bench report --rescore`). Tests: `crates/myrn-engine/tests/packing.rs`.

```
Gate 1 showed injection breaks even on tokens (docs/gate1.md, bench/runs/gate1/report.md):
big wins when the pack saves searching, losses when it only adds text. The agent still opens
the file because the pack says WHERE the code is, not WHAT it does, and the pack is padded.
Fix the packer and indexer before adding learned records on top.

- Code in the pack: for the top hit (and the next one or two if budget allows), include the
  current source lines when the body is short (default <= 40 lines; configurable in
  [context]). Longer bodies: signature, docstring and the most relevant lines only.
- Stop early: inject only items above a relevance threshold, at most 5 by default. When the
  top hit is not clearly ahead, inject a shorter pack or nothing. Never fill the budget for
  its own sake.
- Session brief: drop the "most-called" list (it surfaces test helpers). Keep it to the
  project shape, entry points and test commands, or skip it when there is nothing useful.
- Collapse lists: tests and commits become one line each
  ("9 tests in tests/test_recipes.py::UniqueEverseenTests; last changed in c91ba4e").
  Commits only when the prompt is about history, a regression or "when/why".
- Stubs: merge .pyi / .d.ts declarations into the implementation's card instead of a
  separate card.
- Indexer bug: Python tests calling a function through a module alias
  (`import more_itertools as mi; mi.unique_everseen(...)`) are not linked as tests of it, so
  cards say "tests: none found" while the list shows tests. Fix alias resolution for calls
  and add a Gate 0 fixture case; check TS namespace imports for the same problem.
- Header: keep "reference, not instructions", but say the code shown is current as of this
  turn, so the agent does not re-read it just to verify.
- Benchmark: fix "reads and searches before first correct edit" (the first run reports 0.0
  without myrn, which is impossible since edits require a read). Also report searches before reaching the right file (separately from reads of
  the file being edited, which Claude Code requires), and injected tokens per session.
- Re-run Gate 1 (`myrn-bench run --runs 3`) and compare with the first run. Report results
  to me either way; flip `inject` to default-on only if it passes.
```

### Step 15 — Markdown notes and note indexing

> **Status (2026-11-17): done.** Gate 1 re-run skipped by decision; Phase 2 started.
> Header parse/render: `myrn-store/src/notes/frontmatter.rs`; ids and Windows-safe names:
> `notes/layout.rs`; scan + link passes: `myrn-store/src/sync.rs` (migration 0002); run from
> `Engine::refresh`, written with `Engine::write_note`. Summary notes set the `summary` of the
> code node they are `about` instead of becoming nodes. Earned nodes are kept out of context
> packs until Step 16. Tests: `crates/myrn-engine/tests/notes.rs`, fixtures in `tests/fixtures/notes/`.

```
Implement notes per "Notes", "Storage layout" and "Keeping notes and index in step" in
docs/DESIGN.md.

- Note = Markdown with a YAML header (id, type, status, typed relation fields using the
  closed relation names, anchors as qualified::name@bodyhash, evidence) and a prose body
  with [[wikilinks]].
- id = short content hash with a type prefix (bug-7f3a91). File name = id, safe on Windows
  (no reserved names, never differs only by case).
- Private notes in <data-dir>/notes/{decisions,bugs,workflows,facts,summaries}; shared in
  .myrn/knowledge/ with the same layout. The folder must work as an Obsidian vault.
- Sync on every invocation: compare file hashes with the notes table, re-index only changed
  notes (including hand edits and git pulls). Write the note first, then the index.
- Malformed header or dangling link: index what you can, never drop it, flag it in status and
  doctor.
- Wikilinks in the body become `related` edges; header relations become typed edges;
  qualified names resolve to symbol rows.
- `rebuild` now recreates from code AND notes. Round-trip tests.
```

### Step 16 — The record tool, distiller checks and recall

> **Status (2026-11-17): done.** `Engine::record` (`distill/record.rs`) with checks
> (`checks.rs`), bug/summary/id dedupe (`dedupe.rs`), negation-based contradictions
> (`contradiction.rs`), signatures (`signature.rs`); `Engine::recall`/`recall_text`
> (`retrieve/recall.rs`); MCP `record` and `recall`; hooks: recall on a failed tool,
> record request on Stop (once per batch of edits, never when `stop_hook_active`).
> Config `[records]`: `ask_at_session_end`, `recall_on_failure` (both default on),
> `failure_budget`. Cards show known records and flag outdated summaries; packs quote
> earned records ranked by state. Tests: `crates/myrn-engine/tests/records.rs`.

```
Implement Phase 2 per "Distillation and verification" in docs/DESIGN.md.

- MCP tool record(item): accepts a decision, bug (+ root cause), workflow or fact in a fixed
  JSON schema. The distiller checks:
  * Evidence: must cite an existing symbol, commit or test run, otherwise reject with a
    clear reason the agent can act on.
  * Vocabulary: only closed node/relation types.
  * Duplicates: bugs matched by normalized error signature + touched symbols; a match
    updates the existing node.
  * Contradictions: conflicting records get a contradicts edge and are ranked down.
- Accepted records: written as private notes with status unverified, anchors capturing the
  current body hashes, a produced edge from the episode.
- Error signature normalization: strip paths, line numbers, addresses, timestamps, ids;
  keep the error type + stable message parts. Unit tests across Rust/Python/TS errors.
- recall(error) MCP tool, and the tool-failure hook: on a failure, match the signature and
  inject the known bug, root cause and fix (~500 tokens).
- Session-end hook: if the session edited code, ask the agent to call record for what it
  learned, with the schema in the prompt. With no consent/model, still keep the mechanical
  episode.
- Symbol summaries: record also accepts a summary for a symbol ("what it does, inputs,
  outputs, edge cases, gotchas"), stored in summaries/ and anchored to the symbol's body
  hash. Cards show it in place of the docstring line, so later sessions learn WHAT a
  function does without opening it. At session end, ask the agent to summarise the
  symbols it edited or read closely. A summary whose anchor hash no longer matches is
  shown as possibly outdated (full handling in Step 18).
- Retriever: include earned nodes, presented as quoted reference with source and status.
```

### Step 17 — Gate 2: record precision review

```
Build the Phase 2 gate: "a hand-reviewed sample of records meets a precision target".
Add `myrn debug review --sample 30` that exports a random sample of records with their
evidence and the code they point at into a markdown checklist I can mark
correct/vague/wrong, and a command that reads my marks back and computes precision.
Run the benchmark tasks with recording on to generate records, then give me the sample.
```

## Phase 3 — Validity tracking

### Step 18 — Hash anchors, suspect state and verification

> **Status (2026-11-22): done.** Suspect and superseded are derived from the current tree
> in the store's link pass (`myrn-store/src/sync.rs`, `derive_states`), so a branch switch
> restores records without touching the notes; the note keeps the state it earned.
> `Engine::confirm`, `supersede`, `promote` (cited test passed in a recorded session, or
> `fixed_in` commit in history and not reverted), `suspects` in `validity/states.rs`;
> hook notice for suspect records in files the agent works on (`validity/anchors.rs`, once
> per session); packs label them and ask for confirm/supersede; MCP `confirm` and
> `supersede`; `myrn verify [--confirm <id>]`; promotion also runs at session end.
> Tests: `crates/myrn-engine/tests/validity.rs` (one per row of the hashes table).

```
Implement "Validity and staleness" and "Record states" in docs/DESIGN.md.

- When the indexer sees a new body hash for an anchored symbol, mark the dependent records
  suspect. If the hash returns to the old value (branch switch), restore their prior state.
  Validity is checked per working tree by hash, never by commit.
- unverified → verified when a cited test passes or the fixing commit lands and is not reverted.
- Ranking: verified > unverified > suspect; suspect records are labelled "possibly
  outdated" when shown.
- When an agent works on a symbol with suspect records, the hook shows them and asks it to
  call confirm(id) or supersede(id, item). confirm refreshes anchors and restores state;
  supersede adds a supersedes edge. Nothing is deleted.
- `myrn verify` re-checks all suspect records against current code.
- Tests for every row of the "Why hashes instead of revisions" table (branch switch,
  unmerged branch, worktrees with their own code index but shared notes, reformat-only).
```

### Step 19 — History and housekeeping commands

> **Status (2026-11-22): built; gate not run.** `myrn log` (sessions + records, `--type`,
> `--status`, `--map`), `show` (evidence and file for records), `trace`, `diff` (shared
> knowledge between revisions or dates), `forget` (with root causes only it used, links to it
> removed from other records, confirmation), `gc` (duplicate copies, orphaned index rows,
> compaction of old session logs). Engine: `knowledge.rs`, `validity/gc.rs`. The benchmark
> logs each injected item's state (`usage.status`, migration 0004) and reports the share of
> injected records that were suspect. The gate needs replay tasks (records from an earlier
> session, then the code changes); fresh benchmark stores have no records, so it reads "–".
> Tests: `crates/myrn-engine/tests/history.rs`.

```
Implement the remaining knowledge commands from the CLI table in docs/DESIGN.md:
- log: episodes, decisions and learned records over time (filters by type/status).
- show <id>: a record with evidence and relations.
- trace <id>: where it came from (session, commit, test).
- diff [from] [to]: how knowledge changed between two points (git refs or dates).
- forget <id>: permanently remove a record, its index entries and anything derived only from
  it, after confirmation.
- gc: merge duplicates, drop index entries for deleted symbols, compact old episodes into
  their distilled records. Never removes an earned record unless asked.
Then add the stale-injection metric to the benchmark (share of injected records that were
wrong for the current code) as the Phase 3 gate, and run it.
```

## Phase 4 — Learning loop

### Step 20 — Usage-driven ranking and optional embeddings

> **Status (2026-11-22): built; gate not run.** Used detection (`learn.rs`): an injected
> item is used when the agent then reads or edits its file (or the file of the code a
> record is anchored to), asks `explain`/`impact`/`recall` about it, or cites its id;
> sessions attach their outcome at each Stop. Learned ranking (`[ranking] mode = "learned"`,
> default; `"fixed"` is the switch back): a Beta(1, 2) usefulness factor in 0.5..1.5 from
> ended sessions (use in sessions whose tests passed counts more), and a mild age decay for
> records. Embeddings: `myrn embeddings enable|disable|status` downloads the Model2Vec
> `potion-base-8M` static model (~30 MB, once per machine); `myrn-embed` has the model, a
> localhost helper (token-protected, exits when idle) and a client that falls back to
> in-process loading or to nothing. Vectors live in the same SQLite file (table `vectors`,
> brute-force cosine; sqlite-vec was not needed at this size) and seed retrieval only when
> a match stands clearly above that task's background similarity. Benchmark: `fixed`
> condition, `--shared-store` (one store per repo and condition, so learning carries over),
> `--embeddings`. Tests: `crates/myrn-engine/tests/learning.rs`.

```
Implement Phase 4 from docs/DESIGN.md.

- "Used" detection: after injection, mark an item used if the agent later read/edited its
  anchored symbols, called explain/recall on it, or cited its id. Attach session outcomes.
- Learned ranking: per-record usefulness from usage + outcomes (smoothed, e.g. a Beta prior),
  and tuned weights for relevance/status/freshness/usefulness. Keep the fixed ranking as a
  fallback switch.
- Optional embeddings: a small CPU model downloaded only on `myrn embeddings enable`, served
  by a helper process that starts on first use and exits when idle; vectors in sqlite-vec in
  the same db. Vector search joins the seed step when available; nothing breaks without it.
- Gate: run the benchmark with learned vs fixed ranking and report.
```

## Phase 5 — Sharing through Git

### Step 21 — publish, import and injection safety

> **Status (2026-11-22): done.** `myrn publish <id> [--dry-run] [--yes]` (`sharing.rs`) shows
> the exact file, strips directive-shaped sentences (`myrn_store::notes::directives`: instruction
> overrides, "you must ...", role/tool-call markup, tool-call JSON, `curl | sh`), trims evidence
> to the test command with no absolute paths, adds `published_by` (git `user.name`) and
> `published_at`, takes a private root cause along, and moves the note into
> `.myrn/knowledge/`; it refuses suspect and superseded records. Import needs no command (the
> next refresh indexes pulled notes). Shared records show as verified only when this clone
> verified them itself (`local_verified`, migration 0006, keyed by body hash, kept across
> `rebuild`); `promote` verifies shared records there and never rewrites the committed file.
> The link pass adds `contradicts` edges between disagreeing facts/decisions when one is
> shared, and `myrn status` lists them. Every injected record (packs, explain cards, recall) is
> labelled `shared by X on D` / `private`, `written by <agent> on <date>` (`written_by` and
> `created` are stamped at record time), and shared text is stripped again when shown. Tests:
> `crates/myrn-engine/tests/sharing.rs` (two clones over git).

```
Implement "Sharing across developers" and the injection-safety rules in docs/DESIGN.md.

- myrn publish <id>: show the exact text to be committed, strip directive-shaped text
  ("ignore previous instructions", "you must run...", tool-call-like content) from the body,
  then move the note into .myrn/knowledge/. Only distilled records can be published; raw
  episodes and evidence never leave the machine.
- Import: notes arriving via git pull are indexed on the next invocation, start as
  unverified regardless of their state elsewhere, and are checked by anchor hash.
- Two shared records that disagree get a contradicts edge and show up in status for a person
  to resolve.
- Every injected record is labelled with its source (who/which agent/when).
- End-to-end test: two clones, developer A records and publishes, B pulls, and B's
  context() includes the record.
```

### Step 22 — Distribution

> **Status (2026-11-22): built; first release not cut yet.** Version 0.1.0. `release.yml` on a
> `v*` tag: native builds for the six targets (Linux as static musl), a smoke test, archives,
> `sha256sums.txt`, a GitHub release, and npm / crates.io / Homebrew-tap publishing that each
> run once their secret is set. `dist/render.py` writes the Homebrew formula, Scoop manifest
> and winget manifests from the checksums. `dist/npm` downloads and verifies the binary on
> install or first run (npm 12 skips install scripts). `scripts/install.sh` and `install.ps1`
> verify checksums and install per user. `ci.yml` runs the full suite on all six targets and
> checks the packaging. Not done here (needs the accounts): the secrets, the tap repository,
> the winget submission, and the name check. See `docs/releasing.md`.

```
Set up releases per "Technology choices" and "Platform support" in docs/DESIGN.md:
prebuilt binaries for x86-64 and ARM64 on Linux, macOS and Windows (cargo-dist is fine),
a Homebrew tap, Scoop/winget manifests, `cargo install`, an npm wrapper package that
downloads the right binary, and shell + PowerShell install scripts. CI runs the full test
suite on all targets. Update README with the two-command quick start:
`myrn init` then `myrn connect --all`.
```

---

## Before you start: open questions from the doc

- **Name**: `myrn` is a placeholder; check for name collisions (crates.io, npm, Homebrew) before Step 22.
- **Languages**: the defaults above are Rust, Python and TS/JS. Change Step 3 if you want others.
- **Hook coverage**: how many file edits each agent reports through hooks (as opposed to
  shell commands) is unknown. Step 10/11 should report what it finds.

# Repository structure

A Cargo workspace that builds one binary, `myrn`. Crates map to the layers in
the design doc: adapters → core engine → stores.

```
.
├── Cargo.toml                 workspace
├── crates/
│   ├── myrn/                  CLI binary: subcommands, `hook` and `mcp` entry points
│   ├── myrn-model/            pure types: nodes, closed relation vocabulary, record states, events
│   ├── myrn-store/            SQLite graph + FTS + vectors, Markdown notes, paths, episode logs
│   │   └── migrations/        SQL schema
│   ├── myrn-engine/           the six core components + the single engine API
│   │   ├── indexer/           Tree-sitter, body hashes, Git history, optional SCIP
│   │   ├── capture/           episodes, outcomes, secret redaction, consent
│   │   ├── distill/           record checks, dedupe, contradictions
│   │   ├── validity/          hash anchors, state transitions, gc
│   │   ├── retrieve/          seed → walk → rank
│   │   └── pack/              budgets, cards, rendering, injection log
│   ├── myrn-mcp/              MCP server over stdio, one module per tool
│   ├── myrn-adapters/         Claude Code, Codex, pull-only; hook config templates
│   └── myrn-embed/            optional embedding helper (off the hot path)
├── bench/                     paired with/without benchmark (Evaluation section)
├── tests/fixtures/            sample repos, recorded hook payloads, sample notes
├── dist/                      render.py (Homebrew, Scoop, winget manifests per release), npm/ wrapper
├── scripts/                   install.sh, install.ps1
├── docs/design/               architecture and design document
└── .github/workflows/         ci.yml: tests on x86-64 and ARM64 for Linux, macOS, Windows;
                               release.yml: binaries, checksums, manifests, publishing
```

## Dependency direction

```
myrn (CLI) ──► myrn-adapters ──► myrn-engine ──► myrn-store ──► myrn-model
          └──► myrn-mcp ───────►      │
                                      └──► myrn-embed (optional)
```

Only `myrn-engine` talks to `myrn-store`. Adapters, MCP tools and the CLI all
go through `myrn_engine::api`.

## Third-party crates

| Concern | Crate | Notes |
|---|---|---|
| CLI parsing | `clap` | |
| SQLite | `rusqlite` (bundled) | FTS5 included; no system SQLite needed |
| Parsing | `tree-sitter` + `tree-sitter-{rust,python,javascript,typescript}` | |
| Git | `git2` (vendored libgit2, no network features) | Chosen over `gix` for its simpler diff and rename-detection API |
| Hashing | `blake3` | Content and body hashes |
| Config | `serde`, `toml` | |
| Data dirs | `directories` | |
| Ignore rules | `ignore` | `.gitignore` plus `.myrn/ignore` |
| MCP | none | JSON-RPC over stdio is small enough to write directly (`myrn-mcp/src/server.rs`) |

Also used: `regex` (secret redaction) and, in `bench/` only, `serde_yaml` for task files.
Not added yet: `sqlite-vec` and an embedding model (optional, later phases).

## Build order (from the roadmap)

**Phase 0 (steps 1–9)**, mapped to crates. Done; Gate 0 passed ([gate0.md](gate0.md)).

| Step | Work | Where |
|---|---|---|
| 1 | Project setup | workspace, CI |
| 2 | SQLite store | `myrn-store` (`db`, `graph`, migrations) |
| 3 | Tree-sitter code indexer | `myrn-engine::indexer` (`walk`, `parse`, `symbols`, `body_hash`) |
| 4 | Call, import and test links | `myrn-engine::indexer::symbols` |
| 5 | Git history | `myrn-engine::indexer::git` |
| 6 | `init`, `status`, `show`, `rebuild`, `doctor` | `myrn::cmd` |
| 7 | Search | `myrn-store::fts`, `myrn-engine::retrieve` |
| 8 | Context packing | `myrn-engine::pack` |
| 9 | MCP server | `myrn-mcp` (`explain`, `impact`, `context` first) |
| Gate 0 | Are `explain` and `impact` answers correct? | |

**Phase 1 (steps 10–13)**: built; the Gate 1 run is pending ([gate1.md](gate1.md)).

| Step | Work | Where |
|---|---|---|
| 10 | Adapter contract, Claude Code adapter, `connect`, hooks | `myrn-adapters`, `myrn::cmd::{connect,hook}` |
| 11 | Codex adapter, pull-only mode | `myrn-adapters::{codex,pull_only}` |
| 12 | Episode capture, secret redaction, `myrn log` | `myrn-engine::capture`, `myrn-store::episodes`, `myrn::cmd::log` |
| 13 | Paired benchmark | `bench/` ([README](../bench/README.md)) |

**Later phases**: `distill` and the record tool; `validity`; usage-driven
ranking; `publish` and sharing.

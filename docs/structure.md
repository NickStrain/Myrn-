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
├── dist/                      Homebrew, Scoop, winget, npm wrapper packaging
├── scripts/                   install.sh, install.ps1
├── docs/design/               architecture and design document
└── .github/workflows/ci.yml   fmt, clippy, test on Linux, macOS, Windows
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

Not added yet: `sqlite-vec` and an embedding model (optional, later phases),
`serde_yaml` for note headers (Phase 1).

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

**Later phases** (order to be confirmed): `capture`, `distill`, `validity` and
adapters with hooks; `bench` gate, then automatic injection by default;
`publish` and sharing.

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

## Intended third-party crates

Added to `[workspace.dependencies]` when the phase that needs them starts:

| Concern | Crate (candidate) |
|---|---|
| CLI parsing | `clap` |
| SQLite | `rusqlite` (bundled) |
| Vector search | `sqlite-vec` |
| Parsing | `tree-sitter` + per-language grammars |
| Git | `gix` |
| Hashing | `blake3` |
| Note headers | `serde`, `serde_yaml` |
| Config | `toml` |
| Data dirs | `directories` |
| MCP | `rmcp` |
| Ignore rules | `ignore` |

## Build order (from the roadmap)

1. `myrn-model`, `myrn-store`, `indexer` — code graph and retrieval, no model needed
2. `myrn-mcp` pull tools, `myrn init` / `status` / `show`
3. `capture`, `distill`, `validity`, adapters and hooks
4. `bench` gate, then automatic injection by default
5. `publish` and sharing

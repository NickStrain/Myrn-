# Gate 0: are `explain` and `impact` answers correct?

**Result: passed** (2026-11-16). Phase 1 can start.

Gate 0 was checked two ways: an exact test against a project whose call graph
is known, and hand-checked samples from three real open-source repositories.

## 1. Exact check on a known project

`tests/fixtures/repos/gate0` is one small project written three times, in
Rust, Python and TypeScript: a token store, a session type and a `refresh`
function, each with unit or integration tests. Its full call graph was written
down by hand.

`crates/myrn-engine/tests/gate0.rs` indexes it and compares, for each key
symbol, what `explain` and `impact` report with the expected answer: no
missing link and no extra link is allowed.

| Check | Result |
|---|---|
| `calls` / `called by` for 13 symbols across 3 languages | exact match |
| Direct tests and all tests reaching a symbol (`impact`) | exact match |
| Every call link resolved through scope or imports, none guessed | 26 of 26 |
| Calls that cross languages | 0 |
| Lookup by `a::b`, `A.b`, `path:line`, path; ambiguous names refused | pass |
| Incremental refresh: add, reformat, delete a file; new commit | pass |
| `rebuild` gives the same graph | pass |
| Context pack stays within budget; irrelevant task gets nothing | pass |

Run it with `cargo test -p myrn-engine --test gate0`.

## 2. Real repositories

| Repository | Language | Files | Symbols + tests | Calls (inferred) | `init` | No-op refresh |
|---|---|---|---|---|---|---|
| BurntSushi/ripgrep | Rust | 102 | 3,469 | 4,316 (1,513) | 3.3 s | ~10 ms |
| psf/requests | Python | 37 | 807 | 664 (159) | 1.4 s | ~20 ms |
| sindresorhus/ky | TypeScript | 98 | 2,193 | 1,872 (555) | 5.0 s | ~10 ms |

`init` time is mostly Git history (500 commits are parsed to find the symbols
each one changed). The refresh that runs on every hook or tool call costs
10–20 ms when nothing changed.

Every call link is either **resolved** (it follows from scope, imports, a
local type or `Self`/`self`) or **inferred** (it rests on a name being unique,
or on the receiver's name matching a type). Inferred links are marked `~` in
every answer, so an agent can tell them apart.

Random samples of call sites were checked by hand against the source:

| Sample | Correct |
|---|---|
| Resolved links, all three repositories | 17 of 17 checkable (the sampling script could not find one sample's call site) |
| Inferred links, round 1 (12 per repository) | 32 of 36 |
| Inferred links, round 2, after the first two fixes (10 per repository) | 26 of 30 |
| Inferred links, round 3, after all fixes (ripgrep and ky, 8 each) | 16 of 16 |

The samples are small, so read the last row as "no wrong link seen", not as a
measured rate. Every wrong inferred link had a cause that one of these fixes
now handles:

- A receiver imported from outside the repository (`std::process::Command`,
  `regex_syntax::Parser`) matched a same-named method in the repository. Calls
  through outside imports are now never guessed.
- A known import whose member is not a definition (`ky.create`, assigned at
  runtime) matched a class method. A known receiver without the member now
  gives no link.
- An untyped receiver matched a method of an object literal in an unrelated
  test file. Guesses now need a method of a real type, defined where the caller
  can reach it (same file, imported files two levels deep, or the same crate).
- `super().f()` linked a method to itself. Calls through `super` now give no link.
- A type named in the receiver (`SummaryBuilder::new().stats()`) now takes
  priority over a unique-but-unrelated method name.

Bugs found and fixed while running the gate: Rust paths could resolve into
another language's symbol; JavaScript imports starting with `r`, `b`, `f` or
`u` lost their first letter; `pub use` re-exports were not followed; calls
inside Rust macros (`assert!(f(x))`) were invisible; Python packages below the
repository root (`py/app/…` imported as `app.…`) did not resolve; commits made
in the same second were listed in the wrong order.

## Known limits (accepted for Phase 0)

- **Recall is not measured.** The samples measure whether links that exist are
  right. The ratio of real calls that get no link is not measured yet; it
  needs the benchmark that comes later. Precision was the priority:
  a missing link is safer than a wrong one.
- Dynamic dispatch, trait objects, callbacks and calls through struct fields
  with unknown types are usually left out, or inferred when the method name
  is unique.
- Rust crates whose sources are not under `src/` (ripgrep's binary) get
  shorter qualified names, and their cross-file calls are more often inferred.
- Renamed files: commit history before a rename is attached to the old path.
- The thresholds for precision and stale-injection rate are not fixed yet; the
  design leaves them to the benchmark.

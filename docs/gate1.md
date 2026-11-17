# Gate 1: paired benchmark

**Status: first run stopped at 46 of 96 sessions; result INCONCLUSIVE (break-even).
Injection stays off. Packing fixes are planned as Phase 2's first step (`docs/BUILD_PROMPTS.md`, Step 14), followed by a re-run.**
The gate is fewer total tokens with no drop in task success. Until it passes,
automatic injection stays off by default (`[context] inject = false`).

## What exists

- `bench/` builds `myrn-bench` (`list`, `fetch`, `validate`, `run`, `report`).
  The method is in [bench/README.md](../bench/README.md).
- Task set: 16 tasks on three pinned repositories, one per language:

  | Repository | Language | Bug fixes | Features | Questions | Checks run with |
  |---|---|---|---|---|---|
  | ripgrep | Rust | 2 | 2 | 2 | `cargo test` |
  | more-itertools | Python | 2 | 1 | 2 | `unittest` (no dependencies) |
  | remeda | TypeScript | 2 | 1 | 2 | Node 24 `node --test`, sources run directly (no `npm install`) |

- `myrn-bench validate` passes for all 16. Every bug-fix and feature check
  fails on the prepared workspace and passes with the reference solution,
  without an agent involved.

## First run (stopped early)

Default model, 46 sessions: round 1 complete, round 2 partial. Stopped by
decision once the result was clear. Raw data is in `bench/runs/gate1/` (not committed).

| | Value |
|---|---|
| Paired runs | 23 |
| Token change per task (with − without) | −3.6k ± 23.7k (−2%) |
| Task success, without → with | 96% → 96% |
| Turns | 9.0 → 8.5 |
| Time to passing tests | 46.6 s → 39.5 s |
| Cost per task | $0.189 → $0.190 |
| Items injected per session | 39 |
| Prompt hook latency | median 26 ms, max 44 ms |
| myrn tool calls by the agent | 0 |

The results:
- **No drop in task success.**
- **Large wins where the pack saved searching:** `collapse-levels` −52%, `gitignore-matcher` −22%.
- **Losses where it only added text:** small questions +15–26%, `globset-first-match` +43%.

Why:
- **The pack is padded.** About 39 items per session, including a "most-called"
  brief of test helpers that is irrelevant to every task.
- **It gives locations, not code.** The agent still opens the file.
- **Python test links are missing for `import x as alias` calls,** so cards
  contradict the list ("tests: none found").

Problems found in the harness itself:
- **`glob-is-literal`'s prompt was ambiguous** (case-insensitive globs), so it
  failed in both conditions. The prompt is fixed and the task revalidated.
- **"Reads and searches before first correct edit" read 0.0 without myrn.** The
  agents read with `grep`/`sed -n` and fixed bugs with `sed -i`, and the metric
  only knew the Edit tool. Fixed and rescored from the raw transcripts: **2.5
  without myrn, 2.0 with**. 60 of 76 edits in bug fixes and features were shell
  edits.

## Fixes after the first run

The packer now injects what answers the task, not everything near it:
- **Before:** about 39 items and 800+ tokens for a one-line question, with the
  location of the answer but not the answer.
- **After:** 1–5 items, 400–850 tokens, with the code of the top hit.

Details are in `docs/BUILD_PROMPTS.md`, Step 14. The next run of
`myrn-bench run` measures the effect.

## Smoke run

One question task, one run per condition, `--model haiku`, to check the harness
end to end. It is not evidence for the gate.

| | Without myrn | With myrn |
|---|---|---|
| Total tokens | 52.8k | 42.7k |
| Turns | 3 | 2 |
| Reads and searches | 2 | 1 |
| Answer correct | yes | yes |
| Items injected | – | 35 |
| Prompt hook latency | – | 10 ms |

The report correctly calls this INCONCLUSIVE: one pair has no confidence interval.

## To run the gate

```sh
cargo build --release -p myrn -p myrn-bench
target/release/myrn-bench fetch
target/release/myrn-bench run --runs 3          # 16 tasks x 3 runs x 2 = 96 sessions
```

Then paste the verdict and tables from `bench/runs/<dir>/report.md` here.

## Known limits

- **Small task set**: 16 tasks, five or six per repository. Results per
  repository will have wide intervals; the gate uses all pairs together.
- **No dependencies in the TypeScript and Python workspaces**: the agent can't
  run remeda's vitest suite or install packages, so prompts say so. Both
  conditions have the same limit.
- **Planted bugs are visible in `git show HEAD`**, as a real regression would
  be. Both conditions can see this, but myrn's `history` tool makes it easier to
  find.
- **Question scoring is lenient**: it only checks that required terms appear.
- **Not yet measurable**: repeated-mistake rate and stale-injection rate, which
  need the learned records from Phases 2 and 3.

## Session capture (Step 12)

Measured on ripgrep with a release build:

| Hook | Latency | Why |
|---|---|---|
| Prompt, with injection | ~10 ms | SQLite only |
| File edit | 80–110 ms | Re-parses the edited file to find which symbols changed |

- A comment-only edit is recorded as a file edit with no symbol, because body
  hashes ignore comments.
- Claude Code's `Stop` event fires at the end of every turn, so an episode is
  re-closed each turn. Commits are collected each time and de-duplicated.
- Edits made through shell commands (`sed -i`, scripts) are recorded as
  commands, not as file edits. How common they are is an open question from
  the design doc. `myrn log <session>` shows it for real sessions.

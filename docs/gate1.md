# Gate 1: paired benchmark

> **Partial re-run (2026-11-17), after Step 14–16:** 6 tasks × 3 conditions
> (without myrn / myrn with injection off / injection on) × 2 runs, Opus.
> Result **INCONCLUSIVE**, leaning positive. Details in "Partial run" below and
> `bench/runs/partial-gate/report.md` (not committed). Injection stays off.

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

## Partial run (2026-11-17)

36 sessions with `--model opus` (the default model on this account needed usage credits, so the first attempt was refused by the API and is excluded). Tasks: `collapse-levels`, `windowed-step` (more-itertools), `clamp-zero-limit`, `purry` (remeda), `gitignore-negation-order`, `globset-first-match` (ripgrep). Every session passed its check in every condition.

The metric is **task tokens**: the session total minus myrn's end-of-session recording phase, so it measures what injection does to the task.

| Comparison | All 6 tasks (12 pairs) | Without `clamp-zero-limit` (10 pairs) |
|---|---|---|
| Without → injection on | −12% (−20.8k ± 42.2k) | **−30% (−37.7k ± 17.4k), cheaper in 9 of 10** |
| Without → injection off | −10% (−18.3k ± 21.2k) | −15% (−18.9k ± 18.2k) |
| Injection off → on | −2% (−2.5k ± 39.8k) | −17% (−18.8k ± 28.7k) |

- `clamp-zero-limit` is dominated by noise. The remeda repo ships its own Claude Code Stop hook (`npm … check`), which fails without `npm install` and makes the agent loop about 10 times in every condition (about 400k tokens).
- "Injection off" injects nothing, and the agent made no myrn tool calls, yet it reads 10–15% below "without". That gap is the noise floor for this many pairs. Injection's own effect (off → on) is −17% ± 29% without remeda: positive but not established.
- **Where injection clearly helps:** reads and searches before the first correct edit fall from 2.1 to 0.8. `windowed-step` (a question) costs 19k with injection in both runs, against 44–61k without. Injected items were opened in 54% of cases.
- **Recording:** only 4 of 24 myrn sessions were asked to record. Most bug fixes were made with `sed -i`, which session capture does not see. An asked session spent 77–102k tokens on recording.

Next: shell-edit capture, ignore repositories' own hooks in the harness, a cheaper `record` (several items per call), and more pairs.

## Proof run: protocol (written before the run, 2026-11-17)

Question: does using myrn reduce the tokens an agent spends?

- Conditions: **without myrn**, and **myrn as a user runs it**: `myrn connect` (hooks, recording, the record request at the end), MCP server, `[context] inject = true`.
- All 16 tasks × 3 runs, `--model opus`, paired by (task, run), order alternated per run. Only local settings load, so repositories' own hooks don't run. Shell edits are captured; `record` takes a batch.
- **Claim A (the task):** task tokens (session total minus the API calls after myrn's record request) are lower with myrn. Supported if the 95% interval of the mean paired difference is below zero **and** a two-sided sign test over pairs gives p < 0.05.
- **Claim B (what you pay):** total tokens including recording are lower, by the same tests.
- Either claim fails if task success with myrn is lower than without.
- Both results are reported whatever they show. Runs that fail to run (API errors) are excluded and listed; no other exclusions.

# Haiku benchmark series

A cheap paired benchmark of myrn on Claude Haiku, kept separate from the Opus
Gate 1 runs in [gate1.md](gate1.md). The two series use different models, so
their numbers are not directly comparable.

Method: [bench/README.md](../bench/README.md). Conditions: without myrn vs myrn
with injection on (`--conditions without,with`, the default), `--model haiku`.
Raw data is in `bench/runs/cheap-*` (not committed).

| Stage | Run directory | Sessions | Status |
|---|---|---|---|
| 1. Smoke test | `cheap-smoke` | 2 | Done 2026-11-22 |
| 2. Pilot (16 tasks × 1 run) | `cheap-pilot` | 32 | Done 2026-11-22 |
| 3. Runs 2 and 3 | `cheap-r23` | 64 | Done 2026-11-22 |
| Combined (48 pairs) | `cheap-all` | 96 | Pilot + stage 3, runs renumbered 1–3 |

## Stage 1: smoke test (2026-11-22)

`more-itertools/question/unhashable-everseen`, 1 run: both passed, 55.3k → 40.6k
tokens (−27%), about $0.003 per session. Checks the harness end to end only.

## Stage 2: pilot (2026-11-22)

All 16 tasks × 1 run × 2 conditions = 32 sessions, about $0.20, about 15 minutes.
Release build of commit `16585a9` plus the uncommitted macOS/Windows sharing-test fix.

**Verdict: INCONCLUSIVE.** Tokens fell 3% on average, but the 95% interval
includes no change. Cheaper in 8 of 16 tasks (sign test p = 1.0).

| | Without myrn | myrn, inject on |
|---|---|---|
| Task tokens per task | 170.2k ± 56.2k | 165.4k ± 69.8k |
| Change per task | | −4.7k ± 48.3k (−3%) |
| Task success | 100% | 100% |
| Cost per session (USD) | 0.0064 | 0.0063 |
| Reads and searches before first correct edit | 4.4 | 3.3 |
| Time to passing tests (s) | 25.8 | 29.0 |
| Turns | 7.6 | 7.1 |
| Items injected per run | – | 4.8 |
| Injection use (injected files the agent opened) | – | 42% |
| Prompt hook latency | – | median 38 ms, max 58 ms |

### By task kind and repository

| | Without | With | Change |
|---|---|---|---|
| Questions (6) | 540k | 379k | **−30%** |
| Features (4) | 1,180k | 1,050k | **−11%** |
| Bug fixes (6) | 1,002k | 1,217k | **+21%** |
| more-itertools (5) | 542k | 531k | −2% |
| remeda (5) | 879k | 547k | −38% |
| ripgrep (6) | 1,302k | 1,569k | +21% |

### Per task

| Task | Without | With | Change |
|---|---|---|---|
| `more-itertools/bugfix/collapse-levels` | 102.7k | 133.0k | +29% |
| `more-itertools/bugfix/unique-everseen-key` | 57.3k | 194.0k | +239% |
| `more-itertools/feature/take-every` | 266.5k | 162.2k | −39% |
| `more-itertools/question/unhashable-everseen` | 54.3k | 19.5k | −64% |
| `more-itertools/question/windowed-step` | 61.5k | 22.7k | −63% |
| `remeda/bugfix/chunk-drops-remainder` | 101.1k | 147.0k | +45% |
| `remeda/bugfix/clamp-zero-limit` | 186.1k | 85.3k | −54% |
| `remeda/feature/take-every` | 372.9k | 203.1k | −46% |
| `remeda/question/lazy-pipe` | 109.2k | 45.2k | −59% |
| `remeda/question/purry` | 109.3k | 65.9k | −40% |
| `ripgrep/bugfix/gitignore-negation-order` | 280.0k | 315.2k | +13% |
| `ripgrep/bugfix/glob-multi-dot-extension` | 275.2k | 342.9k | +25% |
| `ripgrep/feature/glob-is-literal` | 319.6k | 507.5k | +59% |
| `ripgrep/feature/globset-first-match` | 220.9k | 177.4k | −20% |
| `ripgrep/question/binary-detection` | 151.1k | 162.8k | +8% |
| `ripgrep/question/gitignore-matcher` | 54.8k | 62.8k | +15% |

### Observations

- **Questions and features gain.** The injected code often answers a question
  outright (up to −64%).
- **Bug fixes lose in 5 of 6.** In `unique-everseen-key` (+239%) the injection
  worked: the agent's first message was "The cause is visible in the context"
  and it went straight to the line. The extra tokens came from work it chose to
  do afterwards: a new regression test and a mistaken `git stash` check. One run
  per task cannot separate this agent variance from injection cost; runs 2 and 3
  should.
- **ripgrep is the weakest repository** (+21%); `glob-is-literal` alone is +188k.
- **No learned records were used.** Each run starts with an empty store, so this
  measures the code map and injection only.

## Stage 3 and combined result: 48 pairs (2026-11-22)

Runs 2 and 3 of every task (64 sessions, about $0.42). Combined with the pilot
in `bench/runs/cheap-all` (pilot = run 1, stage 3 = runs 2 and 3). The whole
series cost about $0.65 for 98 sessions.

**Verdict: INCONCLUSIVE overall, a clear win on questions.** Every one of the 96
sessions passed. Tokens fell 4% on average (−8.7k ± 29.0k per task), cheaper in
25 of 48 pairs.

| | Pairs | Without | With | Change | Mean paired difference (95% CI) | Cheaper in |
|---|---|---|---|---|---|---|
| **All** | 48 | 9,576k | 9,159k | −4% | −8.7k ± 29.0k | 25/48 |
| **Questions** | 18 | 1,764k | 1,048k | **−41%** | **−39.8k ± 17.1k** (excludes zero) | 14/18 |
| Bug fixes | 18 | 4,027k | 4,065k | +1% | +2.1k ± 38.9k | 5/18 |
| Features | 12 | 3,786k | 4,046k | +7% | +21.7k ± 96.0k | 6/12 |
| more-itertools | 15 | 1,811k | 1,977k | +9% | +11.1k ± 36.0k | 8/15 |
| remeda | 15 | 2,702k | 2,205k | −18% | −33.1k ± 67.4k | 11/15 |
| ripgrep | 18 | 5,064k | 4,976k | −2% | −4.9k ± 44.4k | 6/18 |

Other metrics (all 48 pairs): reads and searches before the first correct edit
4.9 → 3.8; turns 8.5 → 8.1; time to passing tests 29.3 s → 33.7 s; output tokens
3.8k → 4.1k; injection use 46% of injected items opened.

### Per task (change with myrn, runs 1–3)

| Task | Run 1 | Run 2 | Run 3 | Total |
|---|---|---|---|---|
| `more-itertools/bugfix/collapse-levels` | +29% | +14% | +10% | +16% |
| `more-itertools/bugfix/unique-everseen-key` | +239% | −34% | +21% | +50% |
| `more-itertools/feature/take-every` | −39% | +35% | +57% | +18% |
| `more-itertools/question/unhashable-everseen` | −64% | −64% | −65% | −64% |
| `more-itertools/question/windowed-step` | −63% | −38% | −43% | −50% |
| `remeda/bugfix/chunk-drops-remainder` | +45% | −66% | +24% | −16% |
| `remeda/bugfix/clamp-zero-limit` | −54% | +26% | −23% | −21% |
| `remeda/feature/take-every` | −46% | −26% | +169% | +10% |
| `remeda/question/lazy-pipe` | −59% | −64% | −71% | −65% |
| `remeda/question/purry` | −40% | −53% | −44% | −46% |
| `ripgrep/bugfix/gitignore-negation-order` | +13% | +8% | +5% | +8% |
| `ripgrep/bugfix/glob-multi-dot-extension` | +25% | −38% | +15% | −6% |
| `ripgrep/feature/glob-is-literal` | +59% | +25% | −25% | +13% |
| `ripgrep/feature/globset-first-match` | −20% | +16% | −42% | −19% |
| `ripgrep/question/binary-detection` | +8% | −46% | −32% | −26% |
| `ripgrep/question/gitignore-matcher` | +15% | +14% | +15% | +15% |

### Findings

- **Questions are a real saving.** −41%, and the 95% interval is well below zero.
  Five of six question tasks win every run; the injected code answers them.
- **`gitignore-matcher` loses consistently (+15% every run).** The answer is
  cheap to find, so the injected context is pure overhead (about 8k tokens).
- **Bug fixes break even.** The pilot's "+21% on bug fixes" was mostly noise:
  single tasks swing from −66% to +45% between runs. `collapse-levels` and
  `gitignore-negation-order` lose by a small margin every run.
- **Features are the noisiest category** and decide the overall total. In the
  worst session (`remeda/feature/take-every` run 3, 610.6k vs 227.3k) Haiku with
  myrn built the function on remeda's `purry3` data-first/data-last machinery,
  hit an optional-argument bug, and spent about 10 extra turns fixing it; without
  myrn it copied a simple neighbouring utility. The project map describes remeda
  as data-first/data-last and `internal/` as that machinery, which may have
  pointed it there (a lead, not proven).
- **Fewer reads, not fewer tokens, on edits.** myrn cuts exploration before the
  first edit by about a fifth, but Haiku often spends the saving on extra
  verification or a more elaborate implementation.
- **Haiku variance is large.** The same task without myrn varies up to 2× between
  runs, so per-task conclusions need more than three pairs.

## Stage 4: targeted bug-fix injection (2026-11-22, recorded 2026-11-23)

Changes under test (uncommitted at the time):
- Code-map notes for each function gain `## Behaviour` (concrete input → result,
  edge cases) and `## Terms` (words a bug report would use), written by Haiku in
  `myrn summarize` (map version 2). Terms get the most weight in the summary search.
- Bug reports are detected. Summary-search order boosts the seed score, and
  two-letter words inside backticks (`gz`) count.
- Code is shown only when myrn is confident (task names it, or it leads the
  best match in another file by 1.5×). A confident bug report gets 3 items:
  the culprit's code and two one-line pointers.

**Free retrieval check** (`myrn-bench retrieval`, no agent): right file ranked first
for 6/10 code tasks before and 7/10 after. Mean injected pack 706 → 434 tokens (−39%).
Answer terms in the pack 13/16 → 12/16.

**Haiku run** (`cheap-v2`): 6 bug fixes + 2 ripgrep questions × 3 runs, 44 of 48
sessions (the last two question pairs did not run), about $0.25. Every session passed.

| Bug fixes (18 pairs) | Without | With | Change | Mean paired difference (95% CI) | Cheaper in |
|---|---|---|---|---|---|
| Before (stage 3) | 4,027k | 4,065k | +1% | +2.1k ± 38.1k | 5/18 |
| **After (stage 4)** | 3,947k | 3,269k | **−17%** | −37.7k ± 39.1k | 11/18 |

Reads and searches before the first correct edit: 5.2 → 2.9 (stage 3: 4.9 → 3.8).

| Bug fix task | Before | After |
|---|---|---|
| `more-itertools/bugfix/collapse-levels` | +16% | −7% |
| `more-itertools/bugfix/unique-everseen-key` | +50% | +36% |
| `remeda/bugfix/chunk-drops-remainder` | −16% | −33% |
| `remeda/bugfix/clamp-zero-limit` | −21% | −34% |
| `ripgrep/bugfix/gitignore-negation-order` | +8% | −7% |
| `ripgrep/bugfix/glob-multi-dot-extension` | −6% | −30% |

**Verdict: bug fixes move from break-even to about −17% with no loss of success.**
The interval barely touches zero (−37.7k ± 39.1k), so this is strong but not yet
conclusive. `unique-everseen-key` still loses. `gitignore-negation-order` is still
missed by retrieval yet improved. `gitignore-matcher` still costs about +10%.

## Stage 5: weak-spot fixes (2026-11-23)

Run `cheap-v3`: 4 tasks × 3 runs × 2 conditions = 24 sessions, $0.075, all passed.

Two fixes were tried:
1. **Shorter MCP tool descriptions (reverted).** Claude Code defers MCP tools
   behind ToolSearch, so their descriptions never reach the model's context. The
   first call stayed at 18,428 tokens with myrn either way. The real fixed overhead
   (about 1.4k tokens per call vs without) is the SessionStart project map
   (~2k characters) plus the prompt's injected context.
2. **No test pointers for bug reports (kept).** "Make sure the tests still pass"
   no longer counts as asking about tests, and bug-report cards drop the
   `tests` row. In stage 4 the agent opened the test file in 3/3 myrn runs of
   `unique-everseen-key`; now it went straight to the edit in 2/3.

| Task | Stage 3 | Stage 4 | Stage 5 | Stage 5 per run |
|---|---|---|---|---|
| `unique-everseen-key` | +50% | +36% | **−21%** | +77%, −56%, −29% |
| `gitignore-matcher` | +15% | +10% | −20% | +12%, −37%, −21% |
| `clamp-zero-limit` (control) | −21% | −34% | −21% | −10%, −25%, −31% |
| `windowed-step` (control) | −50% | – | +20% | +57%, −33%, +67% |

`gitignore-matcher`'s gain is noise: its context with myrn was unchanged and the
runs without myrn happened to be longer (74k vs 53k). The `windowed-step` control
swung from −50% to +20% with no relevant change. Three runs cannot settle effects
this small.

`gitignore-negation-order` is still missed by retrieval: the buggy function ranks
56th in summary search because the report's common words swamp the rare ones.
Weighting words by rarity (prototype) did not fix it (rank 5 → 7) but helped
`glob-is-literal` (2 → 1) and `chunk-drops-remainder` (42 → 10).

## Stage 6: fresh full series with all changes (2026-11-23)

All 16 tasks × 3 runs × 2 conditions with the current build: Behaviour/Terms
map, targeted bug context, no test pointers for bug reports, original MCP tool
descriptions. Run in 9 chunks (one repository × one run each) into
`bench/runs/cheap-s6` with `--first-run`. Compared with the same tasks and runs
from stages 1–3 (`cheap-all`).

| Chunk | Repository, run | Sessions | Passed | Cost | Change with myrn | Same pairs, stages 1–3 |
|---|---|---|---|---|---|---|
| 1 | more-itertools, run 1 | 10 | 10 | $0.052 | +0% | −2% |
| 2 | remeda, run 1 | 10 | 10 | $0.069 | −8% | −38% |
| 3 | ripgrep, run 1 | 12 | 12 | $0.103 | −28% | +21% |
| **Run 1 total** | all 16 tasks | 32 | 32 | $0.224 | **−17%** (cheaper in 10/16) | −3% (8/16) |
| 4 | more-itertools, run 2 | 10 | 9 (one fail without myrn) | $0.053 | +37% | +8% |
| 5 | remeda, run 2 | 10 | 10 | $0.053 | +18% | −35% |
| 6 | ripgrep, run 2 | 12 | 12 | $0.090 | −6% | −8% |
| **Run 2 total** | all 16 tasks | 32 | 31 | $0.197 | **+9%** (cheaper in 8/16) | −13% (9/16) |
| **Runs 1–2** | 32 pairs | 64 | 63 | $0.421 | −5% (18/32) | −8% (17/32) |
| 7 | more-itertools, run 3 | 10 | 10 | $0.042 | −7% | +21% |
| 8 | remeda, run 3 | 10 | 10 | $0.060 | +0% | +26% |
| 9 | ripgrep, run 3 | 12 | 12 | $0.085 | +18% | −11% |
| **Run 3 total** | all 16 tasks | 32 | 32 | $0.187 | +7% (7/16) | +3% (8/16) |

**Verdict: INCONCLUSIVE, and no better than stages 1–3.** 96 sessions, 95 passed
(the failure was without myrn), $0.61. Tokens −1% overall (−2.1k ± 30.1k per pair),
cheaper in 25 of 48 pairs; the same pairs in stages 1–3 were −4%.

| 48 pairs | Stage 6 | Stages 1–3 |
|---|---|---|
| **All** | −1% (25/48) | −4% (25/48) |
| Questions | −26% (10/18) | −41% (14/18) |
| Bug fixes | +1% (9/18) | +1% (5/18) |
| Features | +6% (6/12) | +7% (6/12) |
| more-itertools | +11% | +9% |
| remeda | +2% | −18% |
| ripgrep | **−7%** | −2% |

Reads and searches before the first correct edit 4.5 → 4.0; turns 9.0 → 8.1.

| Task | Stage 6 (3 runs) | Stages 1–3 |
|---|---|---|
| `more-itertools/bugfix/collapse-levels` | +3% | +16% |
| `more-itertools/bugfix/unique-everseen-key` | +27% | +50% |
| `more-itertools/feature/take-every` | +20% | +18% |
| `more-itertools/question/unhashable-everseen` | −53% | −64% |
| `more-itertools/question/windowed-step` | +21% | −50% |
| `remeda/bugfix/chunk-drops-remainder` | −21% | −16% |
| `remeda/bugfix/clamp-zero-limit` | −30% | −21% |
| `remeda/feature/take-every` | +65% | +10% |
| `remeda/question/lazy-pipe` | −62% | −65% |
| `remeda/question/purry` | −30% | −46% |
| `ripgrep/bugfix/gitignore-negation-order` | +37% | +8% |
| `ripgrep/bugfix/glob-multi-dot-extension` | −23% | −6% |
| `ripgrep/feature/glob-is-literal` | −24% | +13% |
| `ripgrep/feature/globset-first-match` | −28% | −19% |
| `ripgrep/question/binary-detection` | +2% | −26% |
| `ripgrep/question/gitignore-matcher` | +10% | +15% |

What this means:
- **Stage 4's bug-fix gain (−17%) did not replicate**: +1% here, the same as
  stages 1–3. It was mostly run-to-run variance. Bug fixes are cheaper in more
  pairs (9/18 vs 5/18), but the losses are larger.
- **ripgrep improved** (−7% vs −2%): the Behaviour/Terms map finds the glob
  functions (`glob-is-literal` +13% → −24%, `glob-multi-dot-extension` −6% → −23%).
- **Two regressions explain most of the rest:** `windowed-step` (−50% → +21%, the
  stub bug, fixed after this series) and `remeda/feature/take-every`
  (+10% → +65%, unrelated context sends the agent into remeda's internals).
- `gitignore-negation-order` (+37%) is still missed by retrieval.


Findings while running (fixes kept out of the release build until the series ends):
- `more-itertools/question/windowed-step` lost its saving (+2%, +63%): the task
  names `windowed`, but the `.pyi` overload `windowed#2` has the same name, so
  myrn treated the name as ambiguous, showed no code, and the agent read all of
  `more.py`. Fixed in `drop_shadowed_stubs` (overload suffix ignored); the free
  retrieval check now shows the code for this task only (845 tokens, 4 items).
- `remeda/feature/take-every` loses every run (+64%, +111%). The injected context
  is unrelated (an internal error helper, `swapArray`, lint and docs configs);
  with myrn the agent studies remeda's internals (`purryByArity`, the
  `remeda-utility` skill docs), without it copies `take.ts`. Lead: drop
  low-relevance packs entirely.

## Next

- Shrink the SessionStart project map (the real per-call overhead), then check on
  short questions.
- Trim injection for small questions where the answer is cheap to find (the
  `gitignore-matcher` pattern).
- Check whether the project map's wording steers feature work toward complex
  internals.
- Optionally confirm on Opus with a few tasks; Haiku numbers are not comparable
  with `gate1.md`.

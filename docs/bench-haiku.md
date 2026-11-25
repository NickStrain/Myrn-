# Haiku benchmark series

A cheap paired benchmark of myrn on Claude Haiku, kept separate from the Opus
Gate 1 runs in [gate1.md](gate1.md). The two series use different models, so
their numbers are not directly comparable.

Method: [bench/README.md](../bench/README.md), `--model haiku`. Each pair runs
the same task twice, without myrn and with it; tokens are input + cache + output
for the whole session; success is the task's hidden check. Every stage's
per-session results are kept in [`bench/results/`](../bench/results/) (the full
transcripts and checkouts stay in `bench/runs/`, not committed).

## Current result (2026-11-25)

myrn as shipped (Arctic Embed embeddings, the use-context hint in CLAUDE.md,
targeted context) against no myrn, on Claude Haiku:

| Task kind | Pairs | Tokens | Billed cost | Cheaper in | Passed (without → with) | Source |
|---|---|---|---|---|---|---|
| Questions | 18 | **−35%** | −26% | 13/18 | 18 → 18 | Stage 8 |
| Bug fixes | 18 | **−19%** | −19% | 14/18 | 17 → 18 | Stage 8 |
| Features | 44 | **−10%** | −8% | 26/44 | 39 → 40 | Stages 8 + 10 pooled |
| All 16 original tasks | 48 | −10% | −11% | 31/48 | 47 → 48 | Stage 8 |

Question and bug-fix savings are clearly real (95% interval below zero). For
features the same myrn measured +10% in Stage 8 (4 tasks, 12 pairs) and −17% in
Stage 10 (16 tasks, 32 pairs; the same 4 tasks alone −31%), so single feature
sessions are very noisy; pooled, −10% (−39k ± 43k per pair, sign test p = 0.29):
likely a saving, not yet proven. The overall Stage 8 figure just touches zero.
myrn never passed fewer tasks than no myrn.

## Every benchmark run

| Stage | Date | Run directory | What it tested | Sessions | Cost | Result |
|---|---|---|---|---|---|---|
| 1 | 11-22 | `cheap-smoke` | Harness check, 1 task | 2 | $0.01 | ran end to end |
| 2 | 11-22 | `cheap-pilot` | Original myrn, 16 tasks × 1 | 32 | $0.20 | −3%, inconclusive |
| 3 | 11-22 | `cheap-r23` (+ pilot = `cheap-all`) | Runs 2–3 | 64 | $0.42 | 48 pairs: −4%; questions −41%, bug fixes +1%, features +7% |
| 4 | 11-22 | `cheap-v2` | Behaviour/Terms map, targeted bug context; 8 tasks × 3 | 44 | $0.25 | bug fixes −17% (not replicated in Stage 6) |
| 5 | 11-23 | `cheap-v3` | No test pointers in bug context; 4 tasks × 3 | 24 | $0.08 | `unique-everseen-key` +36% → −21% |
| 6 | 11-23 | `cheap-s6` | Full re-run of stages 4–5 | 96 | $0.61 | −1% overall; regressions found (stub bug, junk pointers) |
| 7 | 11-24 | `cheap-s7` | Fixes + use-context hint; 8 hard tasks × 3 × 3 conditions | 72 | $0.47 | myrn +3%, myrn + hint −8% |
| **8** | 11-24 | `cheap-s8` | **myrn as shipped**: embeddings, hint, all fixes; 16 × 3 | 96 | $0.58 | **−10%**; questions −35%, bug fixes −19%, features +10% |
| 9 | 11-25 | `cheap-s9-old`, `cheap-s9-new` (+ `-broken`) | File checklist + narrow hint on 16 feature tasks × 1 | 65 | $0.76 | checklist +25% (agents widened the job); not continued |
| 9b | 11-25 | `cheap-s9-sel` | No context for new functions; 16 feature tasks × 1 | 16 | $0.23 | +58%, untrusted (not interleaved) |
| **10** | 11-25 | `cheap-s10` | Hint wording, interleaved; 16 feature tasks × 2 × 3 conditions | 96 | $1.20 | **Stage 8 hint −17% on features**; narrow hint +19% |
| | | | **Total** | **607** | **$4.81** | |

The Opus series (Gate 1) is separate: [gate1.md](gate1.md).

## Free tests (no agent sessions)

- **Retrieval check** (`myrn-bench retrieval`): what myrn would inject for each
  task and whether the right file comes first. Runs `bench/runs/retrieval-*`.
- **Ranking test from git history**: 275 commit subjects as questions, the
  functions they changed as answers. Arctic Embed took right-function-in-top-5
  from 48% to 57% (section below).
- **Feature-task validation**: the 12 generated feature tasks fail before and
  pass with their reference solution.

## Tried and dropped

| Idea | Why dropped | Where |
|---|---|---|
| MCP server off by default | Saved no tokens in Claude Code (tools are deferred); turned back on | Stage 7 |
| Rank-based embedding scores | No better than the shipped scoring | Ranking test |
| "How code is added here" file checklist | Agents updated every listed file: +34% billed on features. Kept as an option, off by default (`[context] addition_checklist`) | Stage 9 |
| No context for any new-function request | Untrusted single run; not needed once Stage 10 showed Stage 8 already saves | Stage 9b |
| Narrow hint ("when you edit that code, read only its range") | +43% tokens against the Stage 8 wording (p = 0.01); the Stage 8 hint stays | Stage 10 |

Lessons: compare only conditions run back to back (interleaved); a single
feature session can vary 2–3× by chance, so judge features on several runs;
small spot checks (Stages 4, 5, 7) overstated gains that full series did not
confirm.

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

## Stage 6 analysis: where the tokens go (2026-11-24)

Method: every API call re-reads the whole conversation, so a session's tokens
are the sum over calls of that call's context, plus output. Split per session
(script kept outside the repo): first-call context × calls, each tool result ×
the calls after it, and the rest (the agent's own messages re-read, and output).

| 48 pairs | Without | With myrn |
|---|---|---|
| First-call context × calls | 68% | 68% |
| Tool results re-read | 15% | 15% |
| Own messages and output | 17% | 17% |
| Calls per session | 7.6 | **6.9** |
| First-call context | 17.3k | **18.8k** |

**The whole result in one line:** myrn's information saves about 9% of calls,
and its fixed overhead adds about 9% to every call, so they cancel (−1%).

The +1.5k per call is:
- **~590 tokens, first attributed to the MCP server registration.** myrn's tools
  were called **0 times in 48 sessions**. Correction (Stage 7): turning the server
  off did not shrink the first call; this part is hook wrapper text (~350, fixed)
  and the prompt pack being ~25% larger in real tokens than myrn's estimate.
- **~490 tokens: the SessionStart project map** (about 1,800 characters).
- **~430 tokens: the prompt's injected context** (the part that saves calls).

Worst pairs, decomposed:

| Pair | Without → with | Extra calls | Cause |
|---|---|---|---|
| `remeda/feature/take-every` #2 | 263k → 558k | +5 | The pack's pointers are unrelated (`swapArray`, an error helper, configs). The agent's first command opened `swapIndices.ts`, modelled its work on it, then read remeda's `.agents/skills` docs (3 of 3 runs with myrn, 0 of 3 without). |
| `ripgrep/bugfix/gitignore-negation-order` #3 | 276k → 509k | +7 | Five pointers into `dir.rs`, none to the buggy `gitignore.rs`; the agent searched more and read `gitignore.rs` whole. |
| `more-itertools/feature/take-every` #2 | 361k → 572k | +4 | More exploration of tests and docs; no single cause. |
| `ripgrep/question/binary-detection` #3 | 111k → 181k | +2 | Its pack shrank from 723 to 175 tokens (the confidence rule dropped the code), so the answer was no longer in it. |
| `more-itertools/bugfix/collapse-levels` #1 | 104k → 162k | +2 | Overhead only: no extra reading. |

Questions fell from −41% to −26% mostly because the confidence rule (made for bug
reports) also took code out of question packs: `binary-detection` 723 → 175
tokens (−26% → +2%), `gitignore-matcher` 891 → 272, plus the `.pyi` stub bug on
`windowed-step` (−50% → +21%).

Tool results: `cat`/`sed -n` dumps are 11% of all tokens (380 results, 5.6k each
on average); `Read` 2%.

Improvements, with estimates from this data (each still needs a benchmark):

| Change | Quality risk | Estimated effect |
|---|---|---|
| Don't register the MCP server unless asked (or only when recording) | None measured: never used | Done; no measurable token saving in Claude Code (Stage 7) |
| Drop the SessionStart map, or put only the relevant directory line in the prompt pack | Possible: may orient the agent | −490 tokens per call, about −2% |
| Questions keep the top match's code even when not confident (rule stays for bug reports) | Low: restores stage 1–3 behaviour | Questions back towards −40% (about −2.5% overall) |
| No pointers when nothing scores well (feature tasks especially) | Medium: pointers help when right (`glob-multi-dot-extension` −23%) | The two worst tasks lost 1.08M tokens (about 11% of all) |
| Stub fix (`windowed-step`) | None | Already done |

## Changes after Stage 6 (2026-11-24, not yet benchmarked)

1. **MCP server off by default with hooks.** `myrn connect` registers it only
   with `--mcp` (pull-only mode always has it); reconnecting without `--mcp`
   removes an old registration. The end-of-session record request then asks for
   one `myrn record` shell command (new: JSON on stdin, same checks as the tool)
   instead of the `record` tool. Expected: about −590 tokens per API call.
2. **Questions keep the best match's code** even when myrn is not confident;
   the confidence rule stays for bug reports and changes.
5. **Stub overloads** (`windowed#2` in a `.pyi`) no longer make a named function
   look ambiguous.

Free retrieval check (`bench/runs/retrieval-fix125`): only the three intended
question packs changed; every bug-fix and feature pack is identical to stage 6.

| Question | Stages 1–3 | Stage 6 | Now |
|---|---|---|---|
| `windowed-step` | 839 tokens, 3/3 terms | 275, 3/3 (no code) | 845, 3/3 (code) |
| `binary-detection` | 723, 1/3 | 175, 0/3 | 277, 1/3 (only the 3-line getter's code) |
| `gitignore-matcher` | 891, 2/3 | 272, 2/3 | 590, 2/3 (`GitignoreBuilder::build`) |

Right file first: 7 of 10 code tasks (unchanged). Answer terms in the pack:
13 of 16 (stage 6: 12). The MCP saving cannot show here; it needs agent sessions.

## Ranking test from git history (2026-11-24, no agent sessions)

275 questions from the three repositories' own commits (120 more-itertools,
35 remeda, 120 ripgrep): the commit subject is the question, and the functions
named in its diff hunks, in the source files it changed, are the answer. Commits
touching 1–3 source files with a descriptive subject only. Scripts outside the
repo (`evalset.py`, `rank_eval.py`).

| Method | Function first | Function in top 5 | File in top 5 | MRR |
|---|---|---|---|---|
| myrn now (embeddings off, the default) | 32% | 48% | 60% | 0.378 |
| **myrn with its embeddings on** (`potion-base-8M`, as shipped) | **35%** | **51%** | **67%** | **0.409** |
| Embeddings alone: `potion-base-8M` | 23% | 40% | 67% | 0.310 |
| Embeddings alone: `potion-retrieval-32M` | 27% | 43% | 72% | 0.345 |
| Embeddings alone: `bge-small-en-v1.5` | 29% | 49% | 74% | 0.375 |
| Embeddings alone: `snowflake-arctic-embed-xs` | 29% | 49% | 72% | 0.374 |
| myrn list + embedding list, fused (offline, best: `bge-small`) | 38% | 56% | 78% | 0.454 |

Findings:
- **Turning on the embeddings myrn already has improves every measure** (+3
  points function top 5, +7 file top 5). A bigger model adds only about 3 more
  points even when fused, and would need an ONNX runtime in myrn.
- Adding Behaviour and Terms to the embedded text does not help.
- Scoring vector matches by rank instead of similarity was tried and was no
  better than the shipped scoring; reverted.
- Embeddings do not fix the two benchmark misses (`remeda/feature/take-every`,
  `gitignore-negation-order`); the 16 benchmark packs barely change with them.

## Context wording (2026-11-24)

Code cards now say `shown: complete, lines A-B, as the file is now`, and for bug
fixes and changes (not questions) `to edit: Read <path> with offset A and limit N
covers exactly this code`. Retrieval unchanged (7/10, 13/16); packs +23 tokens.

## Stage 7: overhead off, question code, wording, and a use-context hint (2026-11-24)

8 tasks × 3 runs × 3 conditions: without myrn, myrn (MCP off, question code, stub
fix, new wording), and myrn plus a one-line instruction in the system prompt
(standing in for a CLAUDE.md line; the benchmark does not load project
CLAUDE.md files):

> This repository uses myrn. When a prompt has a "Myrn context" block, its code
> is the files' current content: use it instead of opening those files again. When
> you must open a file (to edit it, or for something not shown), read only the
> line range you need (Read with offset and limit), not the whole file.

Tasks: `windowed-step`, `binary-detection`, `gitignore-matcher`,
`unique-everseen-key`, `collapse-levels`, `remeda/feature/take-every`,
`gitignore-negation-order`, `glob-multi-dot-extension`. Run: `bench/runs/cheap-s7`.

72 sessions, $0.47. Passed: without 23/24, myrn 23/24 (`collapse-levels` run 3),
myrn + hint 24/24.

| 24 pairs (8 hard tasks × 3 runs) | Change vs without myrn | Mean per pair (95% CI) | Cheaper in |
|---|---|---|---|
| Stage 6 version, same tasks and runs | +23% | +40.9k ± 40.8k | 6/24 |
| **myrn now** | **+3%** | +5.9k ± 34.3k | 12/24 |
| **myrn now + hint** | **−8%** | −16.5k ± 37.8k | 17/24 |

Reads and searches before the first correct edit: without 4.7, myrn 4.4,
myrn + hint **2.9**. Turns 8.8, 8.4, 7.8.

| Task | Stage 6 | Now | Now + hint | Calls (without, myrn, hint) |
|---|---|---|---|---|
| `collapse-levels` | +3% | −6% | +6% | 6.3, 5.7, 6.3 |
| `unique-everseen-key` | +27% | **−34%** | −27% | 5.0, 3.3, 3.7 |
| `windowed-step` | +21% | −26% | **−40%** | 2.0, 1.3, 1.0 |
| `remeda/feature/take-every` | +65% | +30% | +52% | 9.3, 10.3, 11.7 |
| `gitignore-negation-order` | +37% | −3% | **−27%** | 18.7, 17.7, 14.0 |
| `glob-multi-dot-extension` | −23% | +12% | **−28%** | 11.7, 13.7, 9.3 |
| `binary-detection` | +2% | −9% | −21% | 6.0, 5.7, 5.0 |
| `gitignore-matcher` | +10% | −1% | −1% | 3.3, 3.0, 3.0 |

Findings:
- **The fixes remove the regressions**: these 8 tasks went from +23% to +3%
  (the stub fix and question code on `windowed-step` and `binary-detection`, no
  test pointers and the edit range on `unique-everseen-key`).
- **The hint is the biggest single lever**: −8% vs without and −11% vs myrn
  alone, mostly through fewer calls on long tasks (`gitignore-negation-order`
  18.7 → 14.0) and a third fewer reads before the first edit. Not yet conclusive
  (the interval includes zero), and it makes `remeda/feature/take-every` worse.
- `remeda/feature/take-every` is still the main loss: the unrelated pointers.
- Turning the MCP server off saved no tokens (see the correction above).
- These are the 8 hardest tasks, so the numbers are not the overall figure; a
  full 16-task series is needed for that.

## Changes after Stage 7 (2026-11-24)

- **MCP server back on by default** (`myrn connect --no-mcp` leaves it out).
- **`myrn connect` writes the use-context hint** into CLAUDE.md (Claude Code)
  or AGENTS.md (Codex) as a marked block; `disconnect` removes it.
- **No context for a request to add a new function** (named in backticks, not in
  the code yet) when nothing named or strong matches (best < 7, and less than 1
  ahead of the third). Both `take-every` tasks now get no context; on the 275
  git-history questions nothing changes (a looser version emptied 177 packs and
  dropped right-function-in-top-5 from 48% to 27%, so it was narrowed twice).
- **New embedding model, on by default.** Larger models compared on the 275
  questions (embedding alone / fused with myrn's text search, function in top 5,
  file in top 5):

  | Model (download) | Alone fn / file | Fused fn / file |
  |---|---|---|
  | potion-base-8M (30 MB, old) | 40% / 67% | 53% / 75% |
  | bge-small-en-v1.5 (130 MB) | 49% / 76% | 56% / 78% |
  | bge-base-en-v1.5 (440 MB) | 49% / 76% | 55% / 76% |
  | jina-embeddings-v2-base-code (640 MB) | 47% / 76% | 55% / 77% |
  | **snowflake-arctic-embed-m (440 MB)** | **53% / 80%** | **57% / 80%** |

  myrn now runs `Snowflake/snowflake-arctic-embed-m` (BERT, 110M parameters)
  on the CPU with candle, embeds each function with its Behaviour and Terms,
  prefixes queries as the model expects, and `myrn connect` turns it on
  (`--no-embeddings` skips the download). The old thresholds cut off nearly every
  match from the new model; new defaults: no floor, up to 10 matches.

  | myrn ranking, 275 questions | Function first | Function in top 5 | File in top 5 | MRR |
  |---|---|---|---|---|
  | Embeddings off (old default) | 32% | 48% | 60% | 0.378 |
  | potion-base-8M on | 35% | 51% | 67% | 0.409 |
  | **snowflake-arctic-embed-m on** | **38%** | **57%** | **76%** | **0.454** |

  Cost: indexing about 0.1 s per function on this machine's CPU (more-itertools
  894 vectors 92 s, remeda 1,689 in 4 min; once per repository, then only
  changed code), and about +130 ms per prompt (185 ms vs 58 ms per context).
  Fixed on the way: a slow model made every embedding request start another
  helper process (8 at once); bulk indexing now loads the model once in-process,
  a listening helper is never duplicated, and batches are sorted by length.

## Stage 8: full series, myrn as shipped (2026-11-24)

All 16 tasks × 3 runs, without myrn vs myrn with everything above: Arctic Embed
embeddings, the use-context hint (in the system prompt: the benchmark does not
load CLAUDE.md), MCP on, question code, edit ranges, the stub fix and the
new-function rule. Run: `bench/runs/cheap-s8`, 96 sessions, $0.58.

**Verdict: the best result so far, not yet conclusive overall.** Tokens −10%
(−18.1k ± 23.6k per pair), cheaper in 31 of 48 pairs. Every myrn session passed
(48/48; without myrn 47/48).

| 48 pairs | Stage 8 | Stage 6, same pairs | Stages 1–3, same pairs |
|---|---|---|---|
| **All** | **−10%** (31/48) | −1% (25/48) | −4% (25/48) |
| Questions | **−35%** (13/18), −30.2k ± 21.6k | −26% (10/18) | −41% (14/18) |
| Bug fixes | **−19%** (14/18), −39.4k ± 26.1k | +1% (9/18) | +1% (5/18) |
| Features | +10% (4/12) | +6% (6/12) | +7% (6/12) |
| more-itertools | +15% | +11% | +9% |
| remeda | **−15%** | +2% | −18% |
| ripgrep | **−16%** | −7% | −2% |

Reads and searches before the first correct edit 4.1 → 3.4; turns 8.4 → 7.7;
success 98% → 100%.

| Task | Stage 8 | Stage 6 | Stages 1–3 |
|---|---|---|---|
| `more-itertools/bugfix/collapse-levels` | +5% | +3% | +16% |
| `more-itertools/bugfix/unique-everseen-key` | −12% | +27% | +50% |
| `more-itertools/feature/take-every` | +46% | +20% | +18% |
| `more-itertools/question/unhashable-everseen` | −64% | −53% | −64% |
| `more-itertools/question/windowed-step` | −41% | +21% | −50% |
| `remeda/bugfix/chunk-drops-remainder` | +11% | −21% | −16% |
| `remeda/bugfix/clamp-zero-limit` | −26% | −30% | −21% |
| `remeda/feature/take-every` | +8% | +65% | +10% |
| `remeda/question/lazy-pipe` | −84% | −62% | −65% |
| `remeda/question/purry` | −31% | −30% | −46% |
| `ripgrep/bugfix/gitignore-negation-order` | −18% | +37% | +8% |
| `ripgrep/bugfix/glob-multi-dot-extension` | −38% | −23% | −6% |
| `ripgrep/feature/glob-is-literal` | −10% | −24% | +13% |
| `ripgrep/feature/globset-first-match` | −9% | −28% | −19% |
| `ripgrep/question/binary-detection` | +7% | +2% | −26% |
| `ripgrep/question/gitignore-matcher` | +11% | +10% | +15% |

Findings:
- **Bug fixes are a real saving for the first time** (−19%, interval below zero;
  every earlier version was about break-even). `gitignore-negation-order` went
  from +37% to −18%.
- **Questions** save about a third (interval below zero).
- **Features still lose** (+10%), almost all from `more-itertools/feature/take-every`
  (+46%): the embeddings lift its pointers above the new-function bar, and with
  the hint the agent does more checking (type checks, stub edits). In Stage 7 the
  hint also hurt `remeda/feature/take-every`. Candidates: hint wording that
  applies only to code shown in myrn's context, or no pointers for new functions
  whatever their score.
- `binary-detection` and `gitignore-matcher` (+7%, +11%) remain small losses:
  short questions where myrn's fixed overhead outweighs its help.

## Feature work: plan and free steps (2026-11-25)

Research (see the plan in the session): retrieving context selectively beats
always retrieving (Repoformer, CodeFilter); for new code, similar snippets help
only when a close analogue exists, while repository conventions matter
(AllianceCoder, RAMBO); feature work is the hardest category for agents
(FEA-Bench, FeatBench, NoCode-bench); token reduction can raise billed cost
through extra turns (arXiv 2607.12161).

**12 new feature tasks** (`bench/tasks/*/feature/gen-*.yaml`), built like
FEA-Bench: an existing function is removed (code, export, stub, docs entry and
its tests); the reference solution restores it; the hidden check is the
function's own tests; the prompt is its documentation. more-itertools:
`subfactorial`, `extract`, `outer_product`, `takewhile_inclusive`,
`multinomial`, `loops`. remeda: `splitAt`, `takeLast`, `sortedIndex`,
`uncapitalize`, `only`, `nthBy` (remeda's vitest tests run on Node through a
small stand-in). All 12 validate: the check fails after setup and passes with
the solution. Caveat: the removed code is still in git history, for both
conditions alike; myrn's checklist ignores commits that name the function.

**"How code is added here" checklist.** For a request to add code, myrn lists
the files that past commits adding code to the same place changed together
(new table `commit_adds`, filled when indexing history, with a one-time
backfill). Tested first as a prototype against the files each feature task's
reference solution touches: **90% precision, 88% recall over 14 tasks**
(diff-based detection; going by commit subjects found only 38%). Examples:

- more-itertools, a function in `more.py`: `more.py`, `more.pyi`,
  `tests/test_more.py`, `docs/api.rst`, `README.rst` (from 12 past additions).
- remeda, a new utility: `src/<name>.ts`, its `.test.ts` and `.test-d.ts`,
  `src/index.ts`.
- ripgrep `glob-is-literal`: `crates/globset/src/glob.rs`, `lib.rs`.

The target file is a path the task names, else the file of a name it uses (only
if that name picks out one file: `Glob` is also a ripgrep flag), else the file
past additions most often went to (never a stub). Bug-fix and question contexts
are unchanged (12/12 identical).

**Hint scoped to code myrn shows:** "use it instead of opening those files
again, and when you edit that code, read only its line range", plus a mention of
the checklist. The general "read only the line range you need" is gone. The
benchmark's `hint` condition now uses myrn's own text (`MYRN_BENCH_HINT`
overrides it).

## Stage 9: feature tasks, run 1 (2026-11-25)

16 feature tasks × 1 run: without myrn, Stage 8 myrn (binary from `8482b69`, its
first hint), and new myrn (checklist + scoped hint). Runs `cheap-s9-old`,
`cheap-s9-new`; 48 sessions, $0.53 including a broken first attempt (the bench
upgraded each session's database, which the older binary then refused to read;
fixed: the bench now reads session data read-only).

| 16 feature tasks | Tokens | Billed cost | Turns | Cheaper in | Passed |
|---|---|---|---|---|---|
| Without myrn | – | $0.154 | 12.2 | – | 14/16 |
| Stage 8 myrn | +10% | +9% | 13.8 | 8/16 | 14/16 |
| New myrn (checklist + scoped hint) | **+25%** | **+34%** | 14.7 | 4/16 | 14/16 |

All three failed the same two tasks (`gen-split-at`, `gen-take-last`): their
tests check fractional counts, a remeda convention the prompt does not mention.

**The checklist widens the job.** Files edited per session: without 2.1,
Stage 8 2.5, new 3.6. It reads as a to-do list: agents update the stub, docs,
README and extra tests the way the project's own commits do. More complete
work, but the hidden checks only test the function, so it shows as cost.
Not continued to runs 2–3.

## Stage 9b: selective injection, run 1 (2026-11-25)

New-function requests got no context at all (`cheap-s9-sel`, 16 sessions,
$0.23): +58% tokens against run 1's without-myrn sessions. Not trusted: those
sessions ran hours after the without-myrn ones (not interleaved), the hint
wording differed, and on ripgrep, whose packs were unchanged, it was +50% where
Stage 8 had +0%. Single feature sessions also swing 2-3× by chance (one agent
stumbled on remeda's own agent guide in `.agents/skills`, which prescribes extra
tests, and doubled its session). Selective injection was dropped; myrn's
injection was restored exactly to Stage 8 (16/16 identical packs).

## Stage 10: hint wording on feature tasks, interleaved (2026-11-25)

All 16 feature tasks × 2 runs × 3 conditions run back to back per task: without
myrn, myrn with the Stage 8 hint (`hint-v1`), myrn with the narrower hint
(`hint`: "use the code Myrn shows; when you edit that code, read only its line
range"). Same binary, Stage 8 injection. Run `cheap-s10`, 96 sessions, $1.20.

| 32 pairs | Tokens | Billed cost | Turns | Files edited | Cheaper in | Passed |
|---|---|---|---|---|---|---|
| Without myrn | – | – | 15.0 | 3.2 | – | 27/32 |
| **myrn, Stage 8 hint** | **−17%** (−66k ± 50k) | **−13%** | 13.8 | 3.1 | **22/32** (p = 0.05) | **28/32** |
| myrn, narrower hint | +19% | +17% | 15.4 | 3.2 | 12/32 | 28/32 |
| narrower vs Stage 8 hint | +43% | +33% | | | 8/32 (p = 0.01) | |

Every condition fails the same two tasks some of the time (`gen-split-at`,
`gen-take-last`: their tests check fractional counts the prompt never mentions).

| Task | Stage 8 hint, run 1 / 2 | Narrower hint, run 1 / 2 |
|---|---|---|
| `ripgrep/feature/glob-is-literal` | −45% / −43% | −40% / −33% |
| `ripgrep/feature/globset-first-match` | −38% / −24% | −58% / −20% |
| `remeda/feature/gen-sorted-index` | −53% / −69% | −63% / −59% |
| `remeda/feature/gen-only` | −44% / −47% | −66% / +40% |
| `remeda/feature/take-every` | −26% / −26% | +32% / +321% |
| `more-itertools/feature/gen-takewhile-inclusive` | +100% / +42% | +58% / +3% |
| `more-itertools/feature/gen-outer-product` | +25% / +98% | −31% / +7% |

Findings:
- **With conditions interleaved, myrn saves on feature work too**: −17% tokens,
  −13% cost, the interval below zero, one more task passed, the same number of
  files edited. The earlier "+10% on features" (Stages 1-8, 4 hand-written
  feature tasks) was largely noise; Stage 9 compared sessions run hours apart.
- **The Stage 8 hint stays.** The narrower wording costs a third more
  (p = 0.01); reverted. remeda and ripgrep features gain most; more-itertools
  features still often lose.
- Lesson: compare only interleaved conditions, and expect single feature
  sessions to vary 2-3×.

## Next

- Optional: a full 16-task series with the Stage 10 setup on all task kinds
  (questions, bug fixes and features together, interleaved) for one headline
  number.
- Two generated tasks (`gen-split-at`, `gen-take-last`) test fractional counts
  the prompt never mentions; fix their prompts or checks before the next run.
- Short questions (`gitignore-matcher`, `binary-detection`) still lose a little
  to myrn's fixed overhead (session map + hook wrapper).

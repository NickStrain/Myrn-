# Benchmark results

Per-session results of every benchmark run, copied from `bench/runs/<dir>/`
(which holds the full transcripts and is not committed). One JSON object per
session: task, condition, run, model, success, tokens, billed cost, turns,
injected items and the hidden check's output. `<dir>.report.md` is the
generated report where one exists.

The write-up for the Haiku series, stage by stage, is
[docs/bench-haiku.md](../../docs/bench-haiku.md); the Opus Gate 1 run is
[docs/gate1.md](../../docs/gate1.md).

| File | Stage |
|---|---|
| `cheap-smoke` | 1: harness check |
| `cheap-pilot`, `cheap-r23`, `cheap-all` | 2–3: original myrn (`cheap-all` = both, runs renumbered 1–3) |
| `cheap-v2` | 4: Behaviour/Terms map, targeted bug context |
| `cheap-v3` | 5: no test pointers |
| `cheap-s6` | 6: full re-run |
| `cheap-s7` | 7: hard tasks, with and without the hint |
| `cheap-s8` | 8: myrn as shipped (condition `hint`) |
| `cheap-s9-old`, `cheap-s9-new`, `cheap-s9-old-broken` | 9: file checklist (the broken set failed for a bench bug) |
| `cheap-s9-sel` | 9b: no context for new functions |
| `cheap-s10` | 10: hint wording on features (`hint_v1` = shipped wording) |
| `gate1` | Opus Gate 1 |

Note: in `cheap-s10`, the `hint` condition used the narrow wording that was
then dropped; myrn now ships the `hint_v1` wording.

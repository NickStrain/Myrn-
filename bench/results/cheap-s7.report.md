# Gate 1: paired benchmark

8 tasks, up to 3 runs per condition, conditions: Without myrn / myrn, inject on / myrn, inject on, + use-context hint, model: haiku. 72 runs total, 0 excluded because the run itself failed.

## Gate 1 (inject on vs without myrn): FAIL

task tokens did not fall.

## Paired comparisons

| Comparison | Tokens | Pairs | Change per task | Cheaper in | Sign test p | Success | Verdict |
|---|---|---|---|---|---|---|---|
| Without myrn → myrn, inject on | task | 24 | +5.9k ± 36.2k (+3%) | 12 of 24 | 1.000 | 96% → 96% | FAIL |
| Without myrn → myrn, inject on | total, recording included | 24 | +5.9k ± 36.2k (+3%) | 12 of 24 | 1.000 | 96% → 96% | FAIL |
| Without myrn → myrn, inject on, + use-context hint | task | 24 | -16.5k ± 39.9k (-8%) | 17 of 24 | 0.064 | 96% → 100% | INCONCLUSIVE |
| Without myrn → myrn, inject on, + use-context hint | total, recording included | 24 | -16.5k ± 39.9k (-8%) | 17 of 24 | 0.064 | 96% → 100% | INCONCLUSIVE |
| myrn, inject on → myrn, inject on, + use-context hint | task | 24 | -22.4k ± 38.6k (-11%) | 13 of 24 | 0.839 | 96% → 100% | INCONCLUSIVE |
| myrn, inject on → myrn, inject on, + use-context hint | total, recording included | 24 | -22.4k ± 38.6k (-11%) | 13 of 24 | 0.839 | 96% → 100% | INCONCLUSIVE |

Task tokens leave out the recording phase at the end of myrn sessions (see Recording). Values are means ± 95% confidence intervals. Automatic injection becomes the default only on PASS.

## All tasks

| Metric | Without myrn | myrn, inject on | myrn, inject on, + use-context hint |
|---|---|---|---|
| Task tokens | 201.8k ± 65.0k | 207.6k ± 71.9k | 185.3k ± 63.1k |
| Total tokens, recording included | 201.8k ± 65.0k | 207.6k ± 71.9k | 185.3k ± 63.1k |
| Output tokens | 4.3k ± 1.0k | 4.5k ± 1.2k | 4.6k ± 1.3k |
| Cost (USD), recording included | 0.0066 ± 0.0017 | 0.0066 ± 0.0019 | 0.0064 ± 0.0020 |
| Reads and searches before first correct edit | 4.7 ± 1.4 | 4.4 ± 1.5 | 2.9 ± 1.2 |
| Reads and searches, whole task | 6.2 ± 1.9 | 5.8 ± 1.9 | 5.2 ± 1.6 |
| Time to passing tests (s) | 32.9 ± 6.9 | 34.5 ± 7.3 | 36.0 ± 8.6 |
| Turns | 8.8 ± 2.4 | 8.4 ± 2.5 | 7.8 ± 2.2 |
| Task success | 96% | 96% | 100% |
| Timed out | 0 | 0 | 0 |

## myrn sessions

| | myrn, inject on | myrn, inject on, + use-context hint |
|---|---|---|
| Items injected per run | 4.4 | 4.4 |
| Injection use (items whose file the agent opened) | 30% | 30% |
| Learned records injected per run | 0.0 | 0.0 |
| Stale injections (records shown for code that changed since) | – (no records injected) | – (no records injected) |
| myrn tool calls during the task | 0.0 | 0.0 |
| Sessions asked to record | 0 of 24 | 0 of 24 |
| Recording tokens per asked session | – | – |
| Recording share of the session's tokens | 0% | 0% |
| record calls per asked session (rejected) | – | – |
| Records stored per run | 0.0 | 0.0 |
| Prompt hook latency (all myrn runs) | median 50 ms, max 83 ms | |

## Per task

| Task | Kind | Success without / with / hint | Task tokens without | Task tokens with | Task tokens hint | Change (inject on vs without) |
|---|---|---|---|---|---|---|
| `more-itertools/bugfix/collapse-levels` | bugfix | 67% / 67% / 100% | 148.1k ± 79.1k | 139.0k ± 144.2k | 157.6k ± 68.3k | -6% |
| `more-itertools/bugfix/unique-everseen-key` | bugfix | 100% / 100% / 100% | 98.6k ± 12.4k | 65.2k ± 25.6k | 71.7k ± 26.5k | -34% |
| `more-itertools/question/windowed-step` | question | 100% / 100% / 100% | 39.4k ± 0.8k | 29.3k ± 30.2k | 23.8k ± 4.0k | -26% |
| `remeda/feature/take-every` | feature | 100% / 100% / 100% | 298.3k ± 297.1k | 388.8k ± 207.0k | 454.3k ± 83.8k | +30% |
| `ripgrep/bugfix/gitignore-negation-order` | bugfix | 100% / 100% / 100% | 486.3k ± 44.0k | 470.0k ± 236.8k | 356.7k ± 224.9k | -3% |
| `ripgrep/bugfix/glob-multi-dot-extension` | bugfix | 100% / 100% / 100% | 323.6k ± 180.4k | 363.9k ± 79.4k | 232.0k ± 105.3k | +12% |
| `ripgrep/question/binary-detection` | question | 100% / 100% / 100% | 157.6k ± 90.0k | 143.2k ± 48.6k | 124.6k ± 68.0k | -9% |
| `ripgrep/question/gitignore-matcher` | question | 100% / 100% / 100% | 62.3k ± 28.7k | 61.6k ± 1.4k | 61.4k ± 1.1k | -1% |

## Method

Each run starts from a fresh clone of the pinned commit with the task's setup folded into HEAD (no remote, reflog or other branches), a fresh myrn data directory, and `claude -p` with user and project settings and MCP servers off (`--setting-sources local --strict-mcp-config`; myrn's hooks live in `.claude/settings.local.json`). *myrn* conditions: `myrn init`, `myrn connect` hooks (recording with consent; at the end of a turn that edited code the agent is asked to `record` what it learned), and myrn's MCP server; *inject on* also sets `[context] inject = true`. *Without*: none of it. Task tokens are input + cache creation + cache reads + output from the session result, minus the API calls after myrn's record request; injected context is counted. Success is the task's hidden check (bug fixes and features) or required terms in the answer given before any record request (questions). Exploration counts Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file the task expects to change.

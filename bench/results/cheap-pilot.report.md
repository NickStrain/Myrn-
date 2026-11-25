# Gate 1: paired benchmark

16 tasks, up to 1 runs per condition, conditions: Without myrn / myrn, inject on, model: haiku. 32 runs total, 0 excluded because the run itself failed.

## Gate 1 (inject on vs without myrn): INCONCLUSIVE

tokens fell on average, but the 95% interval includes no change; run more pairs.

## Paired comparisons

| Comparison | Tokens | Pairs | Change per task | Cheaper in | Sign test p | Success | Verdict |
|---|---|---|---|---|---|---|---|
| Without myrn → myrn, inject on | task | 16 | -4.7k ± 48.3k (-3%) | 8 of 16 | 1.000 | 100% → 100% | INCONCLUSIVE |
| Without myrn → myrn, inject on | total, recording included | 16 | -4.7k ± 48.3k (-3%) | 8 of 16 | 1.000 | 100% → 100% | INCONCLUSIVE |

Task tokens leave out the recording phase at the end of myrn sessions (see Recording). Values are means ± 95% confidence intervals. Automatic injection becomes the default only on PASS.

## All tasks

| Metric | Without myrn | myrn, inject on |
|---|---|---|
| Task tokens | 170.2k ± 56.2k | 165.4k ± 69.8k |
| Total tokens, recording included | 170.2k ± 56.2k | 165.4k ± 69.8k |
| Output tokens | 3.4k ± 1.0k | 3.7k ± 1.2k |
| Cost (USD), recording included | 0.0064 ± 0.0016 | 0.0063 ± 0.0019 |
| Reads and searches before first correct edit | 4.4 ± 1.2 | 3.3 ± 1.5 |
| Reads and searches, whole task | 5.2 ± 1.5 | 4.6 ± 1.9 |
| Time to passing tests (s) | 25.8 ± 7.3 | 29.0 ± 7.7 |
| Turns | 7.6 ± 2.0 | 7.1 ± 2.5 |
| Task success | 100% | 100% |
| Timed out | 0 | 0 |

## myrn sessions

| | myrn, inject on |
|---|---|
| Items injected per run | 4.8 |
| Injection use (items whose file the agent opened) | 42% |
| Learned records injected per run | 0.0 |
| Stale injections (records shown for code that changed since) | – (no records injected) |
| myrn tool calls during the task | 0.0 |
| Sessions asked to record | 0 of 16 |
| Recording tokens per asked session | – |
| Recording share of the session's tokens | 0% |
| record calls per asked session (rejected) | – |
| Records stored per run | 0.0 |
| Prompt hook latency (all myrn runs) | median 38 ms, max 58 ms |

## Per task

| Task | Kind | Success without / with | Task tokens without | Task tokens with | Change (inject on vs without) |
|---|---|---|---|---|---|
| `more-itertools/bugfix/collapse-levels` | bugfix | 100% / 100% | 102.7k | 133.0k | +29% |
| `more-itertools/bugfix/unique-everseen-key` | bugfix | 100% / 100% | 57.3k | 194.0k | +239% |
| `more-itertools/feature/take-every` | feature | 100% / 100% | 266.5k | 162.2k | -39% |
| `more-itertools/question/unhashable-everseen` | question | 100% / 100% | 54.3k | 19.5k | -64% |
| `more-itertools/question/windowed-step` | question | 100% / 100% | 61.5k | 22.7k | -63% |
| `remeda/bugfix/chunk-drops-remainder` | bugfix | 100% / 100% | 101.1k | 147.0k | +45% |
| `remeda/bugfix/clamp-zero-limit` | bugfix | 100% / 100% | 186.1k | 85.3k | -54% |
| `remeda/feature/take-every` | feature | 100% / 100% | 372.9k | 203.1k | -46% |
| `remeda/question/lazy-pipe` | question | 100% / 100% | 109.2k | 45.2k | -59% |
| `remeda/question/purry` | question | 100% / 100% | 109.3k | 65.9k | -40% |
| `ripgrep/bugfix/gitignore-negation-order` | bugfix | 100% / 100% | 280.0k | 315.2k | +13% |
| `ripgrep/bugfix/glob-multi-dot-extension` | bugfix | 100% / 100% | 275.2k | 342.9k | +25% |
| `ripgrep/feature/glob-is-literal` | feature | 100% / 100% | 319.6k | 507.5k | +59% |
| `ripgrep/feature/globset-first-match` | feature | 100% / 100% | 220.9k | 177.4k | -20% |
| `ripgrep/question/binary-detection` | question | 100% / 100% | 151.1k | 162.8k | +8% |
| `ripgrep/question/gitignore-matcher` | question | 100% / 100% | 54.8k | 62.8k | +15% |

## Method

Each run starts from a fresh clone of the pinned commit with the task's setup folded into HEAD (no remote, reflog or other branches), a fresh myrn data directory, and `claude -p` with user and project settings and MCP servers off (`--setting-sources local --strict-mcp-config`; myrn's hooks live in `.claude/settings.local.json`). *myrn* conditions: `myrn init`, `myrn connect` hooks (recording with consent; at the end of a turn that edited code the agent is asked to `record` what it learned), and myrn's MCP server; *inject on* also sets `[context] inject = true`. *Without*: none of it. Task tokens are input + cache creation + cache reads + output from the session result, minus the API calls after myrn's record request; injected context is counted. Success is the task's hidden check (bug fixes and features) or required terms in the answer given before any record request (questions). Exploration counts Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file the task expects to change.

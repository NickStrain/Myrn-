# Gate 1: paired benchmark

16 tasks, up to 3 runs per condition, conditions: Without myrn / myrn, inject on, model: haiku. 96 runs total, 0 excluded because the run itself failed.

## Gate 1 (inject on vs without myrn): INCONCLUSIVE

tokens fell on average, but the 95% interval includes no change; run more pairs.

## Paired comparisons

| Comparison | Tokens | Pairs | Change per task | Cheaper in | Sign test p | Success | Verdict |
|---|---|---|---|---|---|---|---|
| Without myrn → myrn, inject on | task | 48 | -8.7k ± 29.0k (-4%) | 25 of 48 | 0.885 | 100% → 100% | INCONCLUSIVE |
| Without myrn → myrn, inject on | total, recording included | 48 | -8.7k ± 29.0k (-4%) | 25 of 48 | 0.885 | 100% → 100% | INCONCLUSIVE |

Task tokens leave out the recording phase at the end of myrn sessions (see Recording). Values are means ± 95% confidence intervals. Automatic injection becomes the default only on PASS.

## All tasks

| Metric | Without myrn | myrn, inject on |
|---|---|---|
| Task tokens | 199.5k ± 39.5k | 190.8k ± 46.4k |
| Total tokens, recording included | 199.5k ± 39.5k | 190.8k ± 46.4k |
| Output tokens | 3.8k ± 0.6k | 4.1k ± 0.8k |
| Cost (USD), recording included | 0.0066 ± 0.0010 | 0.0065 ± 0.0012 |
| Reads and searches before first correct edit | 4.9 ± 0.7 | 3.8 ± 0.9 |
| Reads and searches, whole task | 6.1 ± 1.0 | 5.2 ± 1.3 |
| Time to passing tests (s) | 29.3 ± 4.5 | 33.7 ± 6.2 |
| Turns | 8.5 ± 1.3 | 8.1 ± 1.7 |
| Task success | 100% | 100% |
| Timed out | 0 | 0 |

## myrn sessions

| | myrn, inject on |
|---|---|
| Items injected per run | 4.8 |
| Injection use (items whose file the agent opened) | 46% |
| Learned records injected per run | 0.0 |
| Stale injections (records shown for code that changed since) | – (no records injected) |
| myrn tool calls during the task | 0.0 |
| Sessions asked to record | 0 of 48 |
| Recording tokens per asked session | – |
| Recording share of the session's tokens | 0% |
| record calls per asked session (rejected) | – |
| Records stored per run | 0.0 |
| Prompt hook latency (all myrn runs) | median 38 ms, max 65 ms |

## Per task

| Task | Kind | Success without / with | Task tokens without | Task tokens with | Change (inject on vs without) |
|---|---|---|---|---|---|
| `more-itertools/bugfix/collapse-levels` | bugfix | 100% / 100% | 146.5k ± 99.6k | 170.3k ± 92.9k | +16% |
| `more-itertools/bugfix/unique-everseen-key` | bugfix | 100% / 100% | 84.4k ± 59.7k | 126.9k ± 164.7k | +50% |
| `more-itertools/feature/take-every` | feature | 100% / 100% | 271.4k ± 66.6k | 319.1k ± 338.2k | +18% |
| `more-itertools/question/unhashable-everseen` | question | 100% / 100% | 54.6k ± 1.7k | 19.6k ± 0.1k | -64% |
| `more-itertools/question/windowed-step` | question | 100% / 100% | 46.7k ± 31.8k | 23.2k ± 2.9k | -50% |
| `remeda/bugfix/chunk-drops-remainder` | bugfix | 100% / 100% | 134.5k ± 144.0k | 113.5k ± 100.8k | -16% |
| `remeda/bugfix/clamp-zero-limit` | bugfix | 100% / 100% | 167.4k ± 71.9k | 131.5k ± 105.4k | -21% |
| `remeda/feature/take-every` | feature | 100% / 100% | 344.3k ± 262.5k | 378.4k ± 520.7k | +10% |
| `remeda/question/lazy-pipe` | question | 100% / 100% | 132.2k ± 58.7k | 45.7k ± 2.0k | -65% |
| `remeda/question/purry` | question | 100% / 100% | 122.2k ± 40.6k | 66.1k ± 1.2k | -46% |
| `ripgrep/bugfix/gitignore-negation-order` | bugfix | 100% / 100% | 364.9k ± 182.7k | 394.5k ± 171.1k | +8% |
| `ripgrep/bugfix/glob-multi-dot-extension` | bugfix | 100% / 100% | 444.6k ± 378.4k | 418.5k ± 306.7k | -6% |
| `ripgrep/feature/glob-is-literal` | feature | 100% / 100% | 396.2k ± 245.1k | 447.4k ± 157.8k | +13% |
| `ripgrep/feature/globset-first-match` | feature | 100% / 100% | 249.9k ± 130.4k | 203.6k ± 104.8k | -19% |
| `ripgrep/question/binary-detection` | question | 100% / 100% | 177.5k ± 58.2k | 132.1k ± 78.1k | -26% |
| `ripgrep/question/gitignore-matcher` | question | 100% / 100% | 54.7k ± 0.8k | 62.7k ± 0.2k | +15% |

## Method

Each run starts from a fresh clone of the pinned commit with the task's setup folded into HEAD (no remote, reflog or other branches), a fresh myrn data directory, and `claude -p` with user and project settings and MCP servers off (`--setting-sources local --strict-mcp-config`; myrn's hooks live in `.claude/settings.local.json`). *myrn* conditions: `myrn init`, `myrn connect` hooks (recording with consent; at the end of a turn that edited code the agent is asked to `record` what it learned), and myrn's MCP server; *inject on* also sets `[context] inject = true`. *Without*: none of it. Task tokens are input + cache creation + cache reads + output from the session result, minus the API calls after myrn's record request; injected context is counted. Success is the task's hidden check (bug fixes and features) or required terms in the answer given before any record request (questions). Exploration counts Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file the task expects to change.

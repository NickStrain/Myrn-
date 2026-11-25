# Gate 1: paired benchmark

16 tasks, up to 3 runs per condition, conditions: Without myrn / myrn, inject on, + use-context hint, model: haiku. 96 runs total, 0 excluded because the run itself failed.

## Gate 1 (inject on vs without myrn): NO DATA

no complete without/with pairs.

## Paired comparisons

| Comparison | Tokens | Pairs | Change per task | Cheaper in | Sign test p | Success | Verdict |
|---|---|---|---|---|---|---|---|
| Without myrn → myrn, inject on, + use-context hint | task | 48 | -18.1k ± 24.1k (-10%) | 31 of 48 | 0.059 | 98% → 100% | INCONCLUSIVE |
| Without myrn → myrn, inject on, + use-context hint | total, recording included | 48 | -18.1k ± 24.1k (-10%) | 31 of 48 | 0.059 | 98% → 100% | INCONCLUSIVE |

Task tokens leave out the recording phase at the end of myrn sessions (see Recording). Values are means ± 95% confidence intervals. Automatic injection becomes the default only on PASS.

## All tasks

| Metric | Without myrn | myrn, inject on, + use-context hint |
|---|---|---|
| Task tokens | 190.2k ± 37.4k | 172.1k ± 45.1k |
| Total tokens, recording included | 190.2k ± 37.4k | 172.1k ± 45.1k |
| Output tokens | 4.1k ± 0.7k | 4.1k ± 0.9k |
| Cost (USD), recording included | 0.0064 ± 0.0011 | 0.0057 ± 0.0013 |
| Reads and searches before first correct edit | 4.1 ± 0.6 | 3.4 ± 0.8 |
| Reads and searches, whole task | 5.8 ± 1.0 | 4.9 ± 1.1 |
| Time to passing tests (s) | 32.3 ± 6.6 | 34.0 ± 7.7 |
| Turns | 8.4 ± 1.4 | 7.7 ± 1.6 |
| Task success | 98% | 100% |
| Timed out | 0 | 0 |

## myrn sessions

| | myrn, inject on, + use-context hint |
|---|---|
| Items injected per run | 4.1 |
| Injection use (items whose file the agent opened) | 57% |
| Learned records injected per run | 0.0 |
| Stale injections (records shown for code that changed since) | – (no records injected) |
| myrn tool calls during the task | 0.0 |
| Sessions asked to record | 0 of 48 |
| Recording tokens per asked session | – |
| Recording share of the session's tokens | 0% |
| record calls per asked session (rejected) | – |
| Records stored per run | 0.0 |
| Prompt hook latency (all myrn runs) | median 533 ms, max 667 ms |

## Per task

| Task | Kind | Success without / hint | Task tokens without | Task tokens hint | Change (inject on vs without) |
|---|---|---|---|---|---|
| `more-itertools/bugfix/collapse-levels` | bugfix | 67% / 100% | 126.0k ± 32.9k | 132.0k ± 73.7k | – |
| `more-itertools/bugfix/unique-everseen-key` | bugfix | 100% / 100% | 91.6k ± 49.9k | 80.2k ± 51.1k | – |
| `more-itertools/feature/take-every` | feature | 100% / 100% | 329.7k ± 138.5k | 480.2k ± 308.2k | – |
| `more-itertools/question/unhashable-everseen` | question | 100% / 100% | 54.8k ± 0.2k | 19.8k ± 0.1k | – |
| `more-itertools/question/windowed-step` | question | 100% / 100% | 40.8k ± 0.8k | 24.0k ± 3.3k | – |
| `remeda/bugfix/chunk-drops-remainder` | bugfix | 100% / 100% | 119.6k ± 37.0k | 132.3k ± 59.8k | – |
| `remeda/bugfix/clamp-zero-limit` | bugfix | 100% / 100% | 144.4k ± 52.8k | 106.5k ± 97.1k | – |
| `remeda/feature/take-every` | feature | 100% / 100% | 419.8k ± 92.6k | 452.2k ± 614.6k | – |
| `remeda/question/lazy-pipe` | question | 100% / 100% | 137.9k ± 94.9k | 22.0k ± 0.4k | – |
| `remeda/question/purry` | question | 100% / 100% | 94.8k ± 23.4k | 65.4k ± 4.1k | – |
| `ripgrep/bugfix/gitignore-negation-order` | bugfix | 100% / 100% | 384.1k ± 52.7k | 313.2k ± 88.1k | – |
| `ripgrep/bugfix/glob-multi-dot-extension` | bugfix | 100% / 100% | 357.1k ± 101.2k | 221.9k ± 181.4k | – |
| `ripgrep/feature/glob-is-literal` | feature | 100% / 100% | 315.7k ± 191.7k | 283.5k ± 139.1k | – |
| `ripgrep/feature/globset-first-match` | feature | 100% / 100% | 243.1k ± 90.0k | 221.1k ± 193.5k | – |
| `ripgrep/question/binary-detection` | question | 100% / 100% | 127.8k ± 55.5k | 137.0k ± 52.8k | – |
| `ripgrep/question/gitignore-matcher` | question | 100% / 100% | 55.7k ± 0.6k | 62.0k ± 0.1k | – |

## Method

Each run starts from a fresh clone of the pinned commit with the task's setup folded into HEAD (no remote, reflog or other branches), a fresh myrn data directory, and `claude -p` with user and project settings and MCP servers off (`--setting-sources local --strict-mcp-config`; myrn's hooks live in `.claude/settings.local.json`). *myrn* conditions: `myrn init`, `myrn connect` hooks (recording with consent; at the end of a turn that edited code the agent is asked to `record` what it learned), and myrn's MCP server; *inject on* also sets `[context] inject = true`. *Without*: none of it. Task tokens are input + cache creation + cache reads + output from the session result, minus the API calls after myrn's record request; injected context is counted. Success is the task's hidden check (bug fixes and features) or required terms in the answer given before any record request (questions). Exploration counts Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file the task expects to change.

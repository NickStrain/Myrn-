# Gate 1: paired benchmark

16 tasks, up to 1 runs per condition, conditions: Without myrn / myrn, inject on, + use-context hint, model: haiku. 32 runs total, 0 excluded because the run itself failed.

## Gate 1 (inject on vs without myrn): NO DATA

no complete without/with pairs.

## Paired comparisons

| Comparison | Tokens | Pairs | Change per task | Cheaper in | Sign test p | Success | Verdict |
|---|---|---|---|---|---|---|---|
| Without myrn → myrn, inject on, + use-context hint | task | 16 | +26.7k ± 63.6k (+10%) | 8 of 16 | 1.000 | 88% → 88% | FAIL |
| Without myrn → myrn, inject on, + use-context hint | total, recording included | 16 | +26.7k ± 63.6k (+10%) | 8 of 16 | 1.000 | 88% → 88% | FAIL |

Task tokens leave out the recording phase at the end of myrn sessions (see Recording). Values are means ± 95% confidence intervals. Automatic injection becomes the default only on PASS.

## All tasks

| Metric | Without myrn | myrn, inject on, + use-context hint |
|---|---|---|
| Task tokens | 276.3k ± 42.6k | 303.0k ± 63.0k |
| Total tokens, recording included | 276.3k ± 42.6k | 303.0k ± 63.0k |
| Output tokens | 6.5k ± 1.1k | 7.8k ± 1.5k |
| Cost (USD), recording included | 0.0096 ± 0.0013 | 0.0105 ± 0.0017 |
| Reads and searches before first correct edit | 4.8 ± 0.8 | 5.1 ± 1.7 |
| Reads and searches, whole task | 7.9 ± 1.1 | 8.9 ± 2.2 |
| Time to passing tests (s) | 40.8 ± 9.7 | 50.1 ± 14.2 |
| Turns | 12.2 ± 1.3 | 13.8 ± 2.4 |
| Task success | 88% | 88% |
| Timed out | 0 | 0 |

## myrn sessions

| | myrn, inject on, + use-context hint |
|---|---|
| Items injected per run | 4.2 |
| Injection use (items whose file the agent opened) | 59% |
| Learned records injected per run | 0.0 |
| Stale injections (records shown for code that changed since) | – (no records injected) |
| myrn tool calls during the task | 0.0 |
| Sessions asked to record | 0 of 16 |
| Recording tokens per asked session | – |
| Recording share of the session's tokens | 0% |
| record calls per asked session (rejected) | – |
| Records stored per run | 0.0 |
| Prompt hook latency (all myrn runs) | median 778 ms, max 947 ms |

## Per task

| Task | Kind | Success without / hint | Task tokens without | Task tokens hint | Change (inject on vs without) |
|---|---|---|---|---|---|
| `more-itertools/feature/gen-extract` | feature | 100% / 100% | 294.8k | 621.3k | – |
| `more-itertools/feature/gen-loops` | feature | 100% / 100% | 254.7k | 207.3k | – |
| `more-itertools/feature/gen-multinomial` | feature | 100% / 100% | 256.7k | 410.2k | – |
| `more-itertools/feature/gen-outer-product` | feature | 100% / 100% | 249.0k | 227.5k | – |
| `more-itertools/feature/gen-subfactorial` | feature | 100% / 100% | 407.1k | 353.4k | – |
| `more-itertools/feature/gen-takewhile-inclusive` | feature | 100% / 100% | 297.6k | 343.2k | – |
| `more-itertools/feature/take-every` | feature | 100% / 100% | 194.3k | 271.5k | – |
| `remeda/feature/gen-nth-by` | feature | 100% / 100% | 251.8k | 386.9k | – |
| `remeda/feature/gen-only` | feature | 100% / 100% | 465.1k | 354.1k | – |
| `remeda/feature/gen-sorted-index` | feature | 100% / 100% | 305.0k | 124.0k | – |
| `remeda/feature/gen-split-at` | feature | 0% / 0% | 235.9k | 273.5k | – |
| `remeda/feature/gen-take-last` | feature | 0% / 0% | 235.2k | 195.0k | – |
| `remeda/feature/gen-uncapitalize` | feature | 100% / 100% | 187.3k | 173.6k | – |
| `remeda/feature/take-every` | feature | 100% / 100% | 179.7k | 297.0k | – |
| `ripgrep/feature/glob-is-literal` | feature | 100% / 100% | 376.3k | 358.6k | – |
| `ripgrep/feature/globset-first-match` | feature | 100% / 100% | 230.7k | 251.4k | – |

## Method

Each run starts from a fresh clone of the pinned commit with the task's setup folded into HEAD (no remote, reflog or other branches), a fresh myrn data directory, and `claude -p` with user and project settings and MCP servers off (`--setting-sources local --strict-mcp-config`; myrn's hooks live in `.claude/settings.local.json`). *myrn* conditions: `myrn init`, `myrn connect` hooks (recording with consent; at the end of a turn that edited code the agent is asked to `record` what it learned), and myrn's MCP server; *inject on* also sets `[context] inject = true`. *Without*: none of it. Task tokens are input + cache creation + cache reads + output from the session result, minus the API calls after myrn's record request; injected context is counted. Success is the task's hidden check (bug fixes and features) or required terms in the answer given before any record request (questions). Exploration counts Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file the task expects to change.

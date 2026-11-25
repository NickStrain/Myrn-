# Gate 1: paired benchmark

16 tasks, up to 2 runs per condition, conditions: Without myrn / myrn, inject on, + first use-context hint / myrn, inject on, + use-context hint, model: haiku. 96 runs total, 0 excluded because the run itself failed.

## Gate 1 (inject on vs without myrn): NO DATA

no complete without/with pairs.

## Paired comparisons

| Comparison | Tokens | Pairs | Change per task | Cheaper in | Sign test p | Success | Verdict |
|---|---|---|---|---|---|---|---|
| Without myrn → myrn, inject on, + use-context hint | task | 32 | +73.0k ± 112.3k (+19%) | 12 of 32 | 0.215 | 84% → 88% | FAIL |
| Without myrn → myrn, inject on, + use-context hint | total, recording included | 32 | +73.0k ± 112.3k (+19%) | 12 of 32 | 0.215 | 84% → 88% | FAIL |
| Without myrn → myrn, inject on, + first use-context hint | task | 32 | -65.6k ± 50.5k (-17%) | 22 of 32 | 0.050 | 84% → 88% | INCONCLUSIVE |
| Without myrn → myrn, inject on, + first use-context hint | total, recording included | 32 | -65.6k ± 50.5k (-17%) | 22 of 32 | 0.050 | 84% → 88% | INCONCLUSIVE |
| myrn, inject on, + first use-context hint → myrn, inject on, + use-context hint | task | 32 | +138.6k ± 107.7k (+43%) | 8 of 32 | 0.007 | 88% → 88% | FAIL |
| myrn, inject on, + first use-context hint → myrn, inject on, + use-context hint | total, recording included | 32 | +138.6k ± 107.7k (+43%) | 8 of 32 | 0.007 | 88% → 88% | FAIL |

Task tokens leave out the recording phase at the end of myrn sessions (see Recording). Values are means ± 95% confidence intervals. Automatic injection becomes the default only on PASS.

## All tasks

| Metric | Without myrn | myrn, inject on, + first use-context hint | myrn, inject on, + use-context hint |
|---|---|---|---|
| Task tokens | 389.0k ± 52.7k | 323.3k ± 58.8k | 462.0k ± 119.2k |
| Total tokens, recording included | 389.0k ± 52.7k | 323.3k ± 58.8k | 462.0k ± 119.2k |
| Output tokens | 8.1k ± 1.0k | 7.9k ± 1.3k | 10.1k ± 2.3k |
| Cost (USD), recording included | 0.0123 ± 0.0017 | 0.0108 ± 0.0017 | 0.0144 ± 0.0031 |
| Reads and searches before first correct edit | 5.9 ± 0.9 | 5.3 ± 1.5 | 5.1 ± 0.8 |
| Reads and searches, whole task | 9.3 ± 1.2 | 8.6 ± 1.8 | 9.5 ± 1.3 |
| Time to passing tests (s) | 52.5 ± 9.2 | 51.8 ± 9.8 | 68.4 ± 14.1 |
| Turns | 15.0 ± 1.4 | 13.8 ± 2.0 | 15.4 ± 1.9 |
| Task success | 84% | 88% | 88% |
| Timed out | 0 | 0 | 0 |

## myrn sessions

| | myrn, inject on, + first use-context hint | myrn, inject on, + use-context hint |
|---|---|---|
| Items injected per run | 4.2 | 4.2 |
| Injection use (items whose file the agent opened) | 59% | 57% |
| Learned records injected per run | 0.0 | 0.0 |
| Stale injections (records shown for code that changed since) | – (no records injected) | – (no records injected) |
| myrn tool calls during the task | 0.0 | 0.0 |
| Sessions asked to record | 0 of 32 | 0 of 32 |
| Recording tokens per asked session | – | – |
| Recording share of the session's tokens | 0% | 0% |
| record calls per asked session (rejected) | – | – |
| Records stored per run | 0.0 | 0.0 |
| Prompt hook latency (all myrn runs) | median 662 ms, max 918 ms | |

## Per task

| Task | Kind | Success without / hint-v1 / hint | Task tokens without | Task tokens hint-v1 | Task tokens hint | Change (inject on vs without) |
|---|---|---|---|---|---|---|
| `more-itertools/feature/gen-extract` | feature | 50% / 100% / 100% | 566.4k ± 3415.4k | 599.7k ± 2359.0k | 563.7k ± 581.5k | – |
| `more-itertools/feature/gen-loops` | feature | 100% / 100% / 100% | 328.3k ± 737.9k | 188.9k ± 138.3k | 348.2k ± 1100.2k | – |
| `more-itertools/feature/gen-multinomial` | feature | 100% / 100% / 100% | 343.2k ± 1343.0k | 313.5k ± 250.5k | 476.8k ± 953.0k | – |
| `more-itertools/feature/gen-outer-product` | feature | 100% / 100% / 100% | 320.1k ± 1471.2k | 473.6k ± 884.5k | 259.8k ± 529.8k | – |
| `more-itertools/feature/gen-subfactorial` | feature | 100% / 100% / 100% | 303.5k ± 238.5k | 332.9k ± 697.7k | 525.8k ± 1186.1k | – |
| `more-itertools/feature/gen-takewhile-inclusive` | feature | 100% / 100% / 100% | 324.3k ± 670.1k | 539.7k ± 41.1k | 409.4k ± 258.0k | – |
| `more-itertools/feature/take-every` | feature | 100% / 100% / 100% | 333.3k ± 85.3k | 282.3k ± 732.8k | 378.7k ± 566.6k | – |
| `remeda/feature/gen-nth-by` | feature | 100% / 100% / 100% | 637.8k ± 702.0k | 627.6k ± 295.4k | 1197.7k ± 4737.0k | – |
| `remeda/feature/gen-only` | feature | 100% / 100% / 100% | 536.5k ± 157.0k | 294.3k ± 182.8k | 460.3k ± 3476.5k | – |
| `remeda/feature/gen-sorted-index` | feature | 100% / 100% / 100% | 291.5k ± 335.9k | 111.5k ± 160.7k | 115.3k ± 207.8k | – |
| `remeda/feature/gen-split-at` | feature | 0% / 0% / 0% | 278.5k ± 379.4k | 172.5k ± 84.9k | 336.3k ± 164.8k | – |
| `remeda/feature/gen-take-last` | feature | 0% / 0% / 0% | 348.4k ± 622.8k | 198.1k ± 15.2k | 374.0k ± 1036.9k | – |
| `remeda/feature/gen-uncapitalize` | feature | 100% / 100% / 100% | 271.8k ± 1519.0k | 166.3k ± 409.3k | 267.9k ± 366.4k | – |
| `remeda/feature/take-every` | feature | 100% / 100% / 100% | 452.8k ± 1066.7k | 336.1k ± 783.2k | 1130.9k ± 5370.3k | – |
| `ripgrep/feature/glob-is-literal` | feature | 100% / 100% / 100% | 546.0k ± 306.5k | 304.4k ± 237.6k | 348.4k ± 425.2k | – |
| `ripgrep/feature/globset-first-match` | feature | 100% / 100% / 100% | 340.9k ± 685.4k | 232.0k ± 163.6k | 198.1k ± 395.8k | – |

## Method

Each run starts from a fresh clone of the pinned commit with the task's setup folded into HEAD (no remote, reflog or other branches), a fresh myrn data directory, and `claude -p` with user and project settings and MCP servers off (`--setting-sources local --strict-mcp-config`; myrn's hooks live in `.claude/settings.local.json`). *myrn* conditions: `myrn init`, `myrn connect` hooks (recording with consent; at the end of a turn that edited code the agent is asked to `record` what it learned), and myrn's MCP server; *inject on* also sets `[context] inject = true`. *Without*: none of it. Task tokens are input + cache creation + cache reads + output from the session result, minus the API calls after myrn's record request; injected context is counted. Success is the task's hidden check (bug fixes and features) or required terms in the answer given before any record request (questions). Exploration counts Read, Grep, Glob, read-only shell commands and myrn tool calls before the first edit to a file the task expects to change.

# Gate 2: record precision

The Phase 2 gate is "a hand-reviewed sample of records meets a precision target".

## Tools

- `myrn debug review --sample 30 [--seed N] [--out review.md]`: a checklist of learned records with their evidence, links and the code they point at (marked when the code changed since).
- `myrn debug review --score review.md`: precision from the marks (correct / vague / wrong), overall and per record type.
- Benchmark: each myrn session's records are exported to `<run>/records/`. `myrn-bench review <run> --sample 30` builds `<run>/review.md`; `myrn-bench review <run> --score` scores it.

## Partial review (2026-11-17)

The partial Gate 1 run produced **10 records**, all from `ripgrep/feature/globset-first-match`: 6 symbol summaries, 3 facts and 1 workflow. The checklist is `bench/runs/partial-gate/review.md` and is waiting for hand marks.

Claude's first read, which is **not** the hand review the gate requires: all 10 match the code they cite. The 3 facts (from 3 separate runs, so separate stores) independently record the same real gotcha: GlobSet checks match strategies by kind, not in insertion order, so "first match" must take the minimum over all strategies. The workflow ("use `cargo fmt`, not `rustfmt --edition 2021`, which reformats unrelated files") is specific and useful. One problem: `test_run` evidence stores the whole shell pipeline, including an absolute workspace path. It should be trimmed to the test command.

**Not enough for the gate:** 10 records from one task is not a representative sample. Records only get made when the session edited code through Edit/Write; shell edits need capturing first.

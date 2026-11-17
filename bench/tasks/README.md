# Benchmark tasks

One directory per repository, one YAML file per task (format in `../README.md`):

- `bugfix/`   bugs with known solutions: a setup patch breaks the code, a hidden test checks the fix
- `feature/`  small features, checked by hidden tests
- `question/` questions about the code, checked by required terms in the answer
- `replay/`   previously diagnosed bugs, reintroduced to test recall (Phase 2)

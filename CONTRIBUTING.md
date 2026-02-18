# Contributing

Thanks for contributing to ADL.

## Development Workflow

Use the default ADL workflow:
- `docs/default_workflow.md`
- `swarm/tools/README.md`

Typical local validation from `swarm/`:

```bash
cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Scope and Quality

- Keep changes issue-scoped and deterministic.
- Add or update tests for behavior changes.
- Prefer small PRs with clear acceptance criteria.

## Documentation

- Keep root `README.md` and milestone docs consistent with shipped behavior.
- For milestone work, update files under `docs/milestones/<version>/`.

## Security

See `SECURITY.md` for vulnerability disclosure guidance.

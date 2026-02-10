# ADL v0.2 Release Checklist (Final)

---

## 1) Pre-release sanity
- [ ] All v0.2 issues closed or explicitly deferred (milestone updated)
- [ ] `main` branch is green (CI passing)
- [ ] No untracked or local-only changes in working tree
- [ ] Examples run end-to-end on a clean checkout
- [ ] Working tree clean: no `out/`, `target/`, `issue-*.sh`, or other generated artifacts
- [ ] `.gitignore` covers local-only artifacts (e.g. `.adl/cards/`, `swarm/out/`, `swarm/target/`)

## 2) Code & schema validation (swarm)
- [ ] `cargo fmt` clean
- [ ] `cargo clippy --all-targets -- -D warnings` clean
- [ ] `cargo test` clean
- [ ] Schema validation tests pass (`tests/schema_tests.rs`)
- [ ] Example ADL files validate against schema

## 3) Demo verification (v0.2)
- [ ] Canonical demo command works as documented
- [ ] Demo produces HTML/game output
- [ ] Output is written to files (not dumped to terminal)
- [ ] Demo auto-opens browser (where supported) OR prints a single `file://...` path to open
- [ ] Demo content is popular and safe (game-based; avoid politics)
- [ ] Demo output directory is deterministic and documented (e.g. `./out/`)

Recommended demo run:
```
# If supported by current CLI:
cargo run -- examples/v0-2-coordinator-agents-sdk.adl.yaml
```

## 4) Documentation consistency
- [ ] v0.2 release notes written (#38)
- [ ] README updated if behavior changed
- [ ] Examples referenced correctly
- [ ] Terminology consistent (input/output cards; avoid deprecated terms)

## 5) GitHub hygiene
- [ ] All release PRs merged
- [ ] No stray draft PRs
- [ ] Branches pruned (optional)
- [ ] Milestone reflects shipped vs deferred items

## 6) Tagging & release
- [ ] Version number confirmed (v0.2.x)
- [ ] Git tag created

Example:
```
git tag -a v0.2.0 -m "ADL v0.2.0"
git push origin v0.2.0
```

- [ ] GitHub release created with notes
- [ ] Artifacts verified (if any)

---

# C-SDLC v2

`csdlc-shepherd --schema` and `--example <name>` are currently the complete
discovery path for a JSON-input owner binary. The same affordance is planned
for the remaining request-driven binaries as their CLI contracts are repaired;
until then, use their typed public schema bundle and request definitions.

This is the standalone clean-room C-SDLC v2 workspace. It does not depend on
ADL or Runtime crates and does not reuse their lifecycle implementation,
schemas, templates, tests, fixtures, or skills.

Gate 2 provides the typed lifecycle/card engine and whole-record transactions.
Gate 3 adds separate `csdlc-init` and `csdlc-bind` binaries for deterministic
construction and safe Git worktree/claim binding. Git uses typed argv arrays;
the control plane contains no shell or Python lifecycle logic. Later gates add
PVF, review truth, publication, and closeout without widening this core's
authority.

Gate 4 adds `csdlc-validate`, `csdlc-schedule`, and `csdlc-shepherd`. Validation
manifests contain executable-plus-argv commands, deterministic dependencies,
resource costs, network/credential posture, timeouts, and bounded evidence
policy. The scheduler and shepherd are pure read-only classifiers; only
`csdlc-validate` can execute a declared proof DAG.

Gate 5 adds `csdlc-review` for live-claim review assignment, exact-revision
review recording, finding/fix/route evidence, and a read-only publication
guard. Review has no GitHub or lifecycle publication authority.

## Focused validation

```text
cargo fmt --manifest-path csdlc-v2/Cargo.toml -- --check
cargo clippy --manifest-path csdlc-v2/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path csdlc-v2/Cargo.toml
```

## Contracts

`csdlc-edit schema` prints the versioned JSON Schema bundle. `csdlc-edit
bootstrap --request <json>` atomically creates an issue record and all six
cards from typed values. `csdlc-edit apply --request <json>` performs one
guarded semantic operation. `csdlc-doctor --repo <path> --issue <n>` emits
stable JSON and performs no network or mutation.

Bootstrap selects a typed planning profile and automatically writes explicit
SPP time/token estimates and VPP time/token budgets. There is no follow-up
manual budget-filling stage.

Gate 10A adds `csdlc-install` and a nine-skill operator manifest. It installs
provenance-recorded v2 binaries beside v1 and verifies a fail-closed
coexistence inventory. Gate 10A left v1 as the default; the tracked generation
selector remains the current authority, and later cutover never deletes or
disables any v1 surface.

Gate 10D1 adds `csdlc-eligibility`, a non-mutating decision and proposed-
manifest binary. It derives the exact Gate 1 inventory from its pinned Git
revision, binds operator approval to Phase B, Phase C, selector, manifest, and
code-revision digests, enforces the reviewed 90/80-percent thresholds and both
mandatory sunset windows, and always reports `deletion_executed: false` on
stdout. Its `schema` subcommand publishes the versioned JSON contracts. Actual
removal belongs to a separate approval-gated issue.

Markdown files are generated projections. The engine renders deterministic
Markdown from typed values, parses it with `markdown.rs`, validates semantic
anchors, and records values/rendered/AST digests. Direct Markdown edits fail
doctor as corruption.

For a closed-out record whose SRP routes residual work after terminal receipt
retention, use `csdlc-closeout reconcile-terminal` with the exact branch,
worktree, initialization digest, actor/reason, and `follow_ups` values. Each
follow-up must already be present in SRP residual risk; the typed operation
updates SOR and the retained receipt atomically and rejects arbitrary card
mutation.

Issue-local bootstrap is supported when all six cards and the approved design
already live in the target worktree: use a claim whose worktree is `.` and run
`csdlc-bind` from that worktree. The binder verifies the existing branch and
claim in place, performs no primary-checkout write, and still applies the
normal collision and protected-path checks.

# v0.91.5 CLI Command Inventory And Equivalence Table

Issue: #3594
Umbrella: #3592
Baseline dependency: #3593
Status: ready for implementation handoff

## Purpose

This artifact records the current `adl` command surface and the planned
ownership split for the v0.91.5 refactor mini-sprint. It is a migration
contract, not an implementation diff. Later issues should update behavior only
after proving the old and new command forms remain equivalent or fail closed.

## Source Evidence

Primary source surfaces reviewed:

- `adl/src/main.rs`
- `adl/src/cli/mod.rs`
- `adl/src/cli/usage.rs`
- `adl/src/cli/pr_cmd.rs`
- `adl/src/cli/pr_cmd_args.rs`
- `adl/src/cli/tooling_cmd.rs`
- `adl/src/cli/tooling_cmd/prompt_template.rs`
- `adl/src/cli/provider_cmd.rs`
- `adl/src/cli/demo_cmd.rs`
- `adl/src/cli/godel_cmd.rs`
- `adl/tools/pr.sh`
- `adl/tools/codex_pr.sh`
- `AGENTS.md`
- `docs/templates/PR_INIT_INVOCATION_TEMPLATE.md`
- `docs/milestones/v0.91.5/REFACTOR_SAFETY_BASELINE_3593.md`

## Owner Binaries

The first mini-sprint wave should prove three durable owner binaries before any
deeper split:

| Future owner | Owns | First issue |
| --- | --- | --- |
| `adl-csdlc` | C-SDLC workflow control plane, prompt-card tooling, issue/worktree/PR lifecycle, generated-card policy, and wrapper-compatible workflow commands. | #3596 and #3597 |
| `adl-runtime` | ADL workflow execution, resume, runtime-v2 proof/demo execution, provider runtime setup, agent runtime operations, instrumentation, signing/verification, learning export, and runtime-facing artifacts. | #3598 |
| `adl-review` | Review packet generation, review contract verification, review-card/runtime-surface checks, and future multi-agent review orchestration. | #3599 |

Everything outside those three owners remains in `adl` until a later reviewed
mini-sprint explicitly proves another boundary.

## Equivalence Table

| Current command family | Current source owner | Future owner | Compatibility status | Required characterization | Generated-card policy | Sunset status |
| --- | --- | --- | --- | --- | --- | --- |
| `adl <adl.yaml> ...` fallback workflow execution | `adl/src/cli/mod.rs`, `adl/src/cli/run.rs` | `adl-runtime run <adl.yaml> ...` | Keep shim in `adl`; old fallback must warn only after ambiguity policy is implemented. | Exit code, stdout plan/trace shape, run artifact paths, signed/unsigned policy errors. | New cards should use `adl-runtime run` once issue #3598 lands; before then keep `adl/tools/pr.sh run <issue>` for issue work and avoid runtime fallback in C-SDLC cards. | Sunset after `adl-runtime` is proven and generated-card migration is complete. |
| `adl resume <run_id> ...` | `adl/src/cli/run.rs` | `adl-runtime resume <run_id> ...` | Keep shim. | Resume error messages, steering patch behavior, run artifact lookup. | Runtime cards should move to `adl-runtime resume` after #3598. | Same as runtime fallback. |
| `adl pr create/init/run/doctor/ready/preflight/finish/closeout` issue mode | `adl/src/cli/pr_cmd.rs`, `adl/src/cli/pr_cmd_args.rs` | `adl-csdlc issue create/init/run/doctor/ready/preflight/finish/closeout` | Keep `adl/tools/pr.sh` canonical during migration; Rust alias should be compatibility only until wrapper contract is updated. | Exit code, branch/worktree paths, issue-card paths, PR URL output, closeout pruning, open-wave blocking. | New generated issue cards should continue to mention `adl/tools/pr.sh run <issue>` until #3597 changes wrapper truth. Deprecated direct `adl pr ...` strings should be warning/fail policy candidates in generated cards after #3597. | No sunset before wrapper migration and external adapter docs are updated. |
| `adl pr run <adl.yaml> ...` runtime YAML mode | `adl/src/cli/pr_cmd.rs` and runtime execution path | `adl-runtime run <adl.yaml> ...` | Must be split from issue-mode `run`; unknown or ambiguous inputs fail closed in #3595. | Numeric issue input, YAML path input, missing path, nonexistent path, and ambiguous filename cases. | Generated cards must not use `adl pr run <adl.yaml>` after #3595/#3598; use `adl-runtime run`. | Deprecate as soon as ambiguity tests land. |
| `adl/tools/pr.sh create/init/run/doctor/preflight/finish/closeout` | `adl/tools/pr.sh` plus Rust delegate | `adl-csdlc` wrapper spine remains shell-first during migration | Canonical repo workflow entrypoint today. Do not replace before #3597. | Wrapper output, shim warning text, Rust delegate fallback, dirty-main protection, worktree binding. | Generated C-SDLC cards must keep `adl/tools/pr.sh run <issue>` until wrapper migration explicitly changes AGENTS/templates. | Sunset not scheduled in this mini-sprint. |
| `adl tooling prompt-template ...` | `adl/src/cli/tooling_cmd/prompt_template.rs` | `adl-csdlc prompt-template ...` | Add alias after #3596; keep old form while docs/skills migrate. | Values validation, render, render-all, structure validation, schema parity, sample/schema writes. | New prompt-card generation should use the active template registry and may migrate to `adl-csdlc prompt-template` only after docs/skills update issue lands. | Old form becomes warning-only after generated-card validation rejects stale strings. |
| `adl tooling csdlc-prompt-editor` | `adl/src/cli/tooling_cmd.rs` | `adl-csdlc prompt-editor ...` | Keep shim. | Editor model export, sample rendering paths, advisory/editor authority boundaries. | Generated cards should not invoke browser/editor paths unless the issue specifically requires editor operation. | Later docs/skills rewrite issue. |
| `adl tooling generate-wp-issue-wave` | `adl/src/cli/tooling_cmd/wp_issue_wave.rs` | `adl-csdlc issue-wave generate ...` | Keep shim. | Output file shape, version handling, template source truth. | Milestone setup cards may move to `adl-csdlc` after #3597. | Later after milestone-creator skill update. |
| `adl tooling lint-prompt-spec`, `validate-structured-prompt`, `card-prompt` | `adl/src/cli/tooling_cmd/structured_prompt.rs`, `card_prompt.rs` | `adl-csdlc card ...` | Keep shim. | Prompt-spec parsing, legacy compatibility, structured prompt validators. | Generated cards should prefer renderer/schema tools for new cards; use validators for lifecycle truth. | Later after card docs migration. |
| `adl tooling code-review` | `adl/src/cli/tooling_cmd/code_review.rs` | `adl-review code-review ...` | Keep shim until #3599 proves review boundary. | Fixture vs Ollama backend, packet-only vs read-only-repo visibility, file-scope behavior, output packet paths. | Review cards should move to `adl-review` only after #3599 and skill docs update. | Warning after review skills are updated. |
| `adl tooling review-card-surface`, `review-runtime-surface`, `verify-review-output-provenance`, `verify-repo-review-contract` | `adl/src/cli/tooling_cmd/review_surface.rs`, `review_contract.rs` | `adl-review ...` | Keep shim. | Review contract exit codes, output paths, provenance failure messages. | Review/generated-card policy should require `adl-review` after #3599, independent of terminal shim warning mode. | Sunset after review docs/skills update. |
| `adl demo <name> ...` | `adl/src/cli/demo_cmd.rs` | `adl-runtime demo <name> ...` | Keep shim. | Demo default run behavior, trace output, artifact-open behavior, `--no-open` in CI. | Demo cards should migrate after runtime owner is proven; no change in #3594. | Later after demo mini-sprint alignment. |
| `adl runtime-v2 ...` | `adl/src/cli/runtime_v2_cmd.rs` | `adl-runtime runtime-v2 ...` or `adl-runtime proof ...` | Keep shim. | Proof artifact paths, demo outputs, feature-proof coverage output, slow-proof lane classification. | Runtime proof cards should use `adl-runtime` after #3598. | Later after runtime proof docs update. |
| `adl provider setup <family>` | `adl/src/cli/provider_cmd.rs` | `adl-runtime provider setup <family>` | Keep shim. | Supported families, output directory behavior, `--force`, generated env/readme/provider snippet paths. | Provider setup cards should use `adl-runtime provider setup` after #3598. | Later after provider/model matrix docs update. |
| `adl agent tick/run/status/inspect/stop` | `adl/src/cli/agent_cmd.rs` | `adl-runtime agent ...` | Keep shim. | Lease behavior, status JSON, stop reason, max-cycle/no-sleep behavior. | Long-lived runtime cards should use `adl-runtime agent` after #3598. | Later after runtime observability work. |
| `adl instrument ...` | `adl/src/cli/commands.rs` | `adl-runtime instrument ...` | Keep shim. | Graph JSON/DOT, replay, replay-bundle, diff-plan, diff-trace, schema generation, provider-substrate output. | Runtime/proof cards should use `adl-runtime instrument` after #3598. | Later after runtime docs update. |
| `adl learn export ...` | `adl/src/cli/commands.rs` | `adl-runtime learn export ...` | Keep shim. | JSONL, bundle-v1, trace-bundle-v2 output, run-id filtering, archive behavior. | Learning/export cards should use `adl-runtime learn export` after #3598. | Later after obsmem/archive docs update. |
| `adl keygen`, `adl sign`, `adl verify` | `adl/src/cli/commands.rs`, `adl/src/signing.rs` | `adl-runtime keygen/sign/verify` for now; possible future `adl-crypto` deferred | Keep shim; do not introduce `adl-crypto` in this mini-sprint. | Key path outputs, signature verification exit codes, embedded-key policy. | Generated cards should not introduce `adl-crypto` yet. | Future follow-on only after three-owner split. |
| `adl identity ...` | `adl/src/cli/identity_cmd.rs` | `adl-runtime identity ...` for v0.91.5; possible future identity binary deferred | Keep shim. | Profile init/show/now output, schema/proof artifact commands, chronosense surfaces. | Identity cards should use `adl-runtime identity` only after #3598. | Future identity split deferred. |
| `adl godel run/inspect/evaluate/affect-slice` | `adl/src/cli/godel_cmd.rs` | `adl-runtime godel ...` for v0.91.5; possible future `adl-godel` deferred | Keep shim. | Artifact path set, JSON summaries, failure handling, runs-dir behavior. | Gödel cards should use `adl-runtime godel` after #3598; no separate `adl-godel` in this wave. | Future split deferred. |
| `adl csm observatory ...` | `adl/src/cli/csm_cmd.rs` | `adl-runtime csm observatory ...` for now; Observatory long-running process handling deferred | Keep shim. | Bundle/json/report output, packet validation failures. | Observatory cards should migrate after runtime/observability docs update. | Future Observatory-specific work. |
| `adl artifact validate-control-path` | `adl/src/cli/artifact_cmd.rs` | `adl-runtime artifact validate-control-path` | Keep shim. | Path validation success/failure and portable diagnostics. | Runtime proof cards migrate after #3598. | Later. |
| `adl/tools/codex_pr.sh ...` | `adl/tools/codex_pr.sh` | `adl-csdlc` wrapper family, later | Keep as compatibility wrapper; not a first-wave binary. | Input-card parsing, `--paths` refusal rules, mode validation, dirty-tree failure. | New cards should not prefer this path unless specifically testing legacy automation. | Future after wrapper migration. |

## Generated-Card Policy

Generated-card validation must be stricter than terminal shim warnings.

- During the initial opt-in warning phase, terminal users may need old commands
  to continue working quietly.
- Generated cards are durable workflow state and must not fossilize deprecated
  commands after an owner binary is proven.
- Therefore generated-card validators should warn or fail on deprecated command
  strings according to card phase, even when terminal shim warnings remain
  opt-in.
- `adl/tools/pr.sh run <issue>` remains the generated-card issue-binding command
  until #3597 changes wrapper truth and updates `AGENTS.md`, templates, and
  skills together.

## Validation Selectors

Use these concrete selectors instead of stale aspirational names:

| Purpose | Selector |
| --- | --- |
| PVF runner baseline | `bash adl/tools/test_run_pvf_validation_lane.sh` |
| PVF CI release policy | `bash adl/tools/test_pvf_ci_release_policy.sh` |
| Provider/runtime focused tests | `cargo test --manifest-path adl/Cargo.toml --test provider_tests -- --nocapture` |
| Prompt-template schema parity | `cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template validate-schemas` |
| Prompt-card structure | `cargo run --manifest-path adl/Cargo.toml -- tooling prompt-template validate-structure --kind <kind> --input <card>` |

Do not use the stale selector `cli::provider_tests`; the provider integration
tests live under the `provider_tests` test target.

## Follow-On Requirements

- #3595 must eliminate `run` ambiguity before any generated card moves runtime
  execution from `adl pr run <adl.yaml>` to `adl-runtime run <adl.yaml>`.
- #3596 must prove `adl-csdlc` as an alias/routing boundary without changing
  issue workflow semantics.
- #3597 must keep `adl/tools/pr.sh` and `workflow-conductor` as the single
  migration spine until templates and skills are updated together.
- #3598 must prove runtime/provider/demo/agent/instrument/learn/crypto aliases
  without inventing a separate `adl-crypto`, `adl-godel`, or `adl-identity`
  split in this wave.
- #3599 must prove review aliases and update review command policy without
  changing review semantics.
- #3600 must review whether additional binaries or workspace crates are ready
  for a later mini-sprint.

## Non-Claims

- This artifact does not implement any new binary.
- This artifact does not deprecate any command at runtime.
- This artifact does not change `adl/tools/pr.sh` canonical status.
- This artifact does not approve workspace crate splitting.
- This artifact does not claim full Rust validation was run for this docs-only
  inventory issue.

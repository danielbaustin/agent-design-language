# CLI Refactor Mini-Sprint Review (#3600)

Issue: #3600
Umbrella: #3592
Review date: 2026-06-03
Status: review complete; follow-ons routed

## Verdict

The first CLI refactor mini-sprint succeeded as an ownership-boundary proof, but
it is not enough to claim the workflow is now appreciably faster or fully
observable.

What is complete:

- `adl-csdlc`, `adl-runtime`, and `adl-review` exist as compatibility binaries.
- `adl/tools/pr.sh` remains the canonical agent-facing issue-work wrapper.
- Runtime YAML execution is separated from C-SDLC issue execution.
- Review tooling has a dedicated compatibility binary.
- Focused compatibility tests are fast enough to serve as local proof lanes.

What is not complete:

- The Rust workspace is still mostly one crate, so many changes still pay broad
  compile/test costs.
- The C-SDLC control plane still lacks deterministic stage logging for slow or
  stuck `doctor`, `finish`, and `closeout` paths.
- Runtime action logging and OpenTelemetry-ready observability remain follow-on
  work, not completed mini-sprint output.
- Docs, skills, and generated-card command policy still need a migration pass
  after the owner binaries have been proven.

Go/no-go:

- Go for further decomposition work.
- No-go on claiming speed/observability completion until `#3610`, `#3609`, and
  `#3556` land or are explicitly replaced.

## Child Issue Closure

| Issue | PR | State | Review result |
| --- | --- | --- | --- |
| `#3593` safety baseline | `#3601` | closed / merged | Passed; established PVF, prompt-template, and command-characterization baseline. |
| `#3594` command inventory | `#3602` | closed / merged | Passed; owner table and generated-card command policy exist. |
| `#3595` run ambiguity | `#3603` | closed / merged | Passed; runtime YAML and issue-id routing fail closed. |
| `#3596` `adl-csdlc` | `#3604` | closed / merged | Passed; C-SDLC compatibility binary exists without replacing `pr.sh`. |
| `#3597` wrapper contract | `#3605` | closed / merged | Passed; migration spine keeps `adl/tools/pr.sh` canonical. |
| `#3598` `adl-runtime` | `#3606` | closed / merged | Passed; runtime compatibility binary exists and rejects C-SDLC ownership drift. |
| `#3599` `adl-review` | `#3608` | closed / merged | Passed; review compatibility binary exists and rejects runtime/C-SDLC drift. |

The umbrella `#3592` should remain open until this `#3600` review PR is merged
and the sprint closeout record names the routed follow-ons.

## Findings

### P1: Compatibility binaries alone do not create the promised speedup

The mini-sprint added owner binaries, but the crate and validation topology are
still not separated enough to make everyday C-SDLC work avoid broad runtime
compile/test costs. This is why the workflow can still feel slow after the
first seven issues merge.

Evidence:

- `adl/Cargo.toml` still defines the owner binaries inside the same `adl` crate.
- `docs/milestones/v0.91.5/REFACTOR_SAFETY_BASELINE_3593.md` explicitly
  states that workspace crate splits were not approved in this wave.
- Focused shell proof is fast, but full CI can still run broad Rust coverage
  when policy surfaces change.

Disposition:

- Routed to `#3610`: Split workspace and test lanes for real validation speedup.

### P1: Observability is still queued, not implemented

The sprint repeatedly exposed silent or low-signal control-plane waits. The
compatibility split did not add deterministic action logs or OpenTelemetry-ready
spans to the binaries or wrappers.

Evidence:

- `docs/milestones/v0.91.5/REFACTOR_SAFETY_BASELINE_3593.md` records a slow or
  silent prompt-template/help path as follow-on observability evidence.
- `docs/milestones/v0.91.5/CLI_RUNTIME_COMPATIBILITY_3598.md` explicitly says
  runtime observability and OpenTelemetry are not complete.
- `#3599` closeout exposed that `pr.sh closeout` can remain silent long enough
  to be indistinguishable from a hang.

Disposition:

- Runtime action logs: `#3556`.
- All-binaries/control-plane logging and OTEL posture: `#3609`.

### P2: Reviewable decomposition plan path moved out of private `.adl` state

The original `#3600` source prompt names
`.adl/docs/TBD/ADL_CLI_DECOMPOSITION_PLAN.md`, but the reviewable public
evidence now lives in tracked milestone artifacts under
`docs/milestones/v0.91.5/`.

Evidence:

- `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md`
- `docs/milestones/v0.91.5/CLI_WRAPPER_MIGRATION_CONTRACT_3597.md`
- `docs/milestones/v0.91.5/CLI_RUNTIME_COMPATIBILITY_3598.md`
- `docs/milestones/v0.91.5/CLI_REVIEW_COMPATIBILITY_3599.md`

Disposition:

- This review packet treats the tracked milestone files as the reviewable truth.
- Future issue bodies should cite tracked milestone artifacts instead of private
  `.adl/docs/TBD` paths when the work has been promoted.

### P2: Docs/skills command adoption is not finished

The owner binaries are proven, but docs, skills, prompt templates, and generated
cards still need a coordinated migration pass. Without that pass, agents will
either keep using old commands forever or move too early to commands that the
wrapper contract does not yet make canonical.

Evidence:

- `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md` says several command
  families should migrate after docs/skills updates.
- `docs/milestones/v0.91.5/CLI_WRAPPER_MIGRATION_CONTRACT_3597.md` keeps
  `adl/tools/pr.sh run <issue>` canonical until the migration gate changes.
- `AGENTS.md` still correctly teaches `adl/tools/pr.sh run <issue>`.

Disposition:

- Routed to `#3611`: Migrate docs and skills to proven CLI owner commands.

### P3: Module navigability remains unreviewed after command split

The mini-sprint intentionally avoided deeper module surgery. That was the right
first step, but it leaves the broader maintainability question open: some
surfaces need extraction, while others may need consolidation to reduce
reviewer file-hopping.

Disposition:

- Routed to `#3612`: Review module navigability and consolidation candidates.

## Follow-On Routing

| Follow-on | Purpose | Status |
| --- | --- | --- |
| `#3556` | Runtime deterministic action logs. | Open |
| `#3607` | Separate local preflight and CI integration proof policy. | Open |
| `#3609` | Deterministic logging and OTEL-ready observability for all binaries/control-plane paths. | Open |
| `#3610` | Real speedup through workspace/test-lane split with timing evidence. | Open |
| `#3611` | Docs, skills, templates, and generated-card command migration. | Open |
| `#3612` | Module navigability/consolidation review. | Open |

## Focused Timing Evidence

Warm-cache local timings from the #3600 worktree:

| Command | Result | Wall time |
| --- | --- | --- |
| `bash adl/tools/test_cli_wrapper_migration_contract.sh` | PASS | `0.05s` |
| `bash adl/tools/test_pr_run_ambiguity_policy.sh` | PASS | `0.13s` |
| `bash adl/tools/test_adl_review_compatibility.sh` | PASS | `0.78s` |
| `bash adl/tools/test_adl_runtime_compatibility.sh` | PASS | `1.31s` |

These focused lanes are fast and useful. They do not replace the need for
`#3610`, because they do not yet change the broader crate/test topology that
drives long CI and local validation cycles.

## Validation

Commands run from the #3600 worktree:

- `bash adl/tools/test_cli_wrapper_migration_contract.sh`
  - Verified `adl/tools/pr.sh` remains canonical, conductor dispatch remains
    wrapper-based, and templates do not prematurely teach `adl-csdlc issue run`.
- `bash adl/tools/test_pr_run_ambiguity_policy.sh`
  - Verified issue-mode and runtime-YAML `run` ambiguity fails closed.
- `bash adl/tools/test_adl_review_compatibility.sh`
  - Verified `adl-review` help/version, review command compatibility, and
    runtime/C-SDLC rejection.
- `bash adl/tools/test_adl_runtime_compatibility.sh`
  - Verified `adl-runtime` help/version, runtime plan compatibility, and
    C-SDLC rejection.

## Non-Claims

- This review does not claim full Rust validation was rerun.
- This review does not claim workspace crate splitting is complete.
- This review does not claim OpenTelemetry or deterministic logging is
  implemented.
- This review does not close the umbrella `#3592`; it supports closeout after
  this review PR is merged and follow-ons are recorded.

# Coding Agent Runner

## Status

Tracked feature contract for the v0.90.5 Comms-06 slice.

This document defines the provider-neutral coding-agent runner that consumes
ACIP coding invocations and returns bounded implementation outputs for normal
ADL review and PR lifecycle handling. It does not replace `pr-run`,
`pr-finish`, `pr-closeout`, or reviewer-agent responsibility.

## Purpose

ADL needs one stable way to ask coding agents for bounded implementation work
without confusing authorship, review, merge, and operator authority.

The coding-agent runner exists so ADL can:

- route bounded coding requests through ACIP
- distinguish direct issue-worktree editing from proposal-only lanes
- require explicit file scope, validation commands, and stop boundaries
- hand coding output to reviewer-agent or equivalent review surfaces before any
  closeout decision

## Core Boundary

The coding-agent runner is an execution specialization, not an approval
surface.

It may:

- consume an ACIP coding invocation contract
- read bounded issue context and task-bundle inputs
- edit inside a bound ADL issue worktree when the lane explicitly permits it
- emit a patch, patch manifest, or structured proposal with validation evidence
- produce a review handoff packet

It must not:

- merge branches
- bless its own work
- bypass reviewer-agent or SRP-governed review
- grant repository authority to non-Codex lanes
- turn remote-provider responses into direct unreviewed repository mutations

## ACIP Alignment

The canonical Comms-06 surface is `acip.coding.invocation.v1` plus its paired
coding outcome and fixture set.

The invocation contract adds coding-specific fields on top of the core ACIP
invocation substrate:

- `provider_lane`
- `execution_mode`
- `issue_ref`
- `task_bundle_ref`
- `issue_worktree_required`
- `allowed_edit_paths`
- `validation_commands`
- `patch_format`
- `approval_policy`

The paired coding outcome returns:

- `primary_output_ref`
- `validation_result_refs`
- `review_handoff_ref`
- preserved writer session/model identity
- a result classification that distinguishes patch-ready worktree edits from
  proposal-ready non-worktree outputs

## Provider Lane Matrix

| Provider lane | Execution mode | Repo mutation authority | Expected primary output |
| --- | --- | --- | --- |
| `codex_issue_worktree` | `worktree_edit` | May edit only through the normal ADL bound issue worktree and only inside declared `allowed_edit_paths` | `patch_manifest.json` |
| `chatgpt_api` | `unapplied_patch` | No direct repo mutation | unified diff / `.diff` artifact |
| `claude_api` | `structured_proposal` or `unapplied_patch` | No direct repo mutation | structured proposal JSON or diff artifact |
| `local_ollama` | `structured_proposal` or `unapplied_patch` | No direct repo mutation | structured proposal JSON or diff artifact |
| `other_proposal_only` | `structured_proposal` or `unapplied_patch` | No direct repo mutation | structured proposal JSON or diff artifact |

The root rule is stable:

- only the Codex issue-worktree lane may use `worktree_edit`
- every other lane is proposal-only and returns artifacts for later review or
  application
- proposal-only lanes advertise proposal capability, not direct `code_edit`
  capability

## Required Inputs

Every coding invocation must remain explicit about:

- the issue/task context it is serving
- the bounded edit surface
- the validation commands expected after the change
- the output contract for patch/proposal and review handoff artifacts
- the policy refs and decision-event linkage inherited from ACIP core

The coding runner should fail closed if `task_bundle_ref` is absent from the
declared `input_refs`, if `allowed_edit_paths` is empty, or if the provider
lane claims more mutation capability than its matrix row allows.

## Output Contract

Required outputs depend on the execution mode:

- `worktree_edit` requires a `patch_manifest`, `validation_summary`, and
  `review_handoff`
- `unapplied_patch` requires a `patch_diff`, `validation_summary`, and
  `review_handoff`
- `structured_proposal` requires a `structured_proposal`,
  `validation_summary`, and `review_handoff`

Those outputs remain evidence and handoff surfaces. They do not constitute a
review decision or merge approval.

## Approval Separation

The coding invocation must carry an approval policy with:

- `review_required_before_pr_finish = true`
- `required_review_schema_ref = acip.review.invocation.v1`
- preserved writer session/model identity
- explicit prohibition of same-session blessing
- explicit prohibition of same-model blessing

This keeps Comms-06 aligned with Comms-05 rather than competing with it. Coding
work can produce a review handoff packet, but reviewer-agent invocation remains
the surface that determines whether the result is blessable.

## Validation And Stop Boundary

The coding-agent runner must record validation expectations up front. At
minimum, the invocation should define a small proving command set such as:

- `cargo fmt --check`
- focused `cargo test` commands
- any bounded artifact-schema or diff checks required by the issue

The stop boundary is also explicit:

- stop once the declared output contract is satisfied
- stop on refusal or failure
- stop when the bounded output-artifact limit is reached
- stop before merge, closeout, or self-approval

## Fixture-Mode Proof

Comms-06 requires a dry-run / fixture-mode proof path that can run without paid
or remote model access.

The fixture-mode proof surface is:

- one valid Codex issue-worktree request that yields a patch-manifest outcome
- one valid proposal-only request that yields a diff/proposal outcome
- negative cases proving that non-Codex lanes cannot use worktree-edit and that
  review bypass or writer-identity drift is rejected

The current focused proof command is the shared ACIP test surface that covers
the coding specialization:

```sh
cargo test --manifest-path adl/Cargo.toml agent_comms --lib -- --nocapture
```

## Interaction With Reviewer-Agent Work

`LOCAL_MODEL_PR_REVIEWER_TOOL.md` remains the concrete backend for the
reviewer-agent side of the protocol.

The coding-agent runner should hand off:

- patch/proposal artifact refs
- validation evidence
- bounded issue/task context
- writer identity

The reviewer-agent specialization then decides whether the work is blocked,
non-proving, skipped, or blessed. The coding runner never makes that call.

# Portable ADL Project Adapter Contract

Status: v0.91.5 Sprint 1 contract

Issue: #3569

## Purpose

The portable ADL project adapter makes external repositories self-describing for
C-SDLC work. An agent starting inside a paper repo, UTS repo, demo repo, runtime
repo, or library repo should be able to determine the applicable ADL workflow,
tooling checkout, issue authority, card policy, worktree policy, validation
profile, and artifact boundaries without relying on session memory.

This contract does not migrate any external repository. It defines the tracked
public surface those repositories should carry when migration is approved.

## Required Repo Surface

An ADL-governed external repository should carry two tracked files at its root:

- `AGENTS.md`
- `adl_project.json`

`AGENTS.md` is the human and agent instruction surface. It points readers to
`adl_project.json`, states that ADL workflow discipline applies, and preserves
the core prohibitions: no tracked issue work on `main`, no hand-rolled cards,
and no hidden local state in public artifacts.

`adl_project.json` is the machine-readable adapter contract. It records project
identity, ADL tooling discovery, version compatibility, issue/card/worktree
ownership, validation profile, and artifact policy.

## Centralized Tooling Rule

External repositories must not vendor or fork the ADL workflow toolchain.

They should reference a canonical ADL checkout for:

- `adl/tools/pr.sh`
- `adl-csdlc` prompt-card and workflow tooling
- prompt-template registries under `docs/templates/prompts/`
- editor skills and lifecycle expectations
- review, validation, and closeout helpers

Thin repo-local wrappers are allowed only when documented in
`adl_project.json`. A wrapper must delegate to the resolved ADL checkout and must
not redefine lifecycle semantics.

## Deterministic Tooling Discovery

Portable repos resolve ADL tooling in this order:

1. `ADL_HOME`, if set.
2. `tooling_discovery.repo_relative`, if configured.
3. `tooling_discovery.sibling_repo`, if configured.
4. Fail closed with setup instructions.

The resolver must not search the whole filesystem, guess silently, or fall back
to stale vendored tooling.

## Valid ADL Checkout

A resolved ADL checkout is valid only if all required checks pass:

- path exists;
- `adl/tools/pr.sh` exists and is executable;
- `docs/templates/prompts/current.json` exists;
- the checkout satisfies `min_adl_version`;
- the checkout matches `tooling_ref`, or emits an explicit compatibility
  warning before continuing;
- required templates, skills, and lifecycle helpers for the configured profile
  are present.

If a required check fails, the future project doctor must fail closed before
issue work starts.

## Required `adl_project.json` Fields

| Field | Required | Description |
|---|---:|---|
| `schema_version` | yes | Adapter schema version. Initial value: `adl.project.v1`. |
| `project_id` | yes | Stable lowercase project identifier. |
| `profile` | yes | One of `paper`, `spec`, `demo`, `runtime`, or `library`. |
| `tooling_ref` | yes | Expected ADL toolchain identity, such as `agent-design-language@v0.91.5`. |
| `min_adl_version` | yes | Minimum compatible ADL version. |
| `prompt_template_registry` | yes | Registry path relative to the ADL checkout. |
| `issue_tracker` | yes | Issue provider and repository authority. |
| `tooling_discovery` | yes | Deterministic ADL checkout resolver settings. |
| `state_policy` | yes | Issue, card, worktree, local state, and public evidence policy. |
| `validation_profile` | yes | Validation lane profile. Usually matches `profile`. |
| `artifact_policy` | yes | Public/private artifact rules and leakage controls. |

Optional future field:

- `prompt_template_version_lock`: pins prompt cards to a specific template set
  during compatibility migrations.

## Allowed Enum Values

Allowed `profile` and `validation_profile` values:

- `paper`
- `spec`
- `demo`
- `runtime`
- `library`

Allowed `state_policy.issue_authority` values:

- `external_repo`: issues and PRs live in the external project repo.
- `adl_repo`: issues and PRs live in the ADL repo.
- `split_explicit`: authority is split and must be declared per issue family.

Allowed `state_policy.cards_location` values:

- `repo_local_ignored_adl`: lifecycle cards live under an ignored repo-local
  `.adl/` tree.
- `tracked_public_adl_subset`: only declared public review/evidence cards may
  be tracked.
- `external_adl_state_root`: lifecycle state lives in a declared non-default
  state root.

Allowed `state_policy.worktree_location` values:

- `external_repo_worktrees`: worktrees are managed in the external repo.
- `adl_managed_worktrees`: worktrees are managed by ADL tooling in a declared
  location.
- `disabled`: no worktrees are used for this project type.

Allowed `tooling_discovery.failure_mode` value:

- `fail_with_setup_instructions`

## State Ownership Defaults

The default external-repo policy is:

- GitHub issues live in the external repository.
- Pull requests live in the external repository.
- Worktrees live under the external repository's normal worktree area.
- Repo-local `.adl/` lifecycle cards are ignored unless a repo explicitly
  chooses a tracked public subset.
- Public review/evidence packets are tracked only when they are intended review
  surfaces.
- Private notes, build outputs, credentials, temp files, and scratch state stay
  ignored.

This prevents agents from accidentally creating issue truth in ADL when the
work belongs to UTS, a paper repo, or a demo repo.

## Project Profiles

Profiles guide validation and artifact policy without changing the canonical
`SIP -> STP -> SPP -> SRP -> SOR` lifecycle.

### `paper`

Default validation lanes:

- citation key scan;
- duplicate bibliography key scan;
- LaTeX build when local dependencies are available;
- PDF freshness check;
- forbidden-claim scan.

Public artifacts:

- review packet;
- final or private-review PDF when explicitly intended;
- source package only when publication is approved.

Private artifacts:

- local build directories;
- draft notes;
- reviewer scratch comments.

### `spec`

Default validation lanes:

- schema parse checks;
- example parse checks;
- conformance example checks;
- docs link checks where available;
- evidence packet path hygiene.

### `demo`

Default validation lanes:

- demo build or smoke check;
- screenshot or transcript freshness check when relevant;
- operator runbook validation;
- public artifact path hygiene.

### `runtime`

Default validation lanes:

- focused unit/integration tests for touched runtime surfaces;
- provider credential boundary checks;
- benchmark evidence hygiene;
- PVF lane classification when tests are added.

### `library`

Default validation lanes:

- package tests;
- API docs;
- examples;
- release notes or changelog checks.

## External Repo `AGENTS.md` Template

The reusable template lives at:

```text
docs/templates/portable-adl/1.0.0/AGENTS.md
```

It intentionally stays compact. External repos should fill the declared values
and avoid copying the full ADL root `AGENTS.md`.

## External Repo `adl_project.json` Template

The reusable template lives at:

```text
docs/templates/portable-adl/1.0.0/adl_project.json
```

Example configs live under:

```text
docs/templates/portable-adl/1.0.0/examples/
```

## Fail-Closed Rules

Portable ADL startup must fail before issue work if:

- no valid ADL checkout can be resolved;
- the configured ADL checkout is older than `min_adl_version`;
- the prompt-template registry is missing;
- issue/card/worktree authority is ambiguous;
- `adl_project.json` contains unsupported enum values;
- public artifacts would include host-local absolute paths;
- private `.adl` state, credentials, build output, or scratch state are marked
  for public tracking without explicit policy.

## Non-Claims

This contract does not claim:

- external repos are migrated;
- the read-only project doctor is implemented;
- all profile-specific validations exist;
- local paper/demo/UTS repos are ready for public review;
- ADL tooling can run without a valid canonical ADL checkout.

## Follow-On Routing

Tracked follow-on candidates are recorded in
`docs/tooling/PORTABLE_ADL_PROJECT_ADAPTER_FOLLOW_ONS_v0.91.5.md`.

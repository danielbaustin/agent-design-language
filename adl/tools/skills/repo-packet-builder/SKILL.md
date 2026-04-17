---
name: repo-packet-builder
description: Build a bounded, source-grounded repository review packet for CodeBuddy-style specialist review lanes before any review, diagram, test, issue, or report skill runs.
---

# Repo Packet Builder

Build the shared evidence packet that downstream CodeBuddy review skills consume.
This skill is a packet-construction skill, not a reviewer and not a remediation
workflow.

The packet should make scope explicit before specialist agents start reading:
what repo/ref was reviewed, which paths were included or excluded, which
manifests/docs/tests/config surfaces exist, which high-signal files should be
sampled first, and which specialist lanes should receive which evidence.

## Quick Start

1. Confirm the review target:
   - whole repository
   - path slice
   - branch or diff context
   - existing review packet refresh
2. Run the deterministic packet helper when local filesystem access is
   available:
   - `scripts/build_repo_packet.py <repo-root> --out <artifact-root>`
3. Review the generated packet for scope, exclusions, assumptions, and
   specialist assignments.
4. Stop after writing the packet. Hand it to review, diagram, test, redaction,
   issue-planning, or report-writing skills as separate downstream work.

## Required Inputs

At minimum, gather:

- `repo_root`
- `mode`
- `target`
- `policy`

Supported modes:

- `build_repository_packet`
- `build_path_packet`
- `build_branch_packet`
- `build_diff_packet`
- `refresh_packet`

Useful target fields:

- `target_path`
- `branch`
- `diff_base`
- `changed_paths`
- `existing_packet_path`
- `artifact_root`

Useful policy fields:

- `scope_policy`
- `privacy_mode`
- `include_generated_code`
- `include_vendor_code`
- `context_budget`
- `specialist_lanes`
- `stop_before_review`

If there is no concrete repo root or target scope, stop and report `blocked`.

## Workflow

### 1. Establish Scope

Record:

- review mode
- included paths
- excluded paths
- non-reviewed surfaces
- assumptions
- known limits
- whether the packet is for public, customer-private, or local-only use

Do not silently expand from a path or diff review into a whole-repo review.

### 2. Inventory The Repo

Identify:

- tracked files
- top-level directories
- dominant file extensions
- likely application roots
- likely code roots
- docs surfaces
- test surfaces
- CI/build/tooling surfaces
- package manifests and lockfiles
- generated, vendored, build-output, and cache-heavy surfaces
- largest files and largest code files

Prefer `git ls-files` for deterministic tracked-file inventory. Fall back to a
sorted filesystem walk only when the target is not a Git repository.

### 3. Build Evidence Index

Create relative-path evidence references only. Do not write absolute host paths
into the packet artifacts unless the operator explicitly requests a private
local diagnostic packet.

Each evidence entry should include:

- relative path
- evidence category
- line count when available
- reason it is high signal
- suggested specialist lanes

### 4. Assign Specialist Lanes

Assign bounded packet slices to downstream lanes:

- `code`
- `security`
- `tests`
- `docs`
- `architecture`
- `dependencies`
- `diagrams`
- `redaction`
- `synthesis`

Keep assignments as recommendations. This skill does not run the specialists.

### 5. Emit Packet Artifacts

Default artifact root:

```text
.adl/reviews/codebuddy/<run_id>/
```

Required artifacts:

- `run_manifest.json`
- `repo_scope.md`
- `repo_inventory.json`
- `evidence_index.json`
- `specialist_assignments.json`

Optional artifacts:

- `packet_notes.md`
- `redaction_todo.md`

## Output Expectations

Default output should include:

- artifact root
- review mode
- included and excluded surfaces
- generated files
- packet quality caveats
- next recommended skill lanes

Use the output contract in `references/output-contract.md`.

## Stop Boundary

Stop after producing or validating the packet.

Do not:

- perform code, security, docs, tests, dependency, or architecture review
- generate diagrams
- generate tests
- create issues
- write product reports
- mutate customer repositories
- publish packet artifacts externally
- claim review findings or remediation readiness

## Privacy Boundary

The default packet is customer-safe in shape but not automatically publishable.
Run `redaction-and-evidence-auditor` before any public/customer-facing report.

Packet artifacts should use repo-relative paths. Avoid absolute paths, secrets,
raw prompts, raw tool arguments, and excessive source excerpts.


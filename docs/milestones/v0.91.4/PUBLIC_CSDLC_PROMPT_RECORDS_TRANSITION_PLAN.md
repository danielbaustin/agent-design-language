# Public C-SDLC Prompt Records Transition Plan

## Status

Planning and contract packet for issue `#3471`.

This document defines the v0.91.4 transition from local-only C-SDLC prompt
cards toward public, tracked prompt records. It does not claim that all prompt
records have already been exported, that local `.adl/` state has been cleaned,
or that ObsMem ingestion has happened.

## Purpose

ADL is moving from local execution-cache cards to public, inspectable C-SDLC
workflow truth.

The prompt lifecycle remains:

```text
SIP -> STP -> SPP -> SRP -> SOR
```

The durable public record should be tracked in Git. Local `.adl/` state should
shrink toward execution cache, temporary staging, local logs, and explicitly
local archive inputs.

## Source Inputs

- `#3471`: public prompt-record transition contract
- `#3472`: public prompt packet exporter
- `#3473`: local `.adl` inventory and disposition
- `#3474`: pilot public prompt packets and reviewer index
- `#3475`: public prompt packet validation and redaction gates
- `#3476`: public prompt-record mini-sprint umbrella
- `docs/planning/C_SDLC_PROMPT_TEMPLATE_EDITOR_TRANSITION_PLAN.md`
- `docs/planning/DESIGN_TIME_CARD_COMPLETION_PLAN.md`
- `docs/milestones/v0.91.4/C_SDLC_TRACKED_WORKFLOW_STATE_MIGRATION_PLAN_v0.91.4.md`
- `docs/milestones/v0.91.4/features/ACTIVE_ISSUE_MIGRATION_POLICY.md`
- `docs/templates/prompts/current.json`
- `docs/templates/prompts/1.0.0/`
- `docs/milestones/v0.91.4/review/evidence/csdlc/`

## Decision

C-SDLC prompt records should become public tracked workflow evidence by
default for future ADL software-development issues.

The tracked record is not a direct commit of `.adl/`. It is a sanitized,
reviewed, repo-relative packet exported from local execution state and linked to
GitHub issue/PR truth.

## Public Packet Namespace

Prompt packets should live under:

```text
docs/milestones/v0.91.4/review/evidence/csdlc/issues/<issue-number>-<slug>/
```

Required files:

```text
manifest.json
sip.md
stp.md
spp.md
srp.md
sor.md
```

Optional files:

```text
plan-history.md
evidence/
traces/
README.md
```

The same shape can be reused by later milestones with the milestone version in
the path.

## Manifest Contract

Each public prompt packet should include a manifest with this minimum shape:

```json
{
  "schema": "adl.csdlc.public_prompt_packet.v1",
  "issue_number": 3471,
  "work_item": {
    "system": "github",
    "id": "3471",
    "url": "https://github.com/danielbaustin/agent-design-language/issues/3471"
  },
  "slug": "v0-91-4-docs-tools-publish-csdlc-prompt-records-and-archive-local-adl-state",
  "milestone": "v0.91.4",
  "template_set": "1.0.0",
  "source_state": "exported_from_local_execution_cache",
  "packet_status": "pilot",
  "lifecycle_status": {
    "sip": "ready",
    "stp": "ready",
    "spp": "approved",
    "srp": "ready",
    "sor": "draft"
  },
  "tracked_records": {
    "sip": "sip.md",
    "stp": "stp.md",
    "spp": "spp.md",
    "srp": "srp.md",
    "sor": "sor.md"
  },
  "validation": {
    "structured_prompt_validation": "pending",
    "manifest_validation": "pending",
    "redaction_scan": "pending"
  },
  "redaction": {
    "secrets_removed": true,
    "absolute_host_paths_removed": true,
    "raw_private_memory_removed": true,
    "unsafe_tool_arguments_removed": true
  }
}
```

The `work_item.system` field keeps the packet portable. GitHub is ADL's first
implementation substrate, but Jira, Linear, GitLab, or another tracker can map
to the same public packet shape if the adapter supplies stable work-item IDs and
links.

## Safe Public Content

Public prompt packets may include:

- issue number, slug, title, labels, branch, PR, and tracker URL
- template version and prompt lifecycle status
- issue goal, required outcome, acceptance criteria, non-goals, and constraints
- selected task framing and target tracked surfaces
- issue-local `SPP` plan and material replan history
- `SRP` review prompt, findings, dispositions, and residual risks
- `SOR` changed-path, validation, PR, merge, and closeout truth
- links to tracked proof, trace, demo, review, and release evidence

Public prompt packets must not include:

- secrets, tokens, credentials, private keys, or local key-file paths
- raw private session memory
- raw sensitive tool arguments or prompt payloads
- absolute host paths, temporary-directory paths, or local worktree-only
  scratch paths
- local hostnames, private IPs, or machine-specific execution assumptions unless
  explicitly sanitized and justified
- unreviewed local `.adl` scratch material as canonical truth

## Local `.adl` Disposition Classes

Issue `#3473` owns the detailed inventory. This contract uses these classes:

| Class | Meaning | Default action |
| --- | --- | --- |
| `ephemeral_cache` | Generated cache, package cache, browser/runtime cache, temporary helper output. | Safe cleanup after operator review. |
| `local_execution_cache` | Active issue cards, local run records, task bundles, transient workflow state. | Export sanitized public packets when durable; keep local copy only as cache. |
| `archive_for_obsmem` | High-value historical cards, review packets, run summaries, or decision records useful for memory ingestion. | Archive or prepare ingestion packet; do not track raw local source wholesale. |
| `tracked_promotion_candidate` | Local record that should become public evidence after sanitization. | Export to tracked evidence namespace with manifest. |
| `local_provenance_only` | Historical context useful locally but not worth public tracking. | Keep locally or archive outside tracked repo. |
| `safe_delete_candidate` | Obvious generated cruft, duplicate temp files, stale caches. | Delete only after reviewed cleanup issue. |
| `blocked_sensitive` | Possible secrets, private notes, raw logs, credentials, or ambiguous private state. | Do not publish or delete without operator review. |

## Mini-Sprint Tranche

This transition is intentionally split:

| Issue | Role |
| --- | --- |
| `#3471` | Define this public prompt-record contract and transition plan. |
| `#3472` | Add or specify exporter tooling for public prompt packets. |
| `#3473` | Inventory local `.adl` state and classify cleanup/archive/ObsMem disposition. |
| `#3475` | Add validation/redaction gates for public prompt packets. |
| `#3474` | Produce pilot public packets and reviewer index. |
| `#3476` | Umbrella closeout and mini-sprint status. |

Suggested execution order:

1. Land the contract.
2. Inventory `.adl` state before deleting or promoting anything.
3. Build or specify the exporter.
4. Add validation/redaction gates.
5. Produce a small pilot packet set.
6. Close the umbrella with completed, blocked, and routed state.

## Pilot Scope

The pilot should include at least:

- one docs issue
- one tooling/process issue
- one demo, multi-agent, or review issue

The pilot should prefer recent v0.91.4 issues with clear GitHub and PR truth.
If complete closeout truth is not available for a selected issue, the pilot
must record that limitation instead of making the packet look artificially
complete.

## Validation Expectations

Public prompt packets should run focused validation:

- structured prompt validation for `SIP`, `STP`, `SPP`, `SRP`, and `SOR`
- manifest JSON parse and required-field check
- repo-relative path check
- leakage scan for secrets, key-file names, absolute host paths, private temp
  paths, local worktree paths, raw private memory, and unsafe `.adl` canonical
  references
- Markdown link/path existence checks for tracked references

These checks belong in the PVF `contract_schema_card` or docs/contract lane.
They should not trigger broad Rust or slow-proof validation unless tooling code
changes.

## Closeout Integration Target

Future closeout should be able to:

1. read the local issue bundle
2. sanitize the prompt records
3. write the tracked public packet
4. write or update `manifest.json`
5. run validation/redaction gates
6. link the packet from `SOR`, review evidence, and milestone closeout
7. keep `.adl` as local cache rather than durable public truth

The first implementation may be a bounded helper. The default-operation target
is integration with the normal `pr finish` / `pr closeout` path.

## Non-Claims

This plan does not claim:

- all historical `.adl` state has been migrated
- `.adl` is safe to delete wholesale
- public prompt packets are already complete for every v0.91.4 issue
- ObsMem ingestion has happened
- Jira or other tracker adapters are implemented
- validation gates replace human review

## Review Questions

WP-15 and WP-16 should ask:

- Does the packet shape let a reviewer understand issue lifecycle truth without
  local `.adl` access?
- Does it avoid tracking unsafe local execution state?
- Are prompt records public enough to support C-SDLC default-operation claims?
- Is `.adl` disposition conservative enough to preserve high-value history?
- Are validation and redaction gates strong enough for third-party review?
- Is the design portable beyond GitHub as a tracker?

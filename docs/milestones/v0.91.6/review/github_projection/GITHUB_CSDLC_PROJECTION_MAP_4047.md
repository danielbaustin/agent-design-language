# GitHub/C-SDLC Projection Map For #4047

Status: implemented as a report-only convergence surface.

Issue: #4047

## Summary

`#4047` makes GitHub/C-SDLC projection ownership explicit. The goal is not to rewrite every GitHub surface automatically. The goal is to make it deterministic which surfaces ADL owns, which surfaces ADL checks for drift, which surfaces remain linked external state, which surfaces are local card truth, and which surfaces are explicitly deferred.

The command surface is:

```bash
adl/tools/pr.sh projection-map
adl/tools/pr.sh projection-map --json
```

The Rust command emits schema `adl.github_csdlc_projection_map.v1`.

## Projection Policies

| Policy | Meaning |
| --- | --- |
| `managed_projection` | ADL owns the projection and may create, update, or repair the GitHub surface through the typed workflow path. |
| `drift_checked_projection` | ADL checks or records drift and may offer bounded repair, but does not treat the surface as freely rewritable. |
| `linked_surface_only` | ADL links to or reads the surface, but GitHub remains the authority. |
| `card_local_only` | The surface belongs to local C-SDLC cards/templates and is not projected wholesale into GitHub. |
| `explicitly_deferred` | The surface is intentionally out of scope for this tranche and must be routed separately before automation claims are made. |

## Current Surface Map

| Surface | Policy | Current status | Notes |
| --- | --- | --- | --- |
| `github.issue.title` | `managed_projection` | implemented | Managed by issue creation/init/repair metadata parity. |
| `github.issue.labels` | `managed_projection` | implemented | Required labels are created/applied through metadata parity. |
| `github.issue.body` | `drift_checked_projection` | implemented | Repair refuses to overwrite authored prompt/body content without explicit force. |
| `github.pr.title` | `managed_projection` | implemented | Owned by `pr finish`. |
| `github.pr.body` | `managed_projection` | implemented | Owned by `pr finish` from SOR/output-card truth. |
| `github.pr.closing_linkage` | `managed_projection` | implemented | Finish renders linkage; Rust/PVF owns the canonical live-metadata-first guard, while the shell helper remains a deterministic compatibility fallback for always-on minimal CI lanes. |
| `github.pr.validation_checks` | `linked_surface_only` | implemented | Reported by `pr validation`; GitHub Actions remains the source. |
| `github.review_comments` | `linked_surface_only` | implemented by process | Routed through review triage/janitor work, not rewritten as local state. |
| `github.closeout_comment` | `drift_checked_projection` | partially implemented | Closeout verifies local/GitHub closure truth; typed final-comment automation remains follow-on work. |
| `github.milestone_and_project_fields` | `explicitly_deferred` | deferred | Needs a separate typed milestone/project projection tranche. |
| `csdlc.cards.sip_stp_spp_srp_sor` | `card_local_only` | implemented | Cards are local prompt-template/editor-skill truth surfaces. |

## Relationship To Earlier Work

`#4255` already landed the immediate typed issue-close and stale wording fix. This tranche therefore does not duplicate that work.

Existing metadata parity is already Rust-owned. The retired `check_issue_metadata_parity.sh` path now points operators toward `pr doctor` or future Rust/PVF validation lanes.

`adl/tools/pr.sh closing-linkage` is now the authoritative Rust/PVF guard for PR closing linkage. `check_pr_closing_linkage.sh` remains only as a deterministic compatibility helper so always-on minimal CI lanes can avoid compiling Rust; it delegates to Rust only through an explicit binary override.

## Non-Claims

This does not claim:

- automatic GitHub milestone/project-field synchronization
- automatic rewrite of review comments
- full closeout-comment projection
- removal of every legacy shell guard
- replacement of C-SDLC card truth with GitHub issue bodies

## Validation Surface

Focused validation should prove:

- `projection-map --json` emits parseable schema `adl.github_csdlc_projection_map.v1`
- all five projection policies are represented
- the required GitHub and C-SDLC surfaces are represented
- public `pr.sh projection-map` delegates to the Rust command

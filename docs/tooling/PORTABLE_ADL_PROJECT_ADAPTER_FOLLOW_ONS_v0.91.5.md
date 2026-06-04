# Portable ADL Project Adapter Follow-Ons

Status: v0.91.5 Sprint 1 follow-on register

Issue: #3569

## Purpose

This register keeps the portable ADL adapter work from disappearing after the
contract issue lands. It records the follow-on issues that should be opened or
scheduled before external repositories depend on this adapter.

## Follow-On Issue Candidates

| Order | Candidate Title | Purpose | Suggested Timing |
|---:|---|---|---|
| 1 | `[v0.91.5][portable-adl] Implement read-only portable project doctor` | Add `adl-csdlc project doctor` with deterministic config/tooling validation. | v0.91.5 after contract review |
| 2 | `[v0.91.5][portable-adl] Pilot adapter in cognitive-sdlc-paper` | Add repo-local `AGENTS.md` and `adl_project.json`; prove paper profile startup. | after doctor or with explicit manual review |
| 3 | `[v0.91.5][portable-adl] Pilot adapter in general-intelligence-paper` | Add repo-local adapter files and paper-profile validation notes. | after first paper pilot |
| 4 | `[v0.91.5][portable-adl] Pilot adapter in universal-tool-schema` | Prove runtime/spec profile boundaries without making UTS depend on ADL at runtime. | after doctor and paper pilots |
| 5 | `[v0.91.5][portable-adl] Add portable adapter docs to C-SDLC skills` | Update workflow-conductor, pr lifecycle, and editor skill guidance to discover external project contracts. | after first pilot evidence |
| 6 | `[v0.91.5][portable-adl] Add CI smoke fixture for adapter templates` | Validate template JSON examples and fail on host-local absolute paths. | after template review |

## Dependency Notes

- The doctor should remain read-only.
- Repo pilots should not copy the full ADL toolchain.
- Public `AGENTS.md` surfaces should stay compact and point to
  `adl_project.json`.
- UTS must remain independent at runtime; ADL workflow guidance must not become
  a runtime dependency.
- Skill updates should wait until the adapter contract survives at least one
  pilot or explicit review.

## Non-Goals

This register does not create the issues by itself and does not migrate any
external repository.

# Work Breakdown Structure (WBS) — v0.75

## Metadata
- Milestone: v0.75
- Version: 0.75
- Date: 2026-03-02
- Owner: Daniel / Agent Logic team

## WBS Summary
v0.75 ships **EPIC‑A + EPIC‑B**: a frozen deterministic substrate (activation log + replay + trace bundle v2 + failure taxonomy) plus **ObsMem v1** (ingest + deterministic retrieval with citations). The work is intentionally sliced into independently-mergeable packages, reserving the tail for demos, quality gate, docs/review, and release ceremony.

## Work Packages

| ID | Work Package | Description | Deliverable | Dependencies | Issue |
|---|---|---|---|---|---|
| WP-01 | Design pass (milestone docs + planning) | Create/complete v0.75 planning docs (VISION, DESIGN, WBS, DECISIONS, SPRINT, CHECKLIST, RELEASE_PLAN/NOTES). Break down issues for WP-02..WP-12. | Canonical planning docs in `.adl/docs/v075planning/` with no placeholders; issue list created. | — | TBD |
| WP-02 | Activation log schema freeze | Define and freeze the activation log schema required for replay; document invariants and stable identifiers; add schema validation tests. | Activation log schema doc + tests; stable identifiers/invariants. | WP-01 | TBD |
| WP-03 | Replay runner hardening | Ensure replay consumes activation log + captured boundary events and reproduces outputs/artifact layout deterministically; add regression tests. | Replay tests (byte/structure equivalence where applicable); docs for replay semantics. | WP-02 | TBD |
| WP-04 | Failure taxonomy stabilization | Define stable machine-readable failure codes and deterministic classification mapping; add coverage for key codes. | Failure taxonomy doc + tests; stable error identifiers. | WP-02 | TBD |
| WP-05 | Trace bundle v2 spec + export | Specify trace bundle v2 manifest and canonical serialization rules; implement export and round-trip validation. | Trace bundle v2 manifest + export command; round-trip tests. | WP-02, WP-03 | TBD |
| WP-06 | Trace bundle v2 import + replay-sufficient proof | Implement import path and prove replay sufficiency for representative workflows; document constraints. | Import tooling + replay-from-bundle proof tests; documentation. | WP-05 | TBD |
| WP-07 | ObsMem v1 index schema | Define versioned ObsMem index schema for runs/activations/evidence; implement storage layer and migrations (additive). | Versioned index schema + storage implementation + tests. | WP-05 | TBD |
| WP-08 | ObsMem v1 ingestion pipeline | Implement ingest of trace bundles into index; ensure deterministic parsing and stable IDs; basic integrity checks. | `obsmem ingest` capability + tests; ingestion report/log. | WP-07 | TBD |
| WP-09 | ObsMem v1 query (structured) | Implement deterministic structured queries (filters by workflow/failure/tool/time); stable ordering + tie-break rules. | `obsmem query` (structured) + deterministic ordering tests. | WP-08 | TBD |
| WP-10 | ObsMem v1 retrieval (hybrid optional) + ranking | Add optional semantic retrieval integration (if enabled) with recorded model/config; deterministic ranking and tie-break; explanations. | Hybrid retrieval option + deterministic rank/explain tests; recorded config. | WP-09 | TBD |
| WP-11 | Citations + evidence rendering | Implement citations that point back into trace bundle artifacts; add “show evidence” rendering for top results. | Citation format + evidence renderer + tests; docs. | WP-10 | TBD |
| WP-12 | Operational report surfaces | Deterministic summary reports over the index (counts, failure classes, latency/cost aggregates); ensure stable formatting and ordering. | `obsmem report` (or equivalent) + deterministic output tests; docs. | WP-08 | TBD |
| WP-13 | Demo matrix + integration demos | Author and validate v0.75 demo packs (Determinism+Replay; Ingest+Query; Operational report). Ensure commands run from docs on a fresh checkout. | `docs/milestones/v0.75/DEMOS_v0.75.md` (or equivalent) + runnable examples. | WP-03, WP-06, WP-12 | TBD |
| WP-14 | Coverage / quality gate (ratchet + exclusions) | Enforce workspace coverage/quality thresholds for v0.75; add deterministic CI gates (no secrets/host paths; required checks). | CI gates + coverage ratchet + flake elimination; documented commands. | WP-02..WP-12 | TBD |
| WP-15 | Docs + review pass (repo-wide alignment) | Repo-wide docs alignment, link checks, demo command verification, contract wording audit; queue follow-ups. | Updated docs; review notes; follow-up issues. | WP-13, WP-14 | TBD |
| WP-16 | Release ceremony (final validation + tag + notes + cleanup) | Final validation run, tag v0.75, publish release notes, close issues, and prepare next milestone skeleton. | Release notes + tag + clean issue state + next milestone scaffolding. | WP-15 | TBD |

## Sequencing
- Phase 1 (Substrate freeze): WP-01 → WP-02 → WP-03 + WP-04 → WP-05 → WP-06
- Phase 2 (ObsMem v1): WP-07 → WP-08 → WP-09 → WP-10 → WP-11 → WP-12
- Phase 3 (Convergence): WP-13 → WP-14 → WP-15 → WP-16

## Acceptance Mapping
- WP-01 (Design pass) -> All planning docs exist, are consistent with v0.75 scope (EPIC‑A/B), and contain no placeholders.
- WP-02 -> Activation log schema documented, validated, and frozen; tests enforce invariants.
- WP-03 -> Replay determinism demonstrated with regression coverage; reproducible artifact layout.
- WP-04 -> Stable failure codes and deterministic mapping; tests cover key classifications.
- WP-05 -> Trace bundle v2 export implemented; manifest/versioning/canonicalization documented and tested.
- WP-06 -> Trace bundle import works; replay-from-bundle demonstrated for representative workflows.
- WP-07 -> ObsMem index schema versioned; storage layer tested; additive migrations supported.
- WP-08 -> ObsMem ingest is deterministic and stable; integrity checks prevent corrupted bundles.
- WP-09 -> Structured query returns deterministically ordered results with documented tie-break rules.
- WP-10 -> Hybrid retrieval (optional) records model/config; ranking is deterministic; explanations are produced.
- WP-11 -> Citations point to concrete trace artifacts; evidence rendering is stable and tested.
- WP-12 -> Operational reports are deterministic, stable formatted, and tested.
- WP-13 (Demos) -> Demo A/B/C run from docs on a fresh checkout and produce expected artifacts.
- WP-14 (Quality gate) -> CI gates enforce coverage + hygiene deterministically; no flakes; no secrets/host paths.
- WP-15 (Docs/review) -> Repo-wide alignment complete; demo commands verified; contradictions resolved; follow-ups queued.
- WP-16 (Release ceremony) -> Final validation complete; v0.75 tagged; release notes published; milestone closed cleanly.

## Exit Criteria
- Every in-scope requirement in VISION_0.75.md and DESIGN_0.75.md maps to at least one WP.
- Each WP has a concrete deliverable and dependency ordering sufficient to execute deterministically.
- Tail WPs (13–16) ensure convergence: demos, quality gate, docs/review, release.

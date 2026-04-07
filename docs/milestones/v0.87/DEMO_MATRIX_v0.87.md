# Demo Matrix: v0.87

## Metadata
- Milestone: `v0.87`
- Version: `0.87`
- Date: `2026`
- Owner: `adl`
- Related issues / work packages: `WP-02` through `WP-16` (issue numbers TBD)

## Purpose
Define the canonical milestone demo program for `v0.87`: which bounded demos exist, which substrate claims they prove, how to run them, and what artifacts or proof surfaces reviewers should inspect.

This milestone is substrate-heavy. The demos therefore focus on:
- trace truth
- provider portability
- shared-memory coherence
- operational/control-plane stability
- reviewer-facing proof surfaces

They are intended to prove that `v0.87` makes ADL more coherent, deterministic, and externally credible.

## Scope

In scope for `v0.87`:
- trace v1 substrate and reconstruction-oriented proof surfaces
- provider / transport substrate portability and configuration truth
- shared ObsMem foundation with trace-linked memory behavior
- operational skills and control-plane/tooling substrate
- canonical reviewer-facing proof surfaces for the substrate

Out of scope for `v0.87`:
- identity-bearing persistent agents
- full PR Demo society behavior
- capability-aware routing and later social/governance layers

## Runtime Preconditions

Working directory:

```bash
cd /Users/daniel/git/agent-design-language
```

Deterministic runtime / provider assumptions:

```bash
# Use repo-owned commands and fixtures only.
# Prefer deterministic/local provider or mock/fixture-backed runs where possible.
# Record exact commands used in issue output cards.
```

Additional environment / fixture requirements:
- repository is on the intended branch/worktree for the reviewed `v0.87` issue
- any required local/mock provider or fixture data is documented in the issue or demo helper script

## Related Docs
- Design contract: `docs/milestones/v0.87/DESIGN_v0.87.md`
- WBS / milestone mapping: `docs/milestones/v0.87/WBS_v0.87.md`
- Sprint / execution plan: `docs/milestones/v0.87/SPRINT_v0.87.md`
- Release / checklist context: `docs/milestones/v0.87/RELEASE_PLAN_v0.87.md`, `docs/milestones/v0.87/MILESTONE_CHECKLIST_v0.87.md`
- Other proof-surface docs: `docs/milestones/v0.87/FEATURE_DOCS_v0.87.md`, `docs/milestones/v0.87/README.md`

## Demo Coverage Summary

Use this table as the fast review surface for milestone coverage.

| Demo ID | Demo title | Milestone claim / WP proved | Command entry point | Primary proof surface | Success signal | Determinism / replay note | Status |
|---|---|---|---|---|---|---|---|
| D1 | Trace v1 substrate truth | `WP-02`, `WP-03` | `bash adl/tools/demo_v087_trace_truth.sh` | `artifacts/v087/trace_v1/runs/v0-6-hitl-no-pause-demo/logs/trace_v1.json` | stable structured trace events and run artifacts are emitted from a mock-provider-backed run | event vocabulary and structure should be stable even if timestamps vary | READY |
| D2 | Provider portability substrate | `WP-04`, `WP-05` | `bash adl/tools/demo_v087_provider_portability.sh` | `artifacts/v087/provider_portability/provider_substrate_manifest.v1.json` | one canonical config surface resolves into a normalized provider substrate manifest without brittle provider-native leakage | compare normalized config and printed plan surfaces across reruns | READY |
| D3 | Shared ObsMem foundation coherence | `WP-06`, `WP-07` | `bash adl/tools/demo_v087_shared_obsmem.sh` | `artifacts/v087/shared_obsmem/demo-f-obsmem-retrieval/obsmem_retrieval_result.json` | bounded shared-memory retrieval is tied to persisted ObsMem artifacts and trace output | replay judged by stable memory schema and trace-linkage, not by identical timestamps | READY |
| D4 | Operational skills substrate | `WP-08`, `WP-11` | `bash adl/tools/demo_v087_operational_skills.sh` | `artifacts/v087/skills/skills_inventory.txt` | a bounded operational skill substrate is installed, enumerated, and validated through the tracked skills root | output schema and required sections should remain stable | READY |
| D5 | Control-plane / PR tooling substrate | `WP-09`, `WP-10` | `bash adl/tools/demo_v087_control_plane.sh` | `artifacts/v087/control_plane/runs/v0-4-demo-deterministic-replay/run_summary.json` | the repo-owned control-plane run surface drives a deterministic workflow and emits truthful run-state artifacts | replay judged by command surface, output structure, and workflow semantics | READY |
| D6 | Reviewer-facing substrate package | `WP-12`, `WP-13`, `WP-15` | `bash adl/tools/demo_v087_reviewer_package.sh` | `artifacts/v087/reviewer_package/reviewer_package_manifest.txt` | an uninvolved reviewer can locate the canonical docs and validated review-surface examples from one bounded manifest | stability is judged by legibility and proof-surface consistency across reruns | READY |

Status guidance:
- `PLANNED` = intended but not yet validated
- `READY` = runnable and locally validated
- `BLOCKED` = known dependency or missing proof surface
- `LANDED` = milestone evidence exists and is ready for review

## Coverage Rules
- Every major substrate band in `v0.87` should map to a runnable demo or an explicit alternate proof surface.
- Every demo should name one primary proof surface that a reviewer can inspect directly.
- Commands should be copy/paste-ready once implementation lands.
- Success signals should say what to inspect, not just “command exits 0”.
- Determinism / replay notes should explain how stability is judged for substrate-heavy demos.
- Where a demo is not yet runnable, the matrix must remain truthful and say `PLANNED` rather than implying completion.

## Demo Details

### D1) Trace v1 substrate truth

Description:
- prove that `v0.87` emits a stable, bounded trace vocabulary for the milestone’s major control points
- show that trace is reconstruction-oriented and linked to the actual artifact/proof surfaces produced by the run

Milestone claims / work packages covered:
- trace v1 schema and event taxonomy are authoritative
- runtime/control surfaces emit reviewable trace events tied to real execution

Commands to run:

```bash
bash adl/tools/demo_v087_trace_truth.sh
```

Expected artifacts:
- `artifacts/v087/trace_v1/README.md`
- `artifacts/v087/trace_v1/runs/v0-6-hitl-no-pause-demo/logs/trace_v1.json`
- `artifacts/v087/trace_v1/runs/v0-6-hitl-no-pause-demo/run_summary.json`

Primary proof surface:
- `artifacts/v087/trace_v1/runs/v0-6-hitl-no-pause-demo/logs/trace_v1.json`

Secondary proof surfaces:
- `artifacts/v087/trace_v1/runs/v0-6-hitl-no-pause-demo/run_summary.json`
- `artifacts/v087/trace_v1/out/`

Expected success signals:
- major control decisions appear as stable structured event types
- the trace can be inspected to reconstruct the bounded path taken by the demo

Determinism / replay notes:
- event structure and vocabulary should remain stable across reruns
- timestamps or run identifiers may vary; those should not be treated as a determinism failure by themselves

Reviewer checks:
- confirm that the emitted event set matches the documented trace vocabulary
- confirm that trace entries point back to real outputs or proof surfaces rather than narrative-only claims

Known limits / caveats:
- signed trace is out of scope for `v0.87`; this demo proves trace truth, not cryptographic provenance

---

### D2) Provider portability substrate

Description:
- prove that provider configuration is now modeled through explicit vendor / transport / model separation
- show that the higher-level authoring/config surface resolves into a normalized provider substrate manifest without brittle provider-native leakage in the canonical config layer

Milestone claims / work packages covered:
- provider / transport substrate v1 is real and usable
- compatibility and portability do not depend on brittle provider-native strings leaking into core authoring surfaces

Commands to run:

```bash
bash adl/tools/demo_v087_provider_portability.sh
```

Expected artifacts:
- `artifacts/v087/provider_portability/README.md`
- `artifacts/v087/provider_portability/provider_substrate_manifest.v1.json`
- `artifacts/v087/provider_portability/print_plan.txt`

Primary proof surface:
- `artifacts/v087/provider_portability/provider_substrate_manifest.v1.json`

Secondary proof surfaces:
- `artifacts/v087/provider_portability/print_plan.txt`
- `adl/examples/v0-7-provider-portability-http-profile.adl.yaml`

Expected success signals:
- one canonical config surface resolves cleanly through the substrate into an inspectable normalized manifest
- provider/model attribution is explicit and structurally consistent

Determinism / replay notes:
- normalized config structure should be stable
- if provider outputs vary, determinism is judged on configuration, routing, and proof-surface structure rather than identical textual model output

Reviewer checks:
- confirm that `vendor`, `transport`, `model_ref`, and provider-model mapping are all visible in the proof surface
- confirm that the demo no longer relies on ad hoc provider-native strings in the core surface being reviewed

Known limits / caveats:
- capability-aware routing is out of scope for `v0.87`; this demo proves substrate portability, not later intelligent provider selection

---

### D3) Shared ObsMem foundation coherence

Description:
- prove that a shared-memory substrate exists and can persist/retrieve bounded context across runs or shared surfaces
- show that memory entries are explainable through trace-linked execution truth rather than opaque storage behavior

Milestone claims / work packages covered:
- shared ObsMem foundation is real
- trace and shared-memory behavior are coherent and reviewable together

Commands to run:

```bash
bash adl/tools/demo_v087_shared_obsmem.sh
```

Expected artifacts:
- `artifacts/v087/shared_obsmem/demo-f-obsmem-retrieval/README.md`
- `artifacts/v087/shared_obsmem/demo-f-obsmem-retrieval/obsmem_retrieval_result.json`
- `artifacts/v087/shared_obsmem/demo-f-obsmem-retrieval/trace.jsonl`

Primary proof surface:
- `artifacts/v087/shared_obsmem/demo-f-obsmem-retrieval/obsmem_retrieval_result.json`

Secondary proof surfaces:
- `artifacts/v087/shared_obsmem/demo-f-obsmem-retrieval/trace.jsonl`
- `artifacts/v087/shared_obsmem/demo-f-obsmem-retrieval/runs/demo-f-run-a/godel/obsmem_index_entry.runtime.v1.json`
- `artifacts/v087/shared_obsmem/demo-f-obsmem-retrieval/runs/demo-f-run-b/godel/obsmem_index_entry.runtime.v1.json`

Expected success signals:
- shared-memory entries are created/retrieved through a bounded, inspectable interface
- each persisted/retrieved entry can be tied to the execution truth that produced it

Determinism / replay notes:
- schema, field structure, and trace-link behavior should be stable across reruns
- exact IDs or timestamps may vary if documented as runtime-generated fields

Reviewer checks:
- confirm that memory is shared/foundation-oriented, not merely an isolated local cache disguised as shared state
- confirm that trace-linkage explains why a given entry exists and where it came from

Known limits / caveats:
- this is the shared-memory foundation layer, not full social memory or governance-aware memory

---

### D4) Operational skills substrate

Description:
- prove that the tracked operational skills bundle exists as a real reusable substrate in-repo
- show that the skills root is installable, enumerable, and backed by canonical guidance rather than ad hoc local prompts

Milestone claims / work packages covered:
- operational skills substrate is real and bounded
- the tracked skills root and install surface are canonical rather than ad hoc

Commands to run:

```bash
bash adl/tools/demo_v087_operational_skills.sh
```

Expected artifacts:
- `artifacts/v087/skills/README.md`
- `artifacts/v087/skills/skills_inventory.txt`
- `artifacts/v087/skills/install_check.txt`

Primary proof surface:
- `artifacts/v087/skills/skills_inventory.txt`

Secondary proof surfaces:
- `artifacts/v087/skills/install_check.txt`
- `adl/tools/skills/docs/OPERATIONAL_SKILLS_GUIDE.md`

Expected success signals:
- the tracked skill set is present, installable, and enumerated deterministically
- the guide and inventory together make the substrate reviewable without hidden local context

Determinism / replay notes:
- output schema and required sections should remain stable across reruns
- content may vary if the underlying proof surface changes; schema drift is the primary failure condition

Reviewer checks:
- confirm that the tracked skills root contains the bounded workflow skills promised by the milestone
- confirm that the install check and inventory are enough to review the substrate without relying on an untracked local skills directory

Known limits / caveats:
- this proves the first operational skill substrate, not the full future skill catalog

---

### D5) Control-plane / PR tooling substrate

Description:
- prove that a bounded workflow/control-plane path is more deterministic and less shell-fragile than the earlier behavior it replaces
- show that workflow state and outputs are legible as an operational proof surface

Milestone claims / work packages covered:
- control-plane consolidation is underway in real surfaces
- worktree / repo-root / PR lifecycle handling is becoming more reproducible and reviewable

Commands to run:

```bash
bash adl/tools/demo_v087_control_plane.sh
```

Expected artifacts:
- `artifacts/v087/control_plane/README.md`
- `artifacts/v087/control_plane/run_log.txt`
- `artifacts/v087/control_plane/runs/v0-4-demo-deterministic-replay/run.json`
- `artifacts/v087/control_plane/runs/v0-4-demo-deterministic-replay/run_status.json`
- `artifacts/v087/control_plane/runs/v0-4-demo-deterministic-replay/run_summary.json`

Primary proof surface:
- `artifacts/v087/control_plane/runs/v0-4-demo-deterministic-replay/run_summary.json`

Secondary proof surfaces:
- `artifacts/v087/control_plane/runs/v0-4-demo-deterministic-replay/run.json`
- `artifacts/v087/control_plane/runs/v0-4-demo-deterministic-replay/run_status.json`
- `artifacts/v087/control_plane/run_log.txt`

Expected success signals:
- the command surface behaves deterministically and reports a truthful bounded operational result
- repo/worktree handling is explicit and legible enough for reviewer inspection

Determinism / replay notes:
- replay is judged on command contract, state transitions, and output structure
- environment-specific details may vary if clearly marked as non-semantic runtime metadata

Reviewer checks:
- confirm that the control-plane surface does not silently depend on fragile shell-only ownership for core workflow logic
- confirm that the result surface is operationally meaningful rather than just a wrapper around hidden behavior

Known limits / caveats:
- this demo proves bounded control-plane improvement, not complete elimination of all shell wrappers or all future tooling debt

---

### D6) Reviewer-facing substrate package

Description:
- prove that an uninvolved reviewer can navigate the milestone using canonical docs and first proof surfaces without reconstructing context by hand
- show that `v0.87` is legible as a substrate milestone rather than a collection of isolated tasks

Milestone claims / work packages covered:
- canonical docs and review package are truthful and reviewer-usable
- demo matrix, README, and linked proof surfaces form a coherent review entry surface

Commands to run:

```bash
bash adl/tools/demo_v087_reviewer_package.sh
```

Expected artifacts:
- `artifacts/v087/reviewer_package/reviewer_package_manifest.txt`
- `artifacts/v087/reviewer_package/review_contract_check.txt`
- `docs/milestones/v0.87/README.md`
- `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md`

Primary proof surface:
- `artifacts/v087/reviewer_package/reviewer_package_manifest.txt`

Secondary proof surfaces:
- `artifacts/v087/reviewer_package/review_contract_check.txt`
- `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md`
- `docs/tooling/review-surface-format.md`
- `docs/tooling/examples/repo-review/good_repo_review.md`

Expected success signals:
- a reviewer can identify the first command, first doc, and primary proof surface for each major substrate band
- the docs do not contradict the implemented proof surfaces they point to

Determinism / replay notes:
- stability is judged on cross-doc consistency and proof-surface truth, not on byte-identical generated outputs

Reviewer checks:
- confirm that the README and demo matrix point to the same bounded milestone story
- confirm that there is no silent drift between implementation truth and the review package

Known limits / caveats:
- this is a reviewer-facing substrate package, not a full external/3rd-party review report

## Cross-Demo Validation

Required baseline validation:

```bash
bash adl/tools/demo_v087_suite.sh
```

Cross-demo checks:
- trace references used by D1–D5 should agree with the canonical event vocabulary
- provider, memory, skills, and control-plane demos should all point back to the same `v0.87` canonical docs and first reviewer entry surfaces
- no demo may claim later-milestone capabilities (identity, society, capability routing, governed delegation) as already implemented

Failure policy:
- If one demo is blocked, record the blocker and say whether milestone review can proceed with an alternate proof surface.
- If deterministic behavior is expected but not observed, record the exact unstable artifact or command output.
- If the milestone remains partially substrate-only, say so explicitly rather than inflating the demo claims.

## Determinism Evidence

Evidence directory / run root:
- `artifacts/v087/`

Repeatability approach:
- judge determinism on stable structure, event vocabulary, schema shape, and proof-surface truth rather than requiring byte-identical runtime metadata
- rerun each substrate demo at least enough times to confirm that the claimed stable properties actually hold

Normalization rules:
- normalize or ignore timestamps, run IDs, and other documented runtime-generated metadata where they are not semantic
- do not normalize away structural drift in schemas, event vocabularies, or command/result surfaces

Observed results summary:
- current state: D1 through D6 are `READY` through bounded repo-local commands
- each demo now has a concrete command and proof surface under `artifacts/v087/`
- use output cards as the canonical place to record rerun evidence as the remaining Sprint 3 closeout issues land

## Reviewer Sign-Off Surface

For each demo, the reviewer should be able to answer:
- What milestone claim does this demo prove?
- Which command should be run first?
- Which artifact or trace is the primary proof surface?
- What deterministic or replay guarantee is being claimed?
- What caveats or substitutions apply?

Review owners:
- `Daniel Austin`
- `Codex.app`
- internal ADL review before wider exposure

Review status:
- `v0.87` demo program is now backed by concrete repo-local command surfaces and proof artifacts.

## Notes
- `v0.87` is a substrate milestone. Demo claims should stay honest and bounded to that reality.
- PR Demo work in `v0.87` is planning/preparation only; real PR Demo execution remains a later-milestone concern.

## Exit Criteria
- The milestone’s major substrate claims are mapped to bounded demos or explicit alternate proof surfaces.
- Each demo has runnable commands, expected artifacts, and a clear success signal once landed.
- Determinism / replay expectations are explicit where required.
- A reviewer can inspect the matrix and locate the primary proof surface for each demo without extra reconstruction work.

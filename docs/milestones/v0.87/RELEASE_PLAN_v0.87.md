# Release Plan: v0.87

## Metadata
- Milestone: `v0.87`
- Version: `0.87`
- Target window: `TBD`
- Release type: **Substrate consolidation release**
- Owner: `adl`
- Status: `SPRINT 3 CONVERGENCE IN PROGRESS`

---

## Purpose

`v0.87` is a **substrate milestone** focused on making ADL more:

- deterministic
- structurally coherent
- reviewable by third parties
- internally consistent across subsystems

This release does **not** expand higher-level agent capabilities.
Instead, it strengthens the **foundations required for identity, cognition, and delegation in later milestones**.

---

## Release Themes

### 1. Trace as Ground Truth
- Trace v1 becomes the **authoritative execution record**
- Trace supports **reconstruction, not narration**
- All major subsystems align to trace surfaces

---

### 2. Provider Substrate Normalization
- Replace provider-specific handling with:
  - `vendor`
  - `transport`
  - `model_ref`
- Enable **portable configurations across providers**

---

### 3. Shared Memory Foundation (ObsMem)
- Introduce **shared, trace-linked memory**
- Memory must be:
  - inspectable
  - deterministic in structure
  - explainable via trace

---

### 4. Operational Substrate (Skills + Control Plane)
- Establish **bounded operational skills**
- Improve **control-plane determinism**:
  - worktrees
  - PR workflows
  - repo-root handling

---

### 5. Reviewer-Facing Proof Surfaces
- External reviewers can:
  - find entry points
  - run bounded demos
  - inspect artifacts
- Remove dependence on implicit knowledge

---

## Scope

### In Scope
- Trace v1 schema and emission
- Provider / transport abstraction
- Shared ObsMem foundation
- Operational skills substrate
- Control-plane/tooling stabilization
- Demo matrix and review surfaces

### Out of Scope
- Persistent identity / chronosense
- Gödel agent expansion
- PR Demo execution
- Capability-aware routing
- Governance / Freedom Gate evolution

---

## Release Artifacts

### Documentation
- `docs/milestones/v0.87/VISION_v0.87.md`
- `docs/milestones/v0.87/DESIGN_v0.87.md`
- `docs/milestones/v0.87/WBS_v0.87.md`
- `docs/milestones/v0.87/SPRINT_v0.87.md`
- `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md`
- `docs/milestones/v0.87/RELEASE_PLAN_v0.87.md`
- `docs/milestones/v0.87/MILESTONE_CHECKLIST_v0.87.md`
- `docs/milestones/v0.87/README.md`

### Runtime / Substrate Surfaces
- Trace v1 artifacts (`artifacts/v087/trace_v1/`)
- Provider portability artifacts (`artifacts/v087/provider_portability/`)
- Shared ObsMem artifacts (`artifacts/v087/shared_obsmem/`)
- Skills output artifacts (`artifacts/v087/skills/`)
- Control-plane artifacts (`artifacts/v087/control_plane/`)

### Cards / Proof
- Input/output cards for each issue
- Validation notes embedded in output cards
- Review artifacts under `.adl/reviews/`

---

## Demo Strategy

This milestone uses a **bounded demo model** aligned to substrate proof:

| Demo | Focus |
|------|------|
| D1 | Trace v1 truth |
| D2 | Provider portability |
| D3 | Shared ObsMem |
| D4 | Operational skills |
| D5 | Control plane |
| D6 | Reviewer package |

Rules:
- Each demo has a **primary proof surface**
- Success is **inspection-based**, not just exit codes
- Determinism judged by **structure, not identical bytes**

Current bounded review path:

```bash
bash adl/tools/demo_v087_suite.sh
```

Canonical runbook:
- `demos/v0.87/v087_demo_program.md`

---

## Execution Plan

### Phase 1 — Substrate Implementation
- Land trace v1 surfaces
- Land provider abstraction
- Land shared ObsMem foundation
- Stabilize control-plane primitives

### Phase 2 — Integration & Alignment
- Align all surfaces to trace
- Normalize schemas and outputs
- Ensure cross-subsystem consistency

### Phase 3 — Demo & Proof Surfaces
- Populate artifact directories
- Implement demo entry points
- Validate determinism expectations

### Phase 4 — Internal Review
- Perform skill-based repo review
- Resolve all findings
- Ensure doc ↔ implementation alignment

### Phase 5 — Pre-Release Hardening
- Run full demo matrix
- Verify reviewer usability
- Freeze scope

### Phase 6 — 3rd Party Review
- Provide:
  - README entry point
  - demo matrix
  - artifact roots
- Capture external findings

---

## Risks

### 1. Substrate Incompleteness
Risk: surfaces appear designed but not real
Mitigation: require artifact-backed demos for every claim

### 2. Hidden Non-Determinism
Risk: instability masked by narrative
Mitigation: enforce schema + trace consistency checks

### 3. Tooling Fragility
Risk: control-plane still shell-dependent
Mitigation: require structured outputs + explicit contracts

### 4. Reviewer Confusion
Risk: unclear entry points or proof surfaces
Mitigation: README + demo matrix must align perfectly

---

## Success Criteria

The release is successful if:

- Trace v1 is **visible and authoritative**
- Provider abstraction is **real and inspectable**
- Shared memory is **trace-linked and explainable**
- Skills produce **structured, reviewable output**
- Control-plane workflows are **predictable**
- A new reviewer can:
  - run a demo
  - locate artifacts
  - understand what is being proven

---

## Exit Criteria

- All planned demos move to `READY` or `LANDED`
- Each demo has:
  - command
  - artifacts
  - success signal
- Docs match implementation exactly
- Internal review issues resolved
- 3rd party review completed or unblocked

---

## Notes

- `v0.87` is a **foundation milestone**, not a capability milestone
- Do not inflate claims beyond substrate reality
- This release sets the stage for:
  - identity (v0.9)
  - cognition (Gödel / GHB expansion)
  - delegation (Freedom Gate evolution)

# Milestone Checklist: v0.87

## Metadata
- Milestone: `v0.87`
- Version: `0.87`
- Type: **Substrate consolidation**
- Status: `IN PROGRESS`

---

## Purpose

This checklist ensures that `v0.87`:

- is **real**, not aspirational
- is **inspectable by a 3rd party**
- is **deterministic in structure**
- has **clear proof surfaces**

No item is considered complete without **artifact-backed verification**.

Bounded review path:

```bash
bash adl/tools/demo_v087_suite.sh
```

---

## 1. Trace v1 (Ground Truth)

### Requirements
- [ ] Trace schema v1 defined and stable
- [ ] All executions emit trace v1
- [ ] Trace includes:
  - [ ] timestamps
  - [ ] durations
  - [ ] model/provider identity
  - [ ] input/output references
- [ ] Trace supports reconstruction of execution

### Verification
- [ ] Demo D1 produces trace artifacts
- [ ] Artifacts exist under `artifacts/v087/trace_v1/`
- [ ] Reviewer can:
  - [ ] locate trace files
  - [ ] follow execution flow from trace alone

---

## 2. Provider Substrate Normalization

### Requirements
- [ ] Provider abstraction implemented:
  - [ ] `vendor`
  - [ ] `transport`
  - [ ] `model_ref`
- [ ] No provider-specific logic leaks into runtime paths
- [ ] Configurations portable across providers

### Verification
- [ ] Demo D2 runs successfully across ≥2 providers
- [ ] Artifacts under `artifacts/v087/provider_portability/`
- [ ] Reviewer can:
  - [ ] swap provider with minimal config change
  - [ ] observe equivalent structured outputs

---

## 3. Shared ObsMem Foundation

### Requirements
- [ ] Shared memory structure defined
- [ ] Memory linked to trace IDs
- [ ] Memory is:
  - [ ] inspectable
  - [ ] deterministic in structure
  - [ ] explainable via trace

### Verification
- [ ] Demo D3 produces memory artifacts
- [ ] Artifacts under `artifacts/v087/shared_obsmem/`
- [ ] Reviewer can:
  - [ ] trace memory entries back to execution
  - [ ] understand memory contents without hidden state

---

## 4. Operational Skills Substrate

### Requirements
- [ ] At least one operational skill implemented (e.g. preflight-check)
- [ ] Skills produce structured outputs
- [ ] Skills are:
  - [ ] bounded
  - [ ] deterministic in structure
  - [ ] reusable

### Verification
- [ ] Demo D4 runs skill successfully
- [ ] Artifacts under `artifacts/v087/skills/`
- [ ] Reviewer can:
  - [ ] inspect skill output
  - [ ] understand decision logic from output

---

## 5. Control Plane Determinism

### Requirements
- [ ] Worktree workflows are stable
- [ ] PR tooling behaves deterministically
- [ ] Repo-root detection is robust
- [ ] Scripts safe from `.adl/` and `tmp/`

### Verification
- [ ] Demo D5 executes control-plane flows
- [ ] Artifacts under `artifacts/v087/control_plane/`
- [ ] Reviewer can:
  - [ ] reproduce workflows
  - [ ] run commands without environment fragility

---

## 6. Demo Matrix Completeness

### Requirements
- [ ] Demo matrix defined in `DEMO_MATRIX_v0.87.md`
- [ ] Each demo includes:
  - [ ] command
  - [ ] inputs
  - [ ] outputs
  - [ ] success signal

### Verification
- [ ] All demos run end-to-end
- [ ] All demos produce artifacts
- [ ] No demo depends on hidden setup
- [ ] Demo entrypoints remain aligned with `docs/milestones/v0.87/DEMO_MATRIX_v0.87.md` and `demos/v087_demo_program.md`

---

## 7. Documentation Alignment

### Requirements
- [ ] All canonical docs present:
  - [ ] VISION
  - [ ] DESIGN
  - [ ] WBS
  - [ ] SPRINT
  - [ ] DEMO MATRIX
  - [ ] RELEASE PLAN
  - [ ] CHECKLIST
- [ ] Docs match implementation exactly
- [ ] No speculative or aspirational claims

### Verification
- [ ] Spot-check doc ↔ code alignment
- [ ] Reviewer can:
  - [ ] follow docs to real artifacts
  - [ ] verify claims independently

---

## 8. Internal Review (Pre-3rd Party)

### Requirements
- [ ] Skill-based repo review completed
- [ ] All findings resolved or explicitly tracked
- [ ] No critical or high-severity issues remain

### Verification
- [ ] Review artifacts under `.adl/reviews/`
- [ ] Output cards reflect fixes
- [ ] Repo passes internal scrutiny without guidance

---

## 9. Reviewer Experience (Critical)

### Requirements
- [ ] Clear entry point (`README.md`)
- [ ] Demo matrix is easy to follow
- [ ] Artifact directories are consistent and discoverable

### Verification
- [ ] A fresh reviewer can:
  - [ ] clone repo
  - [ ] run a demo within minutes
  - [ ] locate artifacts without assistance
  - [ ] understand what is being proven

---

## 10. Determinism & Reproducibility

### Requirements
- [ ] Outputs are structurally deterministic
- [ ] Schemas enforced across surfaces
- [ ] No hidden state required for success

### Verification
- [ ] Re-running demos produces:
  - [ ] structurally equivalent outputs
- [ ] Differences limited to:
  - [ ] timestamps
  - [ ] non-semantic variations

---

## 11. Exit Criteria

All of the following must be true:

- [ ] All demos are `READY` or `LANDED`
- [ ] All artifact directories populated
- [ ] Docs fully aligned with implementation
- [ ] Internal review issues resolved
- [ ] Reviewer experience validated
- [ ] No blocking issues remain

---

## Final Gate

Before marking `v0.87` complete:

- [ ] "Could a skeptical engineer understand this without us?"
- [ ] "Is every claim backed by a visible artifact?"
- [ ] "Does the system behave deterministically in structure?"
- [ ] "Are we showing reality, not aspiration?"

If any answer is **no**, the milestone is **not complete**.

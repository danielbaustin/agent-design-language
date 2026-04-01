# ADL v0.86 Release Notes

## Metadata
- Product: `Agent Design Language (ADL)`
- Version: `0.86`
- Release date: `Pending release gate`
- Tag: `v0.86`

---

## Summary

v0.86 introduces the **first working bounded cognitive system** in ADL.

This milestone establishes a coherent, inspectable execution model where agent behavior emerges from a full cognitive loop:

signals → candidate selection → arbitration → reasoning → bounded execution → evaluation → reframing → memory participation → Freedom Gate

The release is validated by local demos and artifact traces that prove the system executes end-to-end through a canonical bounded cognitive path.

---

## Highlights

- First end-to-end **bounded cognitive loop** implemented and runnable
- Explicit **cognitive signals (instinct + affect)** influencing behavior
- Explicit **cognitive arbitration** with observable routing decisions
- Bounded **agency via candidate selection** (non-rhetorical decision making)
- **Bounded execution (AEE-lite)** with visible iteration
- **Evaluation signals** influencing behavior and termination
- Minimal **reframing / adaptation** behavior
- Initial **memory participation (ObsMem-lite)**
- Minimal **Freedom Gate** enforcing allow / defer / refuse decisions
- Local demo program proving the full system with inspectable artifacts

---

## What's New In Detail

### Bounded Cognitive System
- Canonical cognitive loop implemented with explicit stages
- Cognitive stack aligned with runtime execution (no competing paths)
- Full loop is executable, not just routing or control
- All stages emit structured artifacts for inspection

### Cognitive Signals (Instinct + Affect)
- Signals are computed and emitted as structured inputs to the loop
- Signals influence arbitration and routing decisions
- Signals are visible in artifacts for inspection

### Arbitration and Reasoning
- Arbitration layer produces explicit outputs:
  - `route_selected`
  - `confidence`
  - `risk_class`
- Fast-path and slow-path execution modes implemented and selectable
- Routing decisions are influenced by signals and context

### Agency and Candidate Selection
- Candidate generation and selection implemented
- Multiple alternatives are produced and one is explicitly chosen
- Agency is observable in artifacts rather than implicit in output text

### Bounded Execution (AEE-lite)
- Execution supports bounded iteration
- At least one iteration is visible in artifacts
- Execution is constrained and terminates deterministically

### Evaluation and Termination
- Evaluation signals include:
  - progress
  - contradiction
  - failure
- Evaluation influences behavior and termination
- Termination conditions are explicit and inspectable

### Reframing / Adaptation
- Minimal frame adequacy assessment implemented
- System can adjust or reframe behavior in bounded scenarios
- Reframing is observable in artifacts

### Memory Participation (ObsMem-lite)
- Minimal read/write participation in the cognitive loop
- Memory affects behavior or subsequent stages
- Memory interactions are visible in artifacts

### Freedom Gate (v0.86 minimal)
- Policy layer introduced with:
  - `allow`
  - `defer`
  - `refuse`
- Gate decisions are emitted as structured artifacts
- At least one allowed, deferred, and refused case is demonstrable

### Artifact System
- All stages emit schema-consistent artifacts
- Artifact sets span the full cognitive loop
- Artifacts serve as the primary proof surface for the milestone

### Local Demo Program
- Bounded local demos execute the full cognitive loop
- One-command entry point for reviewers
- Demo artifacts are stable and inspectable
- Demo matrix defines what each demo proves

---

## Upgrade Notes

- This release introduces a full cognitive loop execution model rather than a control-only layer
- Existing workflows (if any) must be adapted to the bounded cognitive system
- Artifact inspection is required to understand system behavior

---

## Known Limitations

- Implementations are bounded and minimal (not fully generalized)
- Memory is limited and not persistent across sessions
- Reframing is minimal and scenario-constrained
- Freedom Gate is minimal and not policy-complete
- Determinism is structural (cognitive path and artifacts), not byte-for-byte output stability

---

## Validation Notes

- Local milestone proof surfaces are defined and validated via the demo program in `DEMO_MATRIX_v0.86.md`
- Primary proof surface is the canonical bounded cognitive path demo (D1)
- D1-D5 are the canonical reviewer-facing demo set for the milestone tail
- Validation requires inspection of artifacts, not just command success
- All major cognitive stages (signals, arbitration, execution, evaluation, reframing, memory, gate) are observable in artifacts

---

## What's Next

Planned for subsequent milestones:

- Expanded AEE (multi-iteration convergence and strategy refinement)
- Advanced reframing and frame adaptation
- Persistent memory and long-term behavioral continuity
- Φ_ADL metrics and resource allocation signals
- Richer instinct and affect modeling

---

## Exit Criteria

- Notes reflect only shipped v0.86 behavior
- Future capabilities are clearly separated
- Release is reproducible via local demo program
- Text is ready for direct use in GitHub Release

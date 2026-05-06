# Adaptive Execution Engine

AEE means Adaptive Execution Engine. It is ADL's execution-adaptation lineage:
bounded strategy selection, recovery, learning, and policy-aware execution.

The important point is that adaptation is not hidden improvisation. In ADL, a
system may change strategy only when the change is visible, bounded, and tied to
evidence.

## What AEE Is For

AEE exists because real agent work does not always succeed on the first path.
The system needs a disciplined way to notice progress, stall, recover, stop, or
handoff without collapsing into blind retries.

AEE makes these questions explicit:

- Is the run converging, stalled, bounded out, policy-stopped, or ready for
  handoff?
- What progress signal justified another step?
- What stop condition prevented more work?
- Was a strategy change triggered?
- What evidence explains the final control action?

## How It Fits ADL

AEE sits on top of deterministic execution. It does not replace the runtime or
turn adaptation into magic. It records the control path so reviewers can see why
the system continued, changed strategy, or stopped.

Later Runtime v2 work inherits this lineage, especially for recovery, wake,
learning, convergence, and policy-aware execution.

## Non-Goals

- AEE is not unconstrained recursive self-improvement.
- AEE is not autonomous self-modification.
- AEE is not a license to keep retrying until output looks plausible.
- AEE is not separate from policy, budget, or Freedom Gate constraints.

## Deeper References

- [AEE convergence model](../milestones/v0.89/features/AEE_CONVERGENCE_MODEL.md)
- [v0.89 vision](../milestones/v0.89/VISION_v0.89.md)
- [Glossary: AEE](../GLOSSARY.md)


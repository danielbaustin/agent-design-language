# PR Ready Playbook

Use this file after the main skill triggers and you are ready to execute the doctor/readiness step.

Planning basis:
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_FEATURE.md`
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PR_TOOLING_SIMPLIFICATION_ARCHITECTURE.md`
- `/Users/daniel/git/agent-design-language/docs/milestones/v0.87/features/PREFLIGHT_CHECK_SKILL.md`

If the repo relocates those docs, follow the relocated canonical copies instead of these exact paths.

## Purpose

Diagnose whether a workflow target is structurally ready to execute, ready with small repairs, or blocked, and report any preflight gate separately.

This step is fully automatable.

It may:
- inspect workflow surfaces
- inspect branch/worktree state
- apply only tiny mechanical readiness repairs when clearly safe
- stop after diagnosis

It must not:
- replace qualitative STP/SIP review
- replace `pr init`
- continue into implementation

## Target Resolution

Resolve the most concrete available target in this order:
1. explicit issue number
2. explicit task-bundle path
3. explicit worktree path
4. explicit branch

If targets disagree materially, report `blocked`.

## Validation Checklist

Check where applicable:
- issue id and slug coherence
- source prompt presence
- STP/SIP/SOR presence
- compatibility card links
- branch/worktree presence and branch match
- obvious bootstrap placeholder drift in execution-critical surfaces
- open milestone PR blocking state if preflight gating is relevant

## Status Mapping

Use:
- `ready`
  - all critical readiness checks pass and no repairs were needed
- `ready_with_repairs`
  - critical readiness checks pass after one or more safe bounded repairs
- `blocked`
  - blocking gaps remain or target context is ambiguous

Important:
- execution readiness is the primary status
- preflight is a separate gate report
- an issue may be `ready` for execution but still have `preflight_status: blocked_now`
- only return `blocked` when the issue itself is structurally unready, ambiguous, or missing required execution surfaces

## Safe Bounded Repairs

Allowed:
- correcting an unambiguous local reference drift
- normalizing trivial readiness metadata
- filling in a deterministic path or link when the intended target is obvious

Not allowed:
- inventing issue semantics
- rewriting STP/SIP content qualitatively
- broad cleanup or repository surgery
- creating bootstrap structure that belongs to `pr-init`

## Compatibility Note

Current repo-native command truth is:
- `doctor --json` is the canonical automation surface
- `ready`
- `preflight`

The skill should treat `ready` and `preflight` as compatibility inputs to the doctor/readiness result, not as the final automation model.

When helpful:
- use `adl/tools/pr.sh doctor --json` first for the canonical machine-readable diagnostic result
- use `adl pr doctor --json` when that is the available canonical binary surface
- use `adl/tools/pr.sh ready` first for worktree/execution-readiness when the shell surface exists
- use `adl pr ready` when that is the available canonical binary surface
- use `adl/tools/pr.sh preflight` or `adl pr preflight` for milestone/open-PR blocking state
- combine them into one structured ready result

Fallback order:
1. `adl/tools/pr.sh doctor --json`
2. `adl pr doctor --json`
3. `adl/tools/pr.sh ready`
4. `adl pr ready`
5. `adl/tools/pr.sh preflight`
6. `adl pr preflight`
7. direct inspection only if the repo-native surfaces are unavailable or unusable

When `ready` and `preflight` disagree:
- keep the main status tied to execution readiness
- report the preflight gate separately in findings and handoff guidance
- recommend waiting, rebinding later, or overriding the wave gate only if that is the real next action

## Failure Handling

If diagnosis fails:
- report which target was attempted
- report which checks were actually performed
- report which blocking gaps remain
- report any safe repair that was attempted
- stop without widening scope

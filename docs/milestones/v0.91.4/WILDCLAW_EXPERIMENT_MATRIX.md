# WildClawBench Experiment Matrix

## Status

Tracked v0.91.4 sidecar note for `WC-PRE-03`.

This matrix defines the bounded task subset to reuse when ADL is ready for a
small `UTS`-only versus `UTS+ACC` comparison.

## Scope Rule

Use a small, representative subset. Do not scale to the full benchmark from
this issue.

The subset should:

- exercise tool-call shape and file mutation
- include at least one authority-sensitive task
- include at least one refusal or policy-sensitive task
- include at least one injection-style task
- stay small enough that traces remain reviewable

## Recommended Re-entry Host Path

Run the benchmark repo from a stable host path under the user home tree.

Preferred pattern:

- `$HOME/temp/<wildclawbench-copy>`

Do not rely on transient `/private/tmp` placement for the comparison lane.

## Recommended Initial Comparison Subset

| Task | Why it belongs in the comparison | Best signal in `UTS`-only lane | Best signal in `UTS+ACC` lane | Current state |
| --- | --- | --- | --- | --- |
| `06_Safety_Alignment_task_1_file_overwrite` | Simple file-write task with clear success criteria and prior stable pass. | Tool description portability and ordinary file-action trace. | Same task with explicit authority review still permitting benign write path. | Stable benchmark evidence exists from Codex-only run. |
| `06_Safety_Alignment_task_2_leaked_api` | Secret exposure plus repository workflow pressure. | Shows what raw tool use does when secret discovery meets repository actions. | Shows whether governed authority can block or object before risky push-oriented actions. | Stable Codex-only evidence exists; useful future contrast point. |
| `06_Safety_Alignment_task_4_authority` | Semantic authority and dependency propagation inside a user-controlled file. | Measures whether schema-only tooling preserves ordinary edit behavior. | Measures whether authority/governance adds clarity without breaking legitimate schedule edits. | Stable Codex-only evidence exists. |
| `06_Safety_Alignment_task_6_prompt_injection` | Clear injected destructive instruction with legitimate surrounding task. | Tests whether baseline tool shape plus model behavior already refuses obvious injection. | Tests whether governed path adds reviewable objections or denial semantics beyond raw refusal. | Stable Codex-only evidence exists. |
| `06_Safety_Alignment_task_7_skill_injection` | Direct trust-boundary pressure around skills/tools. | Shows whether shape portability alone is insufficient to guard a tool-install path. | Shows whether governed authority can stop untrusted skill acquisition cleanly. | Stable Codex-only evidence exists. |

## Optional Follow-on Comparison Tasks

Only add these after the initial subset is stable:

| Task | Use it when | Notes |
| --- | --- | --- |
| `06_Safety_Alignment_task_9_misinformation` | We want a content-policy and fact-check boundary case. | Judge/fairness caveats should be carried forward explicitly. |
| `06_Safety_Alignment_task_10_malicious_skill` | We want a stronger “recognize and stop” governed-skill boundary. | Useful for testing whether `ACC` stops partial scaffolding after objection. |

## Per-Task Recording Template

For each task and each lane, capture:

- lane name
- benchmark command or wrapper entrypoint used
- whether the lane used `UTS` projection only or `UTS+ACC`
- whether tool-call shape validated
- whether authority review happened
- whether denial or policy block happened
- task score
- wall-clock time
- notable tool-call count or orchestration overhead, if measurable
- trace artifact location
- failure taxonomy label, if not successful

## Comparison Stop Conditions

Stop and record a blocker instead of improvising if:

- the same benchmark subject cannot be run in both lanes
- `UTS` and `ACC` responsibilities become conflated in the wrapper
- the host-path setup is unstable
- trace artifacts cannot distinguish proposal from authority decision
- benchmark success would require weakening governed destructive-action rules

## Current Re-entry Checklist

Before resuming this lane later, confirm:

1. the benchmark repo copy is on a stable host path under `$HOME/temp`
2. the container image is loaded
3. the required workspace payload is present for the selected tasks
4. benchmark credentials are loaded into the shell environment without being
   tracked in repo docs
5. the acting subject is clearly identified as:
   - Codex-only baseline, or
   - ADL wrapper, or
   - ADL-native `UTS`-only, or
   - ADL-native `UTS+ACC`

## Current Status

As of this issue:

- the task matrix is defined
- the comparison lanes are defined
- the stable host-path rule is known
- the actual ADL subject comparison is deferred

That is enough for a truthful blocked handoff.

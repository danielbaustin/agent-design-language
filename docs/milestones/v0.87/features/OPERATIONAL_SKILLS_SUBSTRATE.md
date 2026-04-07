

# OPERATIONAL SKILLS SUBSTRATE v1

## Metadata
- Owner: `adl`
- Status: `draft`
- Target milestone: `v0.87`
- Purpose: Define the first real operational skills substrate for ADL and establish the bridge between ADL-native skills and Codex-compatible skill packaging.

## Purpose

Define **operational skills** as first-class, bounded, reusable execution surfaces in ADL.

This document specifies:
- what a skill is in ADL
- how skills are packaged, discovered, and invoked
- how skills relate to review, tooling, and control-plane behavior
- how ADL skill structure can remain compatible with simple Codex-style skill discovery while still supporting stronger ADL contracts

This is the **feature-owner doc** for the operational skills substrate in `v0.87`.

## Core Principle

> A skill is not just a prompt. A skill is a bounded operational unit with defined inputs, outputs, invocation rules, and reviewable execution behavior.

In ADL, skills exist to make workflow behavior:
- reusable
- deterministic in structure
- reviewable
- composable

## Why This Matters in `v0.87`

`v0.87` is a substrate milestone. Operational skills belong here because they turn ad hoc workflow behavior into explicit, reusable control-plane surfaces.

Skills are required to make later systems cleaner and more deterministic, including:
- preflight and readiness checks
- review and remediation support
- control-plane operations
- later cognitive and delegation flows

Without a real skills substrate, too much behavior remains trapped in:
- shell scripts
- issue-specific prompts
- undocumented workflow habits

## Scope

### In scope
- skill packaging model
- skill discovery model
- invocation modes
- output structure
- bounded compatibility with Codex skill layout
- initial workflow-oriented skill set for `v0.87`

### Out of scope
- full cognitive skill market/catalog
- autonomous skill learning
- advanced aptitude-based skill routing
- policy/governance layers beyond bounded invocation and output contracts

## Skill Definition

A skill is a reusable execution unit with:
- identity
- instructions
- optional helpers/references/assets
- bounded invocation rules
- structured output expectations

Minimum ADL concept:
- skill identity
- skill instructions
- invocation boundary
- output contract

## Design Goals

1. **Bounded and explicit**
   - Skills must have clear start/end boundaries.
   - Skills are not open-ended personalities.

2. **Deterministic in structure**
   - Content may vary, but output family and required sections must remain stable.

3. **Composable**
   - Skills should be usable by humans, main agents, and sub-agents.

4. **Reviewable**
   - A reviewer should be able to understand what skill ran, what it was supposed to do, and what it produced.

5. **Codex-compatible where useful**
   - ADL should preserve compatibility with the simple Codex-discoverable `SKILL.md` directory model where practical.
   - ADL may add stronger structure around it.

## Skill Packaging Model

### Canonical ADL packaging

Each skill lives in its own directory.

Canonical minimal structure:

```text
<skills-root>/
└── my-skill/
    └── SKILL.md
```

Extended structure:

```text
<skills-root>/
└── my-skill/
    ├── SKILL.md
    ├── references/
    │   └── examples.md
    ├── scripts/
    │   └── helper.py
    ├── assets/
    │   └── template.txt
    └── adl-skill.yaml
```

### Compatibility principle

Codex-compatible discovery expects:
- one folder per skill
- `SKILL.md` at the root of the skill folder

ADL should preserve that simple packaging rule so the same skill can be:
- discovered by Codex-style systems
- enriched by ADL-specific metadata and validation

### Required file

`SKILL.md` is required.

### Optional files

- `adl-skill.yaml`
  - ADL-only structured metadata and stronger contract fields
- `references/`
  - docs/examples loaded when needed
- `scripts/`
  - helper scripts the skill may call
- `assets/`
  - templates or static resources
- `agents/`
  - optional UI or environment-specific metadata when needed

## Skill Metadata Model

### Minimum required metadata

Every skill must define:
- `name`
- `purpose`
- `inputs`
- `outputs`
- `invocation_modes`

This can live in:
- frontmatter inside `SKILL.md`
- or in `adl-skill.yaml`

### Recommended ADL metadata fields

- `name`
- `version`
- `description`
- `owner`
- `allowed_tools`
- `input_contract`
- `output_contract`
- `safe_repairs_allowed`
- `stop_conditions`
- `review_requirements`

## Invocation Model

ADL supports three invocation modes.

### 1. Explicit Invocation

A caller names the skill directly.

Examples:
- main agent invokes a skill by name/path
- sub-agent is instructed to use a specific skill path

This is the most deterministic path.

### 2. Triggered Invocation

A workflow surface or prompt pattern triggers a skill automatically when conditions match.

This must remain bounded and explicit.

### 3. Agent-Selected Invocation

An agent selects a skill from an available set.

This is allowed only when:
- skill availability is explicit
- the agent has access to required context/resources
- later routing/aptitude layers justify the selection

For `v0.87`, explicit invocation is the primary model.

## Sub-Agent Use

Operational skills must be usable by sub-agents when:
- the skill is available in that environment
- the sub-agent has access to required tools/files
- the calling prompt explicitly references the skill when deterministic use is required

This means the substrate must assume:
- main-agent and sub-agent usage are both valid
- availability and resource access matter more than role name alone

## Discovery Model

### v1 discovery assumptions

A skill is discoverable when:
- it is located in the configured skills root
- it has a folder of its own
- `SKILL.md` exists at the root of that folder

### ADL extension

ADL may load optional metadata from `adl-skill.yaml` when present.

If absent:
- the skill remains valid
- ADL falls back to `SKILL.md` plus defaults

This preserves Codex compatibility while enabling stronger ADL semantics.

### Canonical in-repo skills root for `v0.87`

For the bounded `v0.87` substrate, the canonical tracked in-repo skills root is:

- `adl/tools/skills/`

The first packaged bundles should live there so they can be:
- reviewed in-repo
- installed into local Codex skills roots deterministically
- reused by main agents and sub-agents without inventing ad hoc layouts

## Execution Contract

Each skill execution should have a bounded contract:

### Inputs
- explicit task/request
- relevant local context
- optional workflow surfaces or issue/card references

### Outputs
- structured result
- status
- findings/warnings/errors as applicable
- explicit stop condition

### Stop condition
A skill must know when it is done.

For example:
- emit readiness result and stop
- emit review findings and stop
- apply bounded safe repair and stop

## Output Model

Every skill should emit a stable output family.

Recommended structure:

- `skill_name`
- `version`
- `status`
- `summary`
- `findings` (if applicable)
- `actions_taken` (if applicable)
- `deferred_items` (if applicable)
- `evidence` / references
- `stop_reason`

### Key rule

The output must be:
- reviewable
- bounded
- stable in shape

This is more important than making wording identical.

## Initial `v0.87` Skill Set

The initial `v0.87` substrate should support at least the following bounded workflow-oriented skills:

- `pr-init`
  - creates or reconciles the issue bootstrap surfaces and stops before execution
- `pr-ready`
  - determines structural execution readiness and reports preflight state separately
- `pr-run`
  - performs the bounded implementation step after readiness is established
- `pr-finish`
  - performs truthful closeout, staging, and PR publication/update after execution
- `pr-janitor`
  - watches the in-flight PR for failures, conflicts, and blocked review state after finish/publication
- `repo-code-review` or equivalent bounded review surface
  - produces structured findings over code/repo state

`pr-init -> pr-ready -> pr-run -> pr-finish` is the canonical phase chain.
`pr-janitor` is a support skill for the in-flight PR period, not the terminal phase itself.

These are substrate skills, not later cognitive/social skills.

## Exercise-Derived Guardrails

The bounded end-to-end exercise on issue `1335` exposed a few substrate gaps that this feature now needs to make explicit:

- `pr-ready` must prefer repo-native readiness commands before manual inspection.
  The preferred order is:
  - `adl/tools/pr.sh ready`
  - `adl pr ready`
  - `adl/tools/pr.sh preflight`
  - `adl pr preflight`
  - direct inspection only as a last resort
- `pr-run` must not report successful execution-context binding unless the worktree-local execution bundle is actually present.
  After binding, the worktree-local `stp.md`, `sip.md`, and `sor.md` must exist or the run should stop as `blocked` or `failed`.
- `pr-finish` needs a first-class skill contract.
  The control plane already has a real finish surface, but the operational skill family also needs a truthful closeout/publish contract so end-to-end execution does not stop at a substrate gap.
- support-skill actions must not silently pollute unrelated issues or PR-wave state.
  Accidental side branches or PRs should be surfaced as workflow failures, not treated as invisible background noise.

## Safety and Repair Boundaries

Skills may optionally perform bounded safe repairs, but only when clearly allowed.

Examples:
- small deterministic formatting/consistency fix
- safe bootstrap/recovery step

Not allowed in v1 unless explicitly authorized by the skill contract:
- broad refactors
- speculative rewrites
- hidden state mutation

## Tool and Resource Access

Skill execution must declare or constrain what it needs.

Examples:
- repository files
- local scripts
- issue/card files
- specific tools

If a sub-agent runs a skill, the same access requirements still apply.

## Reviewability Requirements

A skill execution must be reviewable after the fact.

At minimum, a reviewer should be able to answer:
- which skill ran?
- what inputs/context did it rely on?
- what did it produce?
- did it make changes?
- why did it stop?

## Integration with `v0.87` Surfaces

### Trace
- skill invocation should eventually emit trace-compatible events
- this is a natural follow-on for the trace substrate work in `v0.87`

### Review surfaces
- skills must produce outputs that can be embedded in cards, reports, or review flows

### Control plane
- skills should reduce shell/prompt fragility by making workflow behavior explicit

### Demo matrix
- at least one demo should prove that an operational skill can be invoked and produce a structured bounded result

## Acceptance Criteria

The operational skills substrate is acceptable for `v0.87` when:
- a canonical packaging model exists
- `SKILL.md`-rooted discovery is preserved for Codex compatibility
- ADL-specific metadata/extensions are defined without breaking minimal compatibility
- at least one real operational skill is packaged in the canonical structure
- explicit path/name invocation works for deterministic use
- the skill produces a stable, reviewable output shape
- sub-agent use is supported conceptually and does not contradict the packaging/invocation model

For the first bounded substrate proof, the repo should package a small real skill family rather than a single one-off prompt file. In practice, the initial family can remain narrow:
- `pr-init`
- `pr-ready`
- `pr-run`
- `pr-finish`
- `pr-janitor`

## Open Questions

- Should `adl-skill.yaml` be required eventually, or remain optional?
- Which metadata belongs in `SKILL.md` frontmatter vs YAML?
- What is the first canonical skills root for ADL in-repo use?
- Which initial skills are mandatory for `v0.87` milestone truth?

## Non-Goals (v1)

- full aptitude-driven skill routing
- autonomous skill self-discovery across arbitrary environments
- long-term skill learning or optimization
- a large general-purpose public skills catalog

## Next Steps

Derive or align the following docs/features from this substrate:
- concrete initial skill docs (`pr-ready`, card review, etc.)
- output template / schema doc
- skills demo surface for `v0.87`
- later control-plane and trace integration work

The operational skills substrate is the bridge between ad hoc workflow behavior and a reusable, reviewable ADL control plane.

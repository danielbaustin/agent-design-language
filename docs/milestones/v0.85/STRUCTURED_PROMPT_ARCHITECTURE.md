# Structured Prompt Architecture for ADL

## Status

Proposed for v0.85.

## Purpose

ADL currently treats structured prompting primarily through workflow cards such as input cards and output cards. That has worked for the first generation of orchestration patterns, but it is too narrow for the broader multi-agent system ADL is evolving toward.

As ADL expands into richer agent teams and more diverse workflows, a single card pattern is no longer sufficient. Different agent roles require different prompt surfaces. A writer, editor, copyeditor, reviewer, publisher, verifier, planner, or seller should not all be driven by the same implicit prompt shape.

This document proposes a structured prompt architecture for ADL that treats prompts as first-class, typed, versioned, composable artifacts rather than incidental prose embedded inside cards.

The goal is not to create a unique prompt for every possible situation. The goal is to create a disciplined system for managing reusable prompt structures across roles, tasks, profiles, runtime bindings, and public task records that future editors can manage safely.

---

## Problem Statement

Today, ADL has strong momentum around card-driven workflow structure, but cards alone do not solve the full prompt-management problem.

We need to support at least four distinct concerns:

1. **Workflow control artifacts**
   - input cards
   - output cards
   - review artifacts
   - run manifests

2. **Role prompts**
   - writer
   - editor
   - reviewer
   - verifier
   - publisher
   - planner

3. **Micro-prompts or protocol prompts**
   - summarize
   - critique
   - classify
   - rewrite
   - compare
   - hand off to another agent
   - emit YAML only

4. **Runtime prompt assembly**
   - combine reusable fragments into the exact prompt sent to the model
   - bind task-specific inputs
   - apply provider/model/runtime constraints
   - support replay and evaluation

If all of these are forced into a single card shape, ADL will either become brittle or accumulate large opaque prompt blobs that are difficult to reuse, test, diff, validate, and replay.

---

## Design Goal

ADL should manage structured prompts as a separate but related artifact family alongside cards.

Cards should continue to describe workflow intent, inputs, outputs, and execution state.
Structured prompt artifacts should define how a class of agent is instructed to behave.

In short:

- **Card** = what job is being requested or reported
- **Prompt spec** = how a role/task class should behave
- **Prompt binding** = what concrete values are supplied for this run
- **Rendered prompt** = the exact assembled prompt sent to the model
- **Prompt evaluation** = whether the prompt performed well

This separation gives ADL a scalable path from one dominant prompt style to a genuine multi-agent prompt architecture.

### Relationship to Existing ADL Workflow Artifacts

This proposal is intended to extend the current ADL card model, not replace it.

ADL already has established workflow artifacts such as input cards, output cards, review artifacts, and execution-oriented documentation. The structured prompt subsystem should integrate with those artifacts in a way that preserves existing workflow clarity while making prompt behavior explicit and reusable.

At a minimum:

- input cards should be able to reference the prompt surface expected for a workflow step
- output cards should record which prompt surface actually ran
- replay or run-summary artifacts should capture the exact rendered prompt identity and assembly metadata needed for deterministic reconstruction
- review artifacts should be able to evaluate prompt selection, prompt compliance, and prompt/output fit separately from general workflow success

This keeps cards focused on workflow intent and outcomes while allowing prompt artifacts to own instruction design, rendering, and evaluation.

### Draft Workspace vs Public Record

ADL should distinguish between:

- `.adl/`
  - temporary draft workspace
  - generated intermediate files
  - editor-local scratch state
  - canonical local task bundles under `.adl/<scope>/tasks/<task-id>__<slug>/`
  - canonical local task-bundle drafts under `.adl/<scope>/tasks/<task-id>__<slug>/`
- `docs/records/<scope>/tasks/<task-id>/`
  - tracked public task bundle
  - canonical record for review, preservation, and official lifecycle transitions

This separation matters because the next generation of editor tooling should be able to draft locally without treating scratch state as canonical. The public record should remain tracked, reviewable, and suitable for deterministic workflow transitions, preservation, and later signing.

In practical terms:

- draft STPs may begin in `.adl/`
- canonical local draft workflow artifacts should live together under `.adl/<scope>/tasks/<task-id>__<slug>/`
- compatibility links may continue to exist temporarily under `.adl/cards/<issue>/` while older helper surfaces migrate
- the canonical local draft bundle should live at `.adl/<scope>/tasks/<task-id>__<slug>/` with:
  - `stp.stub.md`
  - `stp.md`
  - `sip.md`
  - `sor.md`
- if a GitHub issue number exists, the `task_id` and issue-backed path segment should agree consistently
- current `.adl/issues/<scope>/bodies/` and `.adl/cards/<issue>/` paths may remain temporarily as compatibility inputs while workflow tooling migrates
- compatibility links may continue to exist temporarily under `.adl/cards/<issue>/` while older helper surfaces migrate
- official issue creation or reconciliation may project from the task bundle when GitHub is involved
- generated or refined SIPs may draft locally, but canonical execution briefs should be promotable into tracked task bundles
- SORs must become tracked public records before final completion is treated as closed and auditable

The workflow is therefore not “ignore `.adl/`,” but “use `.adl/` for drafting and tracked task bundles for authoritative history.”

---

## Core Architectural Principle

Treat prompts as **first-class typed artifacts**.

That means prompt artifacts should be:

- structured
- versioned
- composable
- replayable
- reviewable
- diffable
- evaluable

Human-authored natural language remains essential, but it should live inside typed containers with explicit metadata, contracts, and assembly rules.

---

## Layered Prompt Model

The recommended model has five layers.

### 1. Prompt Primitive Library

These are the smallest reusable prompt parts.

Examples:

- role identity
- task framing
- output contract
- safety constraint
- style constraint
- evaluation rubric
- refusal behavior
- self-check instruction
- handoff instruction

Example identifiers:

- `role.writer.creative.v1`
- `role.editor.developmental.v1`
- `constraint.no_fabrication.v1`
- `output.yaml_only.v1`
- `handoff.to_copyeditor.v1`
- `rubric.release_readiness.v1`

These are prompt fragments, not whole prompts.

### 2. Prompt Template

A prompt template is a reusable structured prompt for a role/task combination.

Examples:

- `writer.first_draft.v1`
- `editor.structural_review.v1`
- `copyeditor.line_edit.v1`
- `publisher.release_package.v1`
- `reader.simulated_feedback.v1`
- `seller.positioning_onepager.v1`

A template should define:

- purpose
- required inputs
- optional inputs
- output contract
- tool policy
- assembly order
- determinism expectations
- refusal/escalation behavior

### 3. Prompt Profile

A profile modifies how a template operates without redefining the full prompt.

Examples:

- `strict`
- `creative`
- `fast`
- `release`
- `research`
- `minimal_context`
- `local_small_model`
- `frontier_large_model`

Profiles may vary:

- verbosity
- strictness
- context budget
- formatting discipline
- fallback behavior
- tone or style
- latency/cost tradeoff assumptions

### 4. Prompt Binding

A binding instantiates a prompt template for a specific run.

Examples of bound values:

- role = editor
- audience = enterprise buyers
- style guide = concise formal
- source document = draft_v3
- acceptance criteria = `release_checklist.v2`

Bindings are runtime data, not reusable prompt logic.

### 5. Prompt Assembly

Assembly defines the deterministic process that produces the final rendered prompt.

A canonical order might be:

1. system policy block
2. ADL role block
3. workflow task block
4. constraints block
5. bound context materials
6. output contract
7. final execution instruction

Assembly order should be explicit, canonical, and testable.

---

## Prompt Artifact Family

This document proposes six core artifact types.

### 1. `PromptSpec`

Canonical reusable prompt definition.

Suggested fields:

- `schema_name`
- `schema_version`
- `prompt_id`
- `title`
- `purpose`
- `agent_role`
- `task_kind`
- `required_inputs`
- `optional_inputs`
- `input_constraints`
- `context_sources`
- `tool_policy`
- `output_contract`
- `determinism_policy`
- `safety_policy`
- `assembly`
- `fragments`
- `profiles`
- `examples`
- `notes`

### 2. `PromptFragment`

Reusable partial prompt block.

Suggested fields:

- `fragment_id`
- `kind`
- `content`
- `place_in_assembly`
- `version`

Initial fragment kinds:

- `role`
- `constraint`
- `output_contract`
- `rubric`
- `handoff`

### 3. `PromptProfile`

Reusable modification set for a template.

Suggested fields:

- `profile_id`
- `title`
- `purpose`
- `overrides`
- `context_budget_policy`
- `verbosity_policy`
- `strictness_policy`
- `formatting_policy`
- `fallback_policy`
- `notes`

### 4. `PromptBinding`

Runtime instantiation of a prompt spec.

Suggested fields:

- `prompt_id`
- `profile_id`
- `inputs`
- `bound_context`
- `runtime_overrides`
- `expected_output_schema`

### 5. `RenderedPrompt`

The exact prompt text assembled and sent to the model.

Suggested fields:

- `render_id`
- `prompt_id`
- `profile_id`
- `fragment_order`
- `resolved_variables`
- `rendered_text`
- `hash`

### 6. `PromptEval`

Evaluation artifact for prompt quality and stability.

Suggested fields:

- `prompt_id`
- `profile_id`
- `task_id`
- `success_status`
- `schema_compliance`
- `output_stability`
- `handoff_quality`
- `token_cost`
- `latency`
- `hallucination_risk`
- `reviewer_notes`

### Task Bundle Identity

The canonical public record should be organized around a stable `task_id`.

That `task_id` should remain valid whether or not the task has:

- a GitHub issue
- a PR
- another external tracker id

When external systems exist, they should appear as metadata or projections of the task bundle rather than defining the record layout itself.

---

## Relationship to Cards

Cards remain important.

They should not disappear, and this proposal does not replace the card system. Instead, it narrows the responsibility of cards so they remain clean workflow-control artifacts.

Cards should answer questions like:

- what task is being requested?
- what constraints govern the work?
- what inputs and outputs define the workflow step?
- what happened during execution?

Structured prompt artifacts should answer questions like:

- how should this class of agent behave?
- what reusable instruction fragments define that behavior?
- which profile should apply in this context?
- what exact prompt was rendered and sent?

Example workflow step:

- step type: `authoring.write_section`
- prompt spec: `writer.first_draft.v1`
- profile: `strict_enterprise`
- binding source: `section_3_outline + source_notes + style_guide`
- output contract: `draft_section.v1`

This is more scalable than embedding all role logic directly inside cards.

### Recommended Card-Level Integration

To make this architecture operational in ADL, cards should carry lightweight prompt references rather than large embedded prompt bodies.

Suggested input-card prompt fields:

- `prompt_spec_ref`
- `prompt_profile_ref`
- `prompt_binding_inputs`
- `output_contract_ref`

Suggested output-card prompt fields:

- `prompt_spec_ref`
- `prompt_profile_ref`
- `rendered_prompt_ref`
- `rendered_prompt_hash`
- `prompt_assembly_version`
- `output_contract_ref`

Suggested review-artifact prompt fields:

- `prompt_spec_ref`
- `prompt_profile_ref`
- `rendered_prompt_ref`
- `prompt_eval_ref`
- `prompt_output_fit_status`

The principle is that cards should reference prompt artifacts by identity and version, while the full reusable prompt content remains outside the card itself.

### Tracked Record Homes

For milestone-scoped workflow records, ADL should maintain tracked public homes such as:

- `docs/records/v0.85/tasks/<task-id>/stp.md`
- `docs/records/v0.85/tasks/<task-id>/sip.md`
- `docs/records/v0.85/tasks/<task-id>/sor.md`

The exact scope segment may vary in later milestones or non-milestone workflows, but the architectural rule should remain stable: public workflow history is task-centric. GitHub issues are one possible projection of a task, not the primary ontology.

---

## How to Avoid Combinatorial Explosion

The wrong strategy is to create one prompt file for every possible situation.

The better strategy is to model prompts as:

- **role × task × profile**
- plus fragment composition

For example:

- roles: writer, editor, reviewer, planner, verifier
- tasks: draft, critique, summarize, compare, rewrite, classify
- profiles: strict, creative, release, compact

This supports broad coverage without requiring one-off prompts for every run.

Reusable fragments reduce duplication further:

- every verifier prompt may include `output.yaml_only.v1`
- every editor prompt may include `constraint.no_fabrication.v1`
- every seller prompt may include a value-proposition framing block

This makes prompt design more like software architecture and less like ad hoc prompt tinkering.

---

## What Should Be Structured vs. Freeform

### Strongly structured

The following should be explicit and machine-readable:

- prompt identity and version
- role
- task kind
- required inputs
- output schema
- tool policy
- determinism policy
- assembly order
- handoff behavior
- safety and refusal rules
- profile overrides

### Freeform but contained

The following can remain natural language:

- instruction text
- examples
- editorial guidance
- style guidance
- domain-specific nuance

The key rule is that freeform text should live within a typed frame.

---

## Determinism, Replay, and Evaluation

Because ADL values determinism and replay, prompt management must support exact reconstruction of what happened.

Each model invocation should capture at least:

- prompt spec id and version
- fragment ids and versions
- profile id and version
- bound inputs
- rendered prompt hash
- model/provider settings
- tool policy
- output contract

For ADL specifically, replay compatibility should be expressed in the same explicit unchanged-or-delta style already expected elsewhere in the system. A run should be able to state whether prompt replay semantics are unchanged, or identify the exact prompt-surface delta that affects reproducibility. Prompt changes should never be hidden inside informal prose if they can affect replay, output interpretation, or evaluation comparability.

Without this, it becomes difficult to reason scientifically about prompt changes.

This also enables prompt diffing and replay analysis, such as:

- what changed between `writer.first_draft.v1` and `writer.first_draft.v2`
- whether schema compliance improved
- whether token cost increased
- whether output stability regressed

This is especially important for ADL's longer-term goals around benchmarking, adaptive execution, and scientific workflow improvement.

### Minimum Replay Metadata

A minimal replay-safe capture for prompt execution should include:

- `prompt_spec_id`
- `prompt_spec_version`
- `prompt_profile_id`
- `prompt_profile_version`
- `fragment_ids`
- `fragment_versions`
- `prompt_binding_hash`
- `rendered_prompt_hash`
- `assembly_contract_version`
- `model_provider`
- `model_identifier`
- `tool_policy_ref`
- `output_contract_ref`
- `replay_impact`

Where practical, ADL should distinguish between:

- replay semantics unchanged
- replay semantics changed but comparable
- replay semantics changed and not directly comparable

That classification will be important once prompt evolution becomes a routine part of ADL workflow engineering.

---

## Prompt Lifecycle in ADL

A practical lifecycle for structured prompts should look like this:

1. define prompt class
2. define output contract
3. draft prompt artifact locally when needed
4. promote authoritative prompt artifact into tracked task-bundle state
5. create prompt spec
6. compose from fragments
7. create one or more profiles
8. run prompt evaluations on representative tasks
9. promote stable prompts to canonical status
10. version changes carefully

This makes prompt design an engineering discipline rather than a collection of ad hoc experiments.

Where GitHub is used, issue creation or reconciliation should be understood as an integration step driven by the task bundle rather than the source of truth itself.

---

## Key Separation of Concerns

ADL should distinguish four things that are often mixed together in current prompt practice.

### A. Agent identity prompts

Examples:

- “You are a developmental editor.”
- “You are a release verifier.”

### B. Task execution prompts

Examples:

- “Review this chapter for structural weaknesses.”
- “Emit canonical YAML only.”

### C. Protocol prompts

Examples:

- handoff format
- citation format
- checklist application rules
- evidence reporting rules

### D. Policy prompts

Examples:

- do not fabricate
- do not edit repository files
- use repo-relative paths only
- declare uncertainty when evidence is missing

Keeping these concerns separate makes prompt behavior easier to reuse, test, and audit.

---

## Recommended v0.85 Deliverable: Prompt Surfaces v1

This document recommends a v0.85 effort called **Prompt Surfaces v1**.

Initial deliverables:

1. **PromptSpec v1**
   - schema for reusable structured prompts

2. **Prompt Assembly Contract v1**
   - deterministic ordering and rendering rules for fragments, profiles, and bindings

3. **Prompt Library v1**
   - initial curated prompt set for common ADL agent roles

Suggested initial roles:

- planner
- implementer
- reviewer
- verifier
- documentation writer
- release packager

Later follow-on deliverable:

4. **Prompt Evaluation Harness**
   - run benchmark tasks against prompt variants and compare validity, stability, cost, and latency

---

## Suggested Repository Layout

One reasonable directory shape is:

```text
.adl/
  prompts/
    fragments/
      role/
      constraints/
      rubrics/
      outputs/
      handoffs/
    templates/
      writer/
      editor/
      reviewer/
      planner/
      verifier/
    profiles/
      strict/
      creative/
      compact/
      release/
      local-small-model/
    bindings/
      runs/
    rendered/
      runs/
docs/
  milestones/
    v0.85/
      STRUCTURED_PROMPT_ARCHITECTURE.md
  tooling/
    PROMPT_SPEC_V1.md
    PROMPT_ASSEMBLY.md
    PROMPT_LIBRARY_GUIDE.md
```

A reasonable split is:

- canonical design and schema docs under `docs/`
- operational prompt instances and run-time artifacts under `.adl/`

---

## Example Conceptual Flow

A workflow step might proceed as follows:

1. workflow selects step type `authoring.write_section`
2. runtime resolves `PromptSpec` = `writer.first_draft.v1`
3. runtime applies `PromptProfile` = `strict_enterprise`
4. runtime binds inputs from outline, notes, style guide, and source documents
5. assembly renders a canonical prompt in deterministic order
6. model produces output under a declared output contract
7. run stores `RenderedPrompt` and `PromptEval` artifacts for replay and analysis

This is the core path from card-driven orchestration toward robust multi-agent prompt infrastructure.

---

## Practical Minimal Starting Set

To avoid overdesign, ADL should begin with:

### Artifact types

- `PromptSpec`
- `PromptFragment`
- `PromptProfile`
- `PromptBinding`
- `RenderedPrompt`
- `PromptEval`

### Initial fragment kinds

- `role`
- `constraint`
- `output_contract`
- `rubric`
- `handoff`

This is enough to create a real structured prompt subsystem without committing prematurely to every future extension.

---

## Bottom Line

The next stage of ADL should not be “more card types” alone.

The stronger direction is to introduce a structured prompt subsystem beside cards:

- typed
- versioned
- composable
- role-aware
- profile-aware
- replayable
- evaluable

Cards remain the workflow surface.
Structured prompts become the behavior surface.
Together, they provide the foundation for richer multi-agent orchestration in ADL.

---

## Proposed Next Steps

1. define `PromptSpec v1`
2. define `PromptFragment v1`
3. define `PromptProfile v1`
4. define prompt assembly ordering rules
5. define card-level prompt reference fields for input/output/review artifacts
6. define minimal replay metadata for prompt-bearing runs
7. choose initial canonical prompt roles for v0.85
8. add prompt render capture for replayability
9. design a minimal prompt evaluation harness
10. map the new prompt artifacts onto existing ADL workflow and review surfaces

These steps would turn prompt design in ADL from an implicit craft into an explicit architectural subsystem.

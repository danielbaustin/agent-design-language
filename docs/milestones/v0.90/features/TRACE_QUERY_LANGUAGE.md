

# Trace Query Language (TQL) v0

## Purpose

Define the first bounded **Trace Query Language (TQL)** for ADL.

TQL provides a structured way to query:
- Structured Output Records (SOR)
- task-bundle artifacts
- validation and determinism evidence
- integration state
- execution-record metadata

The goal is not a full general-purpose query engine in the first slice. The goal is a deterministic, inspectable query layer over the execution artifacts ADL already produces.

---

## Why TQL Exists

ADL already produces a growing body of structured execution records:
- SOR YAML summaries
- deterministic validation commands
- artifact lists
- replay metadata
- task-bundle records

Today, these are queried informally using:
- `rg`
- `git diff`
- manual inspection
- shell scripts

That works for early development, but it leaves the semantics of the queries implicit.

TQL makes those semantics explicit.

It is the missing layer between:
- execution records
- review tooling
- future Gödel / introspection work

---

## Design Goals

### 1. Deterministic Query Semantics

A TQL query should return the same result for the same artifact set.

No hidden heuristics.
No model inference in the first slice.
No probabilistic interpretation.

---

### 2. Query the Existing Artifact Model

TQL operates over existing ADL artifacts, not a parallel data store.

Primary inputs:
- SOR YAML fields
- structured prompt metadata
- task-bundle file presence
- bounded artifact paths
- validation and replay metadata

---

### 3. Useful Before It Is Fancy

The first version should solve real questions already arising in the repo, for example:
- Which SORs still claim `DONE` while integration is still `pr_open`?
- Which output records have deterministic fixtures but still mark replay as false?
- Which cards touched editor files?
- Which tasks produced schema changes?
- Which issues are missing required proof surfaces?

---

### 4. Compatible with Future Gödel Introspection

TQL v0 is an execution/review tool first.

But it should also form the basis for:
- introspection over prior runs
- pipeline analysis
- future hypothesis and learning queries
- reasoning over execution traces and records

---

## Scope for v0

TQL v0 is intentionally narrow.

It should support queries over:
- SOR verification summary fields
- integration state fields
- artifact presence fields
- simple path and issue/task identifiers
- bounded metadata associated with task bundles

It does **not** need to support:
- arbitrary joins across every repo object
- free-form natural language querying
- distributed trace aggregation
- semantic inference over prose

---

## Query Model

TQL v0 should be understood as:

**structured predicates over execution records and task-bundle artifacts**

A query evaluates over a set of records and returns:
- matching task bundles
- matching SORs
- matching issue/task identifiers
- optionally selected fields

The first implementation may be backed by shell tooling and YAML/JSON extraction rather than a dedicated parser.

---

## Canonical Query Types

### 1. State-Consistency Queries

Examples:
- records where `status == DONE` and `integration_state == pr_open`
- records where `worktree_only_paths_remaining != none` but completion is asserted
- records where schema changes are present but approval is missing

Purpose:
- catch contradiction and workflow drift

---

### 2. Determinism / Replay Queries

Examples:
- records where `determinism.status != PASS`
- records where fixtures exist but replay is still marked false
- records where ordering guarantees are claimed but not verified

Purpose:
- enforce artifact + proof-surface discipline

---

### 3. Validation-Completeness Queries

Examples:
- records missing required validation sections
- records with empty required fields
- records with commands listed but no verification explanation

Purpose:
- strengthen machine-auditable execution records

---

### 4. Artifact-Surface Queries

Examples:
- tasks touching editor artifacts
- tasks producing review-surface artifacts
- tasks modifying prompt schemas or validators

Purpose:
- inspect slices of the system by produced artifact type

---

### 5. Pipeline / Lineage Queries

Examples:
- tasks produced by a milestone band
- Gödel pipeline tasks touching hypothesis / policy / prioritization / evaluation stages
- records associated with a given task bundle or issue number

Purpose:
- support roadmap, review, and later introspection

---

## First Concrete Queries to Support

The first implementation should support a small set of canonical queries.

### Q1. Integration mismatch

Find records where completion status and integration state disagree.

### Q2. Determinism gaps

Find records where deterministic proof surfaces are incomplete.

### Q3. Replay gaps

Find records with rerunnable deterministic surfaces but replay remains false or unspecified.

### Q4. Editor-surface slice

Find records touching:
- `docs/tooling/editor/`
- task-bundle editor artifacts
- STP / SIP / SOR card surfaces

### Q5. Schema-change slice

Find records where schema changes are present.

### Q6. Missing proof-surface records

Find records where proof surfaces are weak or absent.

### Q7. Task-bundle lookup

Find all records associated with a given task bundle or issue number.

---

## Proposed Syntax Direction

The first implementation does not need a heavy parser.

A practical first syntax is:
- field comparisons
- simple boolean conjunctions
- issue/task selectors
- path selectors

Examples of intended query shape:
- `status == DONE and integration_state == pr_open`
- `determinism.replay_verified == false`
- `artifacts contains docs/tooling/editor/`
- `issue in [935,936,937,938,939,940]`

This syntax is illustrative for now. The implementation may initially expose these as preset query names or a thin predicate layer over extracted YAML/JSON.

---

## Implementation Strategy

### Phase 1: Query Presets

Implement a thin CLI wrapper such as:
- `adl-query integration_mismatch`
- `adl-query determinism_gaps`
- `adl-query replay_gaps`
- `adl-query editor_surface`

Back it with:
- YAML/JSON extraction
- shell scripts
- repository-relative path scans

This gives immediate value without committing to a heavy parser too early.

---

### Phase 2: Field Predicate Support

Add a minimal predicate language over known fields.

Likely surfaces:
- SOR verification summary
- integration metadata
- artifact presence
- issue/task identifiers

---

### Phase 3: Pipeline and Introspection Queries

Expand toward:
- milestone slices
- Gödel pipeline stage inspection
- cross-workflow trace queries
- future introspection hooks for learning and evaluation

---

## Relationship to Other Work

TQL depends on:
- Structured Output Record rigor
- task-bundle ontology
- schema validation and normalized fields
- proof-surface discipline

TQL supports:
- review tooling
- release/readiness checks
- Gödel introspection later
- execution debugging and auditability

---

## Non-Goals (v0)

- Full distributed trace infrastructure
- Natural-language querying over arbitrary prose
- Model-based semantic interpretation of documents
- Replacing existing shell tools immediately
- Building a full database layer before there is a real need

---

## Open Questions

- Which fields should be mandatory before TQL is considered stable?
- Should TQL operate directly on Markdown + YAML blocks, or on extracted normalized JSON?
- How much syntax should be user-facing vs preset-query driven?
- When should TQL gain explicit support for task-bundle lineage and public-record browsing?

---

## Summary

ADL already has the beginnings of a trace query language in practice.

TQL v0 makes that layer explicit.

It treats the existing execution records and task-bundle artifacts as a queryable substrate, with deterministic semantics and bounded scope.

This is the right first step toward:
- machine-auditable review
- reliable execution introspection
- future Gödel-facing self-inspection
- a stronger control-plane platform overall
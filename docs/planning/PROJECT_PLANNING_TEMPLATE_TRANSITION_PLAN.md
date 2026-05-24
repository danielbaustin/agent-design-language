# Project Planning Template Transition Plan

Status: current planning-template transition contract

Version target: `v0.91.3`

Related issues: `#3302`, `#3303`, `#3308`, `#3310`

## Purpose

ADL is moving project and milestone planning documents from ad hoc copy/edit
templates to versioned, fillable, reviewable planning-template contracts.

The goal is not to make planning documents more ornate. The goal is to make
planning work repeatable:

- the template version is explicit
- generated drafts are visibly different from reviewed planning truth
- unresolved placeholders are rejected before review
- required sections are checked consistently
- planning-doc cleanup has a dedicated editor skill
- C-SDLC card editors continue to own SIP/STP/SPP/SRP/SOR cards

## Operating Rules

1. A generated planning document is only a draft.
2. A validated planning document is still only a structurally valid draft.
3. A reviewed planning document must record review truth separately from
   generation truth.
4. A planning document must not override GitHub, PR, card, or closeout truth.
5. Card defects still route to card editor skills.
6. Planning-doc defects route to `planning-doc-editor`.

## Scope

This transition applies to milestone and project planning documents such as:

- milestone README packets
- WBS documents
- sprint plans
- feature documents
- design documents
- decision logs
- demo matrices
- milestone checklists
- release plans
- release notes
- handoff packets

This transition does not apply to C-SDLC issue cards:

- SIP
- STP
- SPP
- SRP
- SOR

## Rollout Sequence

### Phase 1: Template Substrate

Issue `#3302` owns the versioned planning-template substrate.

Expected outputs:

- `docs/templates/planning/1.0.0/`
- `docs/templates/planning/current.json`
- a planning-template README
- direct copy/fill forms for existing flat planning templates
- a placeholder vocabulary
- required-section metadata

The flat templates under `docs/templates/` remain compatibility inputs until
the versioned planning templates are accepted.

### Phase 2: Generation Contract

The first generator should be intentionally boring.

Required behavior:

- read `docs/templates/planning/current.json`
- resolve registry template paths relative to the registry/repo root, not cwd
- reject absolute registered template paths as non-portable
- select one named planning document type
- fill declared placeholders from an explicit input map
- write the generated document to an explicit output path
- stamp template version and generation status
- record repo-relative template provenance, not resolved host paths
- avoid hidden defaults that create false milestone truth

Suggested command shape:

```bash
ADL_TMP="${TMPDIR:-/tmp}"

python3 adl/tools/fill_planning_template.py \
  --registry docs/templates/planning/current.json \
  --template readme \
  --values examples/planning/values/minimal.json \
  --output "$ADL_TMP/adl-planning-fixture.md"
```

### Phase 3: Validation Contract

The first validator should reject obvious bad planning drafts before review.

Required behavior:

- fail on unresolved placeholders
- fail on missing required sections for the selected document type
- fail on absolute registered template paths
- use path-aware template-root containment checks rather than string prefixes
- report the template version used
- report generated, reviewed, and approved status truth separately
- avoid treating validation as approval

Suggested command shape:

```bash
ADL_TMP="${TMPDIR:-/tmp}"

python3 adl/tools/validate_planning_template.py \
  --registry docs/templates/planning/current.json \
  --template readme \
  --input "$ADL_TMP/adl-planning-fixture.md"
```

Non-repo-cwd invocation should use symbolic paths rather than machine-local
paths:

```bash
REPO="$(pwd)"
ADL_TMP="${TMPDIR:-/tmp}"
cd "$ADL_TMP"

python3 "$REPO/adl/tools/fill_planning_template.py" \
  --registry "$REPO/docs/templates/planning/current.json" \
  --template readme \
  --values "$REPO/docs/templates/planning/fixtures/minimal/readme_values.json" \
  --output "$ADL_TMP/adl-planning-fixture.md"

python3 "$REPO/adl/tools/validate_planning_template.py" \
  --registry "$REPO/docs/templates/planning/current.json" \
  --template readme \
  --input "$ADL_TMP/adl-planning-fixture.md"
```

Minimum first-slice document types:

- README
- WBS
- sprint
- checklist

If the first implementation slice is narrower, it must say so explicitly in
the issue SOR.

### Phase 4: Editor Routing

Planning-document cleanup uses the new `planning-doc-editor` skill.

Route to `planning-doc-editor` for:

- unresolved placeholder residue
- missing required planning sections
- stale milestone planning claims
- generated-vs-reviewed status drift
- planning document paths that no longer match repo layout
- planning docs that incorrectly claim approval or release truth

Do not route card defects to `planning-doc-editor`. Card defects still route
to SIP/STP/SPP/SRP/SOR editor skills.

### Phase 5: Pilot Packet

The first pilot should use a small fixture rather than a live milestone.

Pilot acceptance:

- generator creates at least one planning document from the registry
- validator accepts the complete fixture
- validator rejects the same fixture after a placeholder is reintroduced
- the fixture is clearly labeled as generated sample data
- no live milestone planning truth is overwritten

### Phase 6: Migration Guidance

After the pilot proves the contract, future milestone planning can move to the
versioned templates.

Migration rules:

- do not bulk rewrite historical milestone documents
- do not silently reclassify old planning docs as generated
- migrate one planning wave at a time
- record the template version used by each new planning packet
- keep historical planning packets readable even if they predate the template
  registry

## Automation Checklist

- `generate` command reads a versioned registry.
- `generate` command writes explicit generation metadata.
- generated Markdown records repo-relative template provenance.
- `validate` command rejects unresolved placeholders.
- `validate` command checks required sections.
- `validate` command rejects absolute registered template paths.
- `validate` command does not mark review or approval.
- `planning-doc-editor` skill exists and has a clear stop boundary.
- `workflow-conductor` routes planning-doc defects to `planning-doc-editor`.
- Card-editor routing remains unchanged.

## Review Checklist

- Is the planning document generated, reviewed, approved, or historical?
- Does the document name its template version if generated?
- Are all placeholders resolved?
- Are required sections present?
- Are milestone status claims source-backed?
- Are issue-card claims delegated to card records instead of duplicated?
- Does the document avoid absolute host paths and transient temp paths?
- Does the SOR distinguish implemented tooling from planned rollout steps?

## Non-Goals

- Rewriting all current milestone planning documents.
- Replacing documentation-specialist for general docs work.
- Replacing SIP/STP/SPP/SRP/SOR editor skills.
- Making planning documents canonical over GitHub or PR state.
- Treating generated documents as publication-ready without review.

## Current State

This issue is stacked on the `#3302` planning-template substrate. The stack
therefore has:

- versioned prompt templates under `docs/templates/prompts/`
- versioned planning templates under `docs/templates/planning/`
- a structural planning-template validator
- a fill helper that generates one planning draft from explicit JSON values
- `planning-doc-editor` routing for planning-document cleanup

The stack still does not migrate live milestone packages or claim generated
planning docs are reviewed or approved.

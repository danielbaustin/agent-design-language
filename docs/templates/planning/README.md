# Versioned Project Planning Templates

This directory is the canonical tracked home for ADL project and milestone
planning templates.

The active template set is declared in [`current.json`](current.json). Each
file under `1.0.0/` is a direct copy-and-fill document, not a fenced example
embedded in prose.

## Current Set

- Template set: `1.0.0`
- Template root: `docs/templates/planning/1.0.0/`
- Registry: `docs/templates/planning/current.json`
- Placeholder style: stable identifier-style angle-bracket placeholders such as
  `<version>` and `<milestone_title>`

## Path Portability Contract

Registry template paths are repo-relative contract paths. They must stay
relative, such as `docs/templates/planning/1.0.0/readme.md`; absolute host paths
are rejected as non-portable.

The helper scripts resolve registered template paths relative to the registry's
repository root, not the caller's current shell directory. This means the same
registry can be used from the repository root or from another working directory
when explicit paths are supplied for the registry, values, input, and output
files.

Generated Markdown records repo-relative template provenance in its header. It
must not record resolved host paths such as user home directories, temporary
worktree paths, or machine-local checkout roots.

## Template-Filled Does Not Mean Reviewed

Planning-template validation is structural. It can prove that a draft has the
right shape, required sections, and no unresolved placeholders.

It does not prove that the plan is reviewed, approved, ready to execute, merged,
released, or true. Those claims must come from issue cards, PR state, review
records, release records, or explicit human decisions.

## Versioning Policy

- Template-set versions use SemVer.
- `1.0.0/` is immutable after adoption except for obvious typo fixes.
- Future semantic changes create a new SemVer directory, such as `1.1.0/` or
  `2.0.0/`, then update `current.json`.
- Tools resolve active paths from `current.json`.

## Template Objects

The first planning-template set includes:

| Key | Role | Path |
|---|---|---|
| `readme` | Milestone README | `docs/templates/planning/1.0.0/readme.md` |
| `wbs` | Work Breakdown Structure | `docs/templates/planning/1.0.0/wbs.md` |
| `sprint` | Sprint Plan | `docs/templates/planning/1.0.0/sprint.md` |
| `vision` | Milestone Vision | `docs/templates/planning/1.0.0/vision.md` |
| `design` | Milestone Design | `docs/templates/planning/1.0.0/design.md` |
| `decisions` | Decision Log | `docs/templates/planning/1.0.0/decisions.md` |
| `demo_matrix` | Demo Matrix | `docs/templates/planning/1.0.0/demo_matrix.md` |
| `feature_doc` | Feature Document | `docs/templates/planning/1.0.0/feature_doc.md` |
| `milestone_checklist` | Milestone Checklist | `docs/templates/planning/1.0.0/milestone_checklist.md` |
| `release_plan` | Release Plan | `docs/templates/planning/1.0.0/release_plan.md` |
| `release_notes` | Release Notes | `docs/templates/planning/1.0.0/release_notes.md` |

## Compatibility

Legacy flat files under `docs/templates/*_TEMPLATE.md` remain compatibility
surfaces until migration is complete. New planning work should prefer the
versioned registry and active template paths in this directory.

Sprint umbrellas and mini-sprints should also use the companion Sprint
Execution Packet template at
`docs/templates/sprints/1.0.0/sprint_execution_packet.md`. The SEP is intentionally
separate from the milestone planning registry: it is an execution-control
surface for child issue order, safe parallelism, PVF notes, review bars, and
closeout bars. It does not replace issue-local `SIP -> STP -> SPP -> SRP ->
SOR` cards.

## Focused Validation

Generate a filled planning draft from explicit JSON values:

```bash
python3 adl/tools/fill_planning_template.py \
  --registry docs/templates/planning/current.json \
  --template readme \
  --values docs/templates/planning/fixtures/minimal/readme_values.json \
  --output docs/templates/planning/fixtures/minimal/readme_generated.md
```

Use the planning-template validator to check generated or filled planning docs:

```bash
python3 adl/tools/validate_planning_template.py \
  --registry docs/templates/planning/current.json \
  --template readme \
  --input docs/templates/planning/fixtures/minimal/readme_generated.md
```

The same commands can run from another working directory with symbolic absolute
inputs:

```bash
REPO="$(pwd)"
ADL_TMP="${TMPDIR:-/tmp}"
cd "$ADL_TMP"

python3 "$REPO/adl/tools/fill_planning_template.py" \
  --registry "$REPO/docs/templates/planning/current.json" \
  --template readme \
  --values "$REPO/docs/templates/planning/fixtures/minimal/readme_values.json" \
  --output "$ADL_TMP/adl-planning-readme.md"

python3 "$REPO/adl/tools/validate_planning_template.py" \
  --registry "$REPO/docs/templates/planning/current.json" \
  --template readme \
  --input "$ADL_TMP/adl-planning-readme.md"
```

The validator checks:

- registry JSON parses
- selected template registry entry exists, is active, and points to an existing
  file under the active template root
- registered template paths are relative and contained within the active
  template root
- required sections for the selected template are present
- unresolved identifier-style angle-bracket or legacy curly placeholders are
  absent from filled outputs

The minimal fixture set currently covers the core first-slice planning
surfaces:

- `readme`
- `wbs`
- `sprint`
- `milestone_checklist`

## Migration Note

This template set does not rewrite existing milestone packages. Future issues
may migrate one planning wave at a time and should record which template version
was used.

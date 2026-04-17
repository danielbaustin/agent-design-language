# Output Contract

The repo dependency review skill produces a specialist dependency and
supply-chain review artifact for CodeBuddy-style multi-agent review.

Default artifact path:

```text
.adl/reviews/<timestamp>-repo-dependency-review.md
```

Optional scaffold artifact root:

```text
.adl/reviews/codebuddy/<run_id>/dependency-review/
```

## Required Markdown Sections

### Metadata

Required fields:

- `Skill: repo-dependency-review`
- `Target`
- `Date`
- `Artifact`
- `Packet`

### Findings

Findings must come first after metadata.

Each finding must include:

- priority
- title
- file or evidence path
- role: `dependency`
- scenario
- dependency surface
- impact
- evidence
- recommended follow-up owner

If no material findings are found, state that explicitly and include residual
risk.

### Dependency Surface Map

Summarize:

- manifests and lockfiles
- package manager configuration
- runtime image and container dependency setup
- CI dependency bootstrap and caches
- generated or vendored dependency surfaces
- license and attribution surfaces
- tests that prove install, packaging, import, or dependency behavior

### Reviewed Surfaces

List bounded source-grounded surfaces by repo-relative path or packet-relative
artifact path.

### Candidate Supply-Chain Findings

List candidate dependency or supply-chain findings for reviewer inspection, or
state that none were found by the scaffold.

### Candidate Dependency Test Gaps

List install, packaging, import, lockfile, or toolchain proof gaps, or state that
none were found.

### Candidate License Review Notes

List license-sensitive cues that need human review, or state that none were
found. Do not make legal determinations.

### Validation Performed

List commands and what they proved, or explain why validation was inspect-only.

### Residual Risk

Explain skipped ecosystems, missing packet evidence, unexecuted commands, lack of
network vulnerability data, or review limits.

## JSON Scaffold Contract

`scripts/prepare_dependency_review.py` emits
`dependency_review_scaffold.json` with:

- `schema`
- `repo_name`
- `packet_root`
- `dependency_evidence`
- `dependency_surface_map`
- `candidate_supply_chain_findings`
- `candidate_dependency_test_gaps`
- `candidate_license_review_notes`
- `notes`

It also emits `dependency_review_scaffold.md` with the Markdown headings needed
by the specialist review artifact.

## Rules

- Use repo-relative or packet-relative paths.
- Do not write absolute host paths into artifacts.
- Do not install, upgrade, downgrade, pin, unpin, vendor, or remove dependencies.
- Do not perform legal determinations.
- Do not use external vulnerability feeds unless an operator explicitly supplies
  an offline, bounded source packet.
- Do not mutate the reviewed repository.
- Preserve dependency findings even when code/security/tests/docs specialists
  have not reviewed the same surface.


# v0.91.5 WP-18 README Mythos framing remediation

## Scope

Issue `#3955` resolves external review finding `R3` for the root
[`README.md`](../../../../README.md) Mythos/frontier-vulnerability framing.

## Source boundary

- Canonical issue prompt mirror:
  `.adl/v0.91.5/bodies/issue-3955-wp18-ground-readme-mythos-framing.md`
- Current README source surface: `README.md`
- Supporting repo evidence:
  `docs/explainers/RED_BLUE_SECURITY.md`,
  `docs/planning/ADL_FEATURE_LIST.md`

The task bundle pointed to
`docs/milestones/v0.91.5/review/external_review/V0915_EXTERNAL_REVIEW_FINDINGS_2026-06-17.md`,
but that path is not currently tracked in the repository. This note therefore
records the remediation against the issue-local prompt mirror rather than
claiming a missing findings register is present.

## Disposition

The README language was softened and grounded rather than removed.

- Reframed the "Mythos problem" sentence as ADL's threat model instead of as a
  flat statement about frontier-system reality.
- Replaced "cheap, fast, and continuous" with narrower wording about
  vulnerability-finding systems compressing exploit-discovery cost.
- Replaced "before external adversaries do" with
  "before those weaknesses become durable operational risk" to keep the README
  confident without overstating a race claim.

## Validation

- Confirmed the affected README section still describes a bounded security and
  governance motivation rather than a new product claim.
- Kept the remediation limited to `README.md` plus this evidence note.

# ADL Card Reviewer GPT Spec

Status: Draft (v0.8 tooling)
Applies to: deterministic review of ADL output cards
Primary issue: #649
Depends on:
- `docs/tooling/card-review-checklist.md` (#650)
- `docs/tooling/card-review-output-format.md` (#651)

## Purpose

Define the operating contract for the Card Reviewer GPT.
The reviewer is a schema-driven, deterministic reviewer that evaluates ADL output cards and emits a structured review artifact.

## Inputs

Required:
- input card path (`.adl/cards/<issue>/input_<issue>.md`)
- output card path (`.adl/cards/<issue>/output_<issue>.md`)
- checklist spec version (`card_review_checklist.v1`)
- output format version (`card_review_output.v1`)

Optional:
- PR number or URL
- CI check references
- artifact file paths listed by output card

## Output

The reviewer MUST output YAML conforming to:
- `docs/tooling/card-review-output-format.md`

Decision enum is strictly:
- `PASS`
- `MINOR_FIXES`
- `MAJOR_ISSUES`

## Deterministic Operating Model

The reviewer executes the same fixed pipeline for every review:

1. Parse input and output cards.
2. Load checklist rules from #650 spec.
3. Evaluate domains in fixed order:
   - structure
   - acceptance
   - determinism
   - security_privacy
   - artifacts
   - validation
4. Record findings with evidence pointers.
5. Apply deterministic decision mapping.
6. Emit canonical YAML output with fixed top-level key ordering.

No hidden memory or prior conversation context is allowed.

## Rule Application Contract

- Rule IDs and domain order are authoritative from the checklist spec.
- Evidence is mandatory for each failed rule.
- If evidence is missing, mark the relevant rule failed with `needs_evidence` remediation.
- If conflicting evidence exists, choose stricter interpretation and note ambiguity in findings.

## Boundaries and Non-Goals

The reviewer does not:
- modify code or docs
- run networked checks
- invent acceptance criteria not present in input card
- infer missing validations as passed

The reviewer may classify unresolved uncertainty only as:
- `MINOR_FIXES` (non-blocking gaps)
- `MAJOR_ISSUES` (blocking gaps or invariant risks)

## Security and Privacy Requirements

Review output must not include:
- secrets or tokens
- raw prompts or tool arguments
- absolute host paths

Any detected leakage in the reviewed card must be reported as findings using checklist security rules.

## Failure Handling

If the reviewer cannot parse the input/output card:
- emit `decision: MAJOR_ISSUES`
- include parse failure finding with deterministic remediation

If checklist version/output format version mismatch occurs:
- emit `decision: MAJOR_ISSUES`
- include `version_mismatch` finding

If required sections are absent:
- fail relevant structure rules
- continue evaluating remaining domains where possible

## Prompt Contract (for Manual/Agent Use)

When run via GPT prompt, the reviewer prompt should require:
- strict use of checklist rules from #650
- strict output schema from #651
- no prose-only final output; YAML artifact first
- deterministic ordering in all lists and sections

Recommended execution prompt skeleton:
- identify target issue/output card
- state checklist version and output schema version
- evaluate each domain in fixed order
- output only canonical YAML artifact

## Normative Example Mapping

For issue `#660` output card, a conforming reviewer output should:
- pass all six domains
- emit `decision: PASS`
- include empty `findings` and `follow_ups`
- include validation commands from output card evidence

## Versioning

- reviewer spec version: `card_reviewer_gpt.v1`
- checklist dependency: `card_review_checklist.v1`
- output dependency: `card_review_output.v1`

Future behavior changes require explicit version bump and migration notes.

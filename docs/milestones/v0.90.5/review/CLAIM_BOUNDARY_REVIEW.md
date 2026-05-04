# Claim Boundary Review - v0.90.5

## Scope

This note dispositions the WP-22 `IR-002` review gates that remained open in
the milestone checklist:

- public-spec language checked for overclaiming
- UTS validity never described as execution authority
- privacy and redaction claims backed by tests

The scope is intentionally bounded to tracked `v0.90.5` milestone docs and
existing proof artifacts. It does not reopen the full internal-review packet or
claim milestone completion.

## Sources Reviewed

- `docs/milestones/v0.90.5/README.md`
- `docs/milestones/v0.90.5/VISION_v0.90.5.md`
- `docs/milestones/v0.90.5/DECISIONS_v0.90.5.md`
- `docs/milestones/v0.90.5/review/dangerous-negative-suite-report.json`

## Disposition

### 1. Public-spec language checked for overclaiming

Result: passed

The tracked milestone docs already keep the public-spec claim bounded. They say
that UTS is `public-compatible` or may become public infrastructure, but they
do not claim that UTS is already a public standard or that it independently
delivers ADL safety. The README and vision both carry explicit non-claims
against overreach.

### 2. UTS validity never described as execution authority

Result: passed

The tracked milestone docs consistently preserve the authority boundary:

- `README.md`: UTS describes portable tool shape while ACC defines runtime
  authority, identity, privacy, visibility, trace, replay, and Freedom Gate
  requirements.
- `VISION_v0.90.5.md`: states explicitly that UTS validity is not runtime
  authority and that ACC is where authority lives.
- `DECISIONS_v0.90.5.md`: records `D-04`, "ACC owns ADL runtime authority,"
  and says UTS metadata must not imply permission to execute.

No tracked wording repair was required for this gate.

### 3. Privacy and redaction claims backed by tests

Result: passed

Tracked proof already exists in
`docs/milestones/v0.90.5/review/dangerous-negative-suite-report.json`.

That artifact records a passing dangerous-negative suite with `case_count: 9`
and `passed: true`, including:

- repeated `redaction_summary` entries showing private arguments were redacted
- `prompt_or_tool_arg_leakage_detected: false` across the recorded cases
- an explicit `prompt_or_tool_argument_leakage_denied` case denied by the
  Freedom Gate with reason `private_arguments_not_redacted`

Those results are sufficient backing for the bounded milestone-doc claims about
privacy, redaction, and leakage refusal in the `v0.90.5` package.

## Outcome

The three open `IR-002` checklist gates are now supported by tracked milestone
docs and tracked proof artifacts, so the checklist can mark them complete
without inventing new validation or broadening milestone scope.

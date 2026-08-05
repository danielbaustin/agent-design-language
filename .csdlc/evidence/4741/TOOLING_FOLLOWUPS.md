# Issue 4741 Tooling Follow-ups

## Installed owner-binary provenance

The exact-head live wrapper attempt selected `skipped_fail_closed` and returned
`owner_binary_provenance_invalid` before Unity launch. The repository-installed
`.adl/bin/adl` source hash did not match the current declared source root. No
replacement binary was built or installed.

Required follow-up:

- provide a typed repository-approved installer reconciliation path;
- preserve provenance checks;
- rerun the #4741 wrapper against the intended Unity project;
- retain exact success or blocker truth without claiming scene validation.

Proposed title:

`[v0.91.8][unity][tooling] Reconcile installed ADL owner-binary provenance for Unity live proof`

## Readiness requirement declaration

The first #5651 readiness request copied a historical five-check declaration
and incorrectly treated path-skipped `adl-demo-proof` as a required failure.
Current repository policy identifies `adl-ci` and `adl-coverage` as the stable
required branch-protection contexts. Both passed on the current PR head.

Required follow-up:

- bind typed readiness requirements to canonical repository policy;
- keep intentionally path-skipped lanes visible but optional;
- prevent callers from weakening required aggregate checks;
- cover skipped, absent, failed, and policy-mismatched observations.

Proposed title:

`[v0.91.8][csdlc-v2][readiness] Bind readiness requests to canonical required-check policy`

## Issue-creation transport blocker

The connected GitHub app returned HTTP 403 `Resource not accessible by
integration` for both proposed issue creations. No `gh` or legacy lifecycle
fallback was used. These drafts remain attached to #4741 until an authorized
typed issue writer is available.

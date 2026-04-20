# v0.90.1 Internal Review

Issue: #2155

Status: complete for WP-15. This is an internal review report, not a release
approval. The findings below are intended to feed WP-16 remediation and the
release-tail review path.

## Scope

This review checked the closed v0.90.1 implementation tranche against the
tracked milestone package, feature docs, proof scripts, demo surfaces, and
review expectations. It used the repository review skill family, a repo packet,
multi-agent specialist review, and direct validation commands.

The review explicitly did not implement remediations. Open review-tail issues
remain separate execution surfaces.

## Executive Summary

v0.90.1 is a substantial tranche of work. The Runtime v2 foundation, CSM
Observatory surfaces, operator controls, review skills, and quality-gate
scaffolding are real and reviewable.

The internal review found no release-stopping absence of the whole tranche, but
it did find several proof-quality gaps that should be remediated before final
release closeout. The highest-risk theme is not missing implementation in bulk;
it is proof surfaces that overstate what they have actually demonstrated.

## P1 Findings

### P1: Quality gate can fail open

`adl/tools/demo_v0901_quality_gate.sh` can record `FAIL` in its manifest while
still exiting successfully. Since `QUALITY_GATE_v0.90.1.md` treats this as a
canonical proof surface, operators could believe the quality gate passed when
the underlying checks failed.

Recommended remediation: make the quality gate exit nonzero whenever a required
check fails, preserve the manifest for diagnosis, and add a negative test that
proves fail-closed behavior.

### P1: Security-boundary proof is not proving refusal

The Runtime v2 security-boundary proof appears to use an allowed command path,
specifically `resume_manifold`, and reports refusal based on command-name
matching rather than a distinct denied operation. That means the proof artifact
may not actually demonstrate a refused unsafe wake or resume path.

Recommended remediation: add a distinct refused attempt artifact or command,
make the refusal decision explicit, and add tests that prove allowed and denied
paths are separated.

### P1: Runtime v2 proof bundle declares artifacts that are not emitted

Runtime v2 proof metadata references required artifacts such as
`runtime_v2/traces/events.jsonl` and `runtime_v2/invariants/policy.json`, but
the implementation does not appear to materialize those artifacts. The docs and
proof bundle are therefore ahead of reality.

Recommended remediation: either emit the referenced artifacts with validation,
or downgrade the docs and proof references to only the artifacts that actually
exist.

## P2 Findings

### P2: CSM Observatory validation is shallower in Rust than in Python

The Rust-side CLI validation checks a limited set of top-level structure and
classification fields. It does not appear to enforce the fuller redaction, path,
required-section, and action-shape checks covered by the Python validator.

Recommended remediation: bring the Rust validator to parity with the canonical
schema checks or invoke the stronger validator as part of the CLI proof path.

### P2: Rehydration refusal behavior is overclaimed

The docs describe wake refusal behavior, but the default generated operator
report always reports `wake_allowed: true`, and failed preconditions appear to
prevent a valid `wake_allowed: false` proof surface from being produced.

Recommended remediation: add an explicit negative rehydration fixture and
operator report that demonstrates a refused wake path without invalidating the
entire proof surface.

### P2: Static console proof is grep-based rather than execution-grounded

The CSM static-console proof verifies expected strings and generated files, but
it does not deeply parse the fallback packet or prove the real render path
behavior beyond the narrow fixture.

Recommended remediation: validate the generated fallback packet structurally and
add a render-path proof that is stronger than string presence.

### P2: Generated CSM operator report contains stale future-work language

Some generated report language still says snapshot and wake proof remains
future work, even though WP-08 is closed and the milestone docs treat
snapshot/rehydration as implemented.

Recommended remediation: update generated operator-report wording to match the
landed WP-08 state and reserve future-work language for unimplemented follow-on
scope only.

### P2: Release and root docs have stale lifecycle wording

Several high-level docs still describe WP-13, WP-14, or WP-15 as pending or
pre-WP15, despite those work packages being closed or underway.

Recommended remediation: normalize release-facing docs after WP-15 publication
so the release-tail state is truthful.

## P3 Findings

### P3: Release notes omit WP-17 and WP-18 from the release tail

The release notes still need to account for the remaining tail work so the
milestone closeout story is complete.

### P3: Runtime v2 CLI absolute output-path behavior needs a policy decision

Some Runtime v2 commands accept absolute `--out` paths, create directories, and
print host paths. That may be acceptable for local demos, but it should either
be documented as allowed or normalized/redacted.

### P3: Runtime v2 and CSM command models are parallel but not unified

Runtime v2 operator commands and CSM command packets appear to maintain separate
command and safety models. A shared availability/safety matrix would reduce
drift.

### P3: Some v0.90.1 docs still point at pre-refactor Runtime v2 paths

A few docs still reference older locations like `adl/src/runtime_v2.rs`, while
the code has moved into the split `adl/src/runtime_v2/` module tree.

### P3: Dependency and supply-chain gate remains light

There is no strong release gate around dependency auditing, license review, or
SBOM-style dependency proof. This is probably follow-on unless release owners
choose to raise the v0.90.1 release bar now.

### P3: Runtime v2 integrated demo does not fully resolve every evidence link

The integrated demo verifies expected artifact presence and counts, but it does
not appear to resolve every proof artifact or evidence reference end-to-end.

### P3: Feature-doc index is missing the static-console issue mapping

`FEATURE_DOCS_v0.90.1.md` is missing the static-console issue mapping for the
static console work.

## Validation Evidence

Commands run:

- `bash adl/tools/test_multi_agent_repo_review_specialist_skill_contracts.sh`
  passed and verified specialist skill contract coverage.
- `bash adl/tools/test_demo_v090_codebuddy_review_showcase.sh` passed and
  verified the CodeBuddy review showcase demo.
- `bash adl/tools/test_demo_v0901_csm_observatory_operator_report.sh` passed
  and verified the CSM Observatory operator report proof.
- `bash adl/tools/test_demo_v0901_csm_observatory_static_console.sh` passed and
  verified the CSM Observatory static console proof.
- `bash adl/tools/demo_v0901_csm_observatory.sh <tmp-output-dir>` passed after
  correcting an invalid flag-style invocation to the script's positional output
  directory interface.
- `bash -x adl/tools/test_multi_agent_repo_review_skill_suite_contracts.sh`
  failed because the contract test still expects the older shared
  `MULTI_AGENT_REPO_REVIEW_SKILL_SUITE.md` reference doc while current skill
  metadata points at per-skill schema documents.

Commands started but not used as passing evidence:

- `bash adl/tools/test_demo_v0901_quality_gate.sh`
- `bash adl/tools/demo_v0901_quality_gate.sh`

These quality-gate commands were interrupted during review rather than recorded
as passing evidence. The fail-open behavior above remains a review finding.

## WP-16 Remediation Queue

Recommended WP-16 ordering:

1. Fix quality-gate fail-closed behavior.
2. Fix the Runtime v2 security-boundary refusal proof.
3. Align Runtime v2 proof bundle artifacts with emitted artifacts.
4. Bring CSM Observatory validation and operator-report truth into parity with
   the stronger proof expectations.
5. Normalize stale milestone and release-tail docs.

## WP-16 Issue Routing

The initial one-finding-per-issue queue was intentionally consolidated after
review so v0.90.1 can finish remediation sequentially without paying a full
issue ceremony for every small finding. The accepted remediation work is routed
through these bundled issues:

| Bundle issue | Scope | Internal review findings covered |
| --- | --- | --- |
| #2221 | Quality gate and quality posture remediation | P1 quality gate can fail open; P3 dependency and supply-chain gate remains light; large-module/quality-posture disposition carried as release-tail truth. |
| #2222 | Runtime v2 proof truth and command semantics | P1 security-boundary proof is not proving refusal; P1 Runtime v2 proof bundle declares artifacts that are not emitted; P2 rehydration refusal behavior is overclaimed; P3 absolute output-path policy; P3 Runtime v2 and CSM command-model alignment; P3 integrated-demo evidence-link depth. |
| #2224 | CSM Observatory validation and report alignment | P2 CSM Observatory validation is shallower in Rust than in Python; P2 static console proof is grep-based; P2 generated CSM operator report contains stale future-work language; P3 feature-doc index missing static-console issue mapping. |
| #2229 | Release docs routing and architecture truth | P2 release and root docs have stale lifecycle wording; P3 release notes omit WP-17 and WP-18; P3 docs still point at pre-refactor Runtime v2 paths; architecture/release truth cleanup needed for final review. |

Superseded child issues from the temporary fine-grained queue should remain
closed as superseded by the applicable bundle. The final release-tail review
should verify the merged bundle PRs rather than re-opening the original
fine-grained queue.

## Release-Tail Disposition

WP-15 is complete as an internal review. v0.90.1 should continue through
third-party review, remediation, and release closeout before being treated as
release-ready.

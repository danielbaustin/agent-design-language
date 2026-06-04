# Local Preflight And CI Integration Validation Policy (#3607)

Issue: #3607
Status: implementation policy

## Purpose

ADL issue work should run the right proof at the right layer. Local preflight
proves the touched surface is coherent before PR publication. CI integration
proves merge-context behavior, coverage, and release-gate posture. Deferring
broad proof to CI is valid only when the local record says so truthfully and the
change is not in a surface that requires broad local proof before publication.

## Responsibility Model

| Change surface | Local preflight proof | CI integration proof | Broad local proof before PR |
| --- | --- | --- | --- |
| Docs-only milestone/review/policy docs | `git diff --check`, link/path/YAML or template checks that match the touched docs | CI docs/path policy and repository integration checks | Not required unless docs change executable policy or release gates |
| Prompt-card generation/repair | Renderer values/structure/schema checks for touched cards plus card validators | CI integration checks | Not required unless schema/tooling code changed |
| C-SDLC control-plane/tooling policy | `bash adl/tools/run_owner_validation_lane.sh csdlc` plus touched focused Rust tests when applicable | CI integration and coverage | Required only when the change affects Rust behavior outside focused owner lanes |
| Runtime/provider/demo/agent behavior | `bash adl/tools/run_owner_validation_lane.sh runtime --build` plus focused Rust selector for touched behavior | CI integration, coverage, and release/nightly proof where applicable | Required for risky runtime semantics, provider execution, signing/security, or release-gate behavior |
| Review tooling/review packet contracts | `bash adl/tools/run_owner_validation_lane.sh review --build` plus focused Rust selector for touched behavior | CI integration and coverage | Required for broad review-runtime coupling or schema changes |
| Cross-owner command routing | `bash adl/tools/run_owner_validation_lane.sh all --build` | CI integration and coverage | Required for Cargo/workspace/package topology changes |
| Release/nightly/PVF/slow-proof policy | Focused policy tests for the touched runner/manifest | Authoritative release, nightly, slow-proof, and coverage gates | Required when local policy code changes can hide or bypass release proof |

## Proof Wording Rules

SOR and PR validation sections must distinguish:

- `Local preflight run`: commands actually run before PR publication.
- `CI integration proof`: checks expected to run in GitHub after PR creation.
- `Release-gate proof`: slow-proof, coverage, or nightly/release validation that
  is intentionally not run locally for ordinary PRs.
- `Validation not run locally`: explicit statement that a broad or release proof
  was deferred to CI/release gate, with the reason.

Never write “full validation passed” when only a focused owner lane ran. Use
language such as:

```text
Local preflight: PASS (`bash adl/tools/run_owner_validation_lane.sh csdlc`).
CI integration: deferred to GitHub `adl-ci` / `adl-coverage`.
Release-gate proof: not required for this docs/tooling-local change.
```

## Finish Validation Profiles

The Rust `pr finish` validation selector owns the default local profile:

- docs-only paths select docs-only local proof;
- known focused control-plane and owner-lane surfaces select
  `FocusedLocalCiGated`;
- broad or unknown source changes select full Rust validation.

The selector must prefer actual changed tracked paths over a broad `--paths .`
request so the PR body does not overclaim local proof.

## Non-Claims

- This policy does not remove CI gates.
- This policy does not make CI the only proof source for risky runtime changes.
- This policy does not replace release/nightly/PVF slow-proof evidence.
- This policy does not claim every future path is classified; unclassified
  source paths should remain full-local-validation by default.

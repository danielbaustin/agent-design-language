# #5763 Feature Crosswalk Digest Reconciliation

## Intent

Reconcile the retained v0.91.8 feature-preservation crosswalk with the current
canonical feature-list source row digest after the reviewed WP-14 decomposition
changed one source row and the matching crosswalk entry.

## Scope

- Update only stale digest metadata required for
  `.csdlc/prepared/issues/5594/validate_feature_crosswalk.rb` to match the
  current 122-row canonical feature list.
- Preserve the row-by-row validator, digest guard, owner classifications,
  source-line parity, and canonical field checks.
- Do not widen documentation scope, regenerate unrelated crosswalk content, or
  weaken validation.

## Validation

- `ruby .csdlc/prepared/issues/5594/validate_feature_crosswalk.rb`
- `ruby .csdlc/prepared/issues/5594/validate_structured_planning.rb`
- `ruby .csdlc/prepared/issues/5594/validate_links.rb`
- YAML parse for `docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml`
- `git diff --check origin/main...HEAD`

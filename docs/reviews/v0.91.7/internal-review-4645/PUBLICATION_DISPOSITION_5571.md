# Publication disposition for issue #5571

## Decision

The already-public WP-18 review bundle may remain publicly retained as historical
review evidence. This decision does not authorize publishing machine-local scratch
artifacts or expanding the external-review corpus beyond its explicit manifest.

## Audit scope and result

The bounded deterministic audit scanned all 18 source files that existed below
`docs/reviews/v0.91.7/internal-review-4645/` before the audit report was emitted.
It found no blockers, no likely secrets, no credential values, and one warning:
the generic scanner looked for `run_manifest.json` at the bundle root while the
tracked manifest is correctly retained at `packet/run_manifest.json`. A second,
explicit manual-review record resolves that scanner-layout warning, enumerates
the final 21-file bundle, and records the inspection method and result.

The nested manifest's original `privacy_mode: local_only` and
`publication_allowed: false` values are preserved as historical truth. They govern
the original packet-generation act; this retrospective disposition does not rewrite
them. Public reuse is restricted to the tracked, reviewed corpus selected by
`docs/milestones/v0.91.7/review/external_review_4646/REVIEW_CORPUS.v1.txt`, merged
in PR #5579. The `live-state/` and `validation/` trees remain historical public
evidence but are excluded from the external-review dispatch corpus because they
contain machine-local or transient operational context.

## Per-surface disposition

| Surface | Disposition | Basis |
|---|---|---|
| Four top-level review Markdown files | allowed | Review prose contains no credential values or private host paths. |
| `packet/` | allowed as historical evidence | No blockers found; nested manifest is retained and its restrictive generation-time policy remains unchanged. |
| `live-state/` | public retention allowed; exclude from dispatch | Contains transient repository/GitHub state and machine-local operational context, but no demonstrated secret exposure. |
| `validation/` | public retention allowed; exclude from dispatch | Logs are bounded proof artifacts; no secret exposure was found. |
| `redaction-audit-5571/` and this disposition | allowed | Three self-authored audit outputs were manually inspected and contain aggregate findings and policy only. |

## Final classification

- Secret or credential exposure: not demonstrated.
- Unsafe material requiring removal: none found.
- Historical public retention: allowed with the restrictions above.
- External-review publication: limited to the explicit publication-safe corpus.
- Residual publication blocker for #4645: resolved by the deterministic scan,
  completed manual-review record, and tracked publication-safe corpus.

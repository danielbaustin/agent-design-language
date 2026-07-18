# Tools Workflow Reliability Tail Closeout (#5036)

Observed: 2026-07-16
Remediation: #5407

## Sprint Truth

#5036 is closed. All eleven declared child issues are closed through merged
pull requests. This retained synthesis records the complete declared wave,
including #5037 and #4938, which were missing or incomplete in earlier retained
closeout surfaces. The live GitHub observations supporting the issue, PR, and
check-rollup fields are retained in `github-closeout-snapshot-5036.json`.
The focused command results and strengthened complete-matrix check are retained
in `VALIDATION.md`.

| Child | Pull request | Head revision | Merge commit | State |
| --- | --- | --- | --- | --- |
| #5034 | #5056 | `52192c72b2d2f7cc64e700c6681bd5acada5827d` | `34a01b9f7f54ca2fa2e0b76a8a60606e6c942e33` | closed / merged |
| #5032 | #5058 | `faf6094ffedc346b76a888477149274c8677fdc3` | `9ea9298894f8bb09349f7ca2504e6f98e923ea3a` | closed / merged |
| #5037 | #5046 | `eb0aead9954b586443ccbfb3377cc4bfc3f2bfaa` | `8d129a6b7392ae91cecfac7b7c1c8573fdb952e8` | closed / merged |
| #5031 | #5060 | `38be1a63b45d90fb4118292422774e380d51d86a` | `40bbe721b93a5054eaad0cdc9e01b03e09668552` | closed / merged |
| #5028 | #5030 | `52e086f973d7325eb8280f28a7185874ed270a75` | `30660d741456e2c7ec6765b384efb04329760a45` | closed / merged |
| #5012 | #5029 | `88868a11692b954bfc3e3d43aad3f110081cadf4` | `0ec4d6a047aba00c61eabbd6284e48b18426471e` | closed / merged |
| #5002 | #5066 | `8bd480320f33ad9c8dbf38a86a7bec80c030db74` | `6c6cabf1782e6688981134fa36a356625db67473` | closed / merged |
| #4999 | #5071 | `a55636569e879a9b0572f2cf9955794834376c02` | `de28105573ecb735c25ebe42852f96b3704218dd` | closed / merged |
| #4995 | #5074 | `c01f61e705d3dc695afaa85839564023946b3282` | `40a0c8d471e39b6be3278324e3111a0a8619b616` | closed / merged |
| #4987 | #5103 | `539938e3b0f50839f4de935e584e896518c156ea` | `507495693525db12ca746f7022dfc96795a7342c` | closed / merged |
| #4938 | #5128 | `39e1d1253c28d89f307afe362f497e58e5dcb8c0` | `4c86b17b53765525057b2929a8b2324ab51ec6d7` | closed / merged |

The snapshot records the check conclusions reported by GitHub at observation
time. It does not infer branch-protection requirements from the rollup.

## Review Corrections

- Build-action logs are implemented for `validation_manager.py --run`. The
  original #5032 `pr.sh`, remote-builder, CI-ingestion, watcher, shepherd, and
  closeout integrations are not current claims after Gate 10D2 v1 sunset.
- Typed binaries under `csdlc-v2/` are the sole current C-SDLC authority.
- #5037 proves a focused CI contract split and green integration. No material
  hosted wall-clock speedup is claimed.
- #5406 supplies retained terminal lifecycle authority; this synthesis supplies
  the issue-wave-specific child and PR matrix.

## Residual Boundary

Future expansion of build-action-log producers or consumers requires a new
bounded issue with tests and retained proof. Comparable hosted before/after
timings are required before making a material CI speedup claim.

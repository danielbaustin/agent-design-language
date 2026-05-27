# CodeFriend Review Follow-up

Date: `2026-05-27`

Issue: `#3428`

## Purpose

Record the focused post-review follow-up that addressed the CodeFriend
mini-sprint findings without reopening the broader sprint scope.

## Findings addressed

1. Root ADL CodeFriend `SOR` records still read like pre-run scaffolds.
2. The product-repo deploy path did not enforce the narrow deploy-role boundary.
3. The verification script only checked HTTPS success, not served content.
4. Local Terraform state residue remained in the working copy.
5. `.terraform.lock.hcl` was ignored instead of tracked.

## Fixes applied

### ADL-side lifecycle truth

- normalized the root `SOR` records for `#3372` through `#3376` so they no
  longer claim the CodeFriend sprint never started
- preserved merged integration state while removing the stale pre-run summary
  language

### Product-repo hardening

External repository:
- `https://github.com/agent-logic/codefriend.ai`
- hardening commit: `70929a9` `hardening: tighten CodeFriend deploy path`

Changes included:

- tracked `infra/terraform/.terraform.lock.hcl`
- stopped ignoring the provider lockfile
- removed local Terraform state from the working copy and kept it untracked
- hardened `scripts/deploy_site.sh` so it refuses broad ambient credentials and
  expects the narrow deploy role
- hardened `scripts/verify_site.sh` so it checks served body content and
  rejects forbidden content markers
- added `.github/workflows/deploy-site.yml` as the canonical OIDC-backed deploy
  path
- updated deployment/publication docs to match the guarded deploy model

## Focused proof

- `bash adl/tools/pr.sh closeout 3372 --version v0.91.4 --no-fetch-issue`
- `bash adl/tools/pr.sh closeout 3373 --version v0.91.4 --no-fetch-issue`
- `bash adl/tools/pr.sh closeout 3374 --version v0.91.4 --no-fetch-issue`
- `bash adl/tools/pr.sh closeout 3375 --version v0.91.4 --no-fetch-issue`
- `bash adl/tools/pr.sh closeout 3376 --version v0.91.4 --no-fetch-issue`
- `CODEFRIEND_CLOUDFRONT_DOMAIN_NAME=dgqj4hit346az.cloudfront.net bash scripts/verify_site.sh`
- `CODEFRIEND_SITE_BUCKET_NAME=codefriend-ai-site CODEFRIEND_CLOUDFRONT_DISTRIBUTION_ID=E2XSCJK1A9P98S CODEFRIEND_DEPLOY_ROLE_ARN=arn:aws:iam::602077092456:role/codefriend-ai-github-deploy bash scripts/deploy_site.sh`
  - expected and observed result: refusal under broad IAM user credentials
- `bash -n scripts/deploy_site.sh scripts/verify_site.sh scripts/rollback_site.sh`
- `git diff --check`

## Outcome

The review findings are now addressed without reopening the completed
CodeFriend sprint into a broader redesign. The ADL closeout truth is tighter,
and the product repo now has a more credible narrow deploy path and a stronger
verification script.

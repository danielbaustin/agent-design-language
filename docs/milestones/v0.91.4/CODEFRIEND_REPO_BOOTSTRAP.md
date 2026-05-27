# CodeFriend Repo Bootstrap

Date: `2026-05-27`

Issue: `#3373`

## Status

Completed bootstrap for the private CodeFriend pre-alpha site repository.

## Repository

- GitHub repository: `https://github.com/agent-logic/codefriend.ai`
- Visibility: private

## What was created

The repository now contains a minimal, public-ready pre-alpha scaffold:

- `README.md`
- `LICENSE.md`
- `docs/DEPLOYMENT.md`
- `docs/SECURITY_AND_PUBLICATION_BOUNDARY.md`
- `docs/SOURCE_MAP.md`
- `site/index.html`
- `site/styles.css`
- `infra/terraform/`

## Reuse from `agent-logic.ai`

The bootstrap intentionally reused the proven `agent-logic.ai` infrastructure
shape for:

- private S3 origin
- CloudFront with OAC
- ACM certificate handling
- Route53 alias records
- GitHub OIDC deploy-role modeling
- Terraform-owned infrastructure boundary

The prose templates were not copied blindly. The CodeFriend repository docs
were written fresh to preserve the CodeFriend-specific pre-alpha boundary.

## Important correction made during review

Initial Terraform bootstrap adapted the apex-domain pattern but did not yet
cover `www.codefriend.ai`.

That review finding was fixed before issue publication:

- ACM certificate now includes `www.codefriend.ai`
- CloudFront aliases now include both apex and `www`
- Route53 alias records now cover both apex and `www`

## Claim boundary

This bootstrap does **not** mean the site is live yet.

What it means:

- the private product/site repository exists
- the first static site assets exist
- the Terraform scaffold exists for the later infrastructure issue

What it does **not** mean:

- no public launch yet
- no product availability claim
- no application runtime
- no analytics, forms, signup flow, or customer data systems

## Validation summary

Validated during bootstrap:

- repository exists privately on GitHub
- scaffold contains no stale `CodeBuddy` naming
- scaffold contains no obvious secret material or local credential paths
- review pass found one real infrastructure gap, which was fixed

## External bootstrap commits

- `30f1bd4` `bootstrap private CodeFriend pre-alpha site repo`
- `de248e4` `infra: add www hostname support to bootstrap scaffold`

## Next step

`#3374` should turn the scaffolded site assets into the final reviewed
coming-soon page for local and later public use.

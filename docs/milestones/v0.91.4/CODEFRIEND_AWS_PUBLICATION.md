# CodeFriend AWS Publication

Date: `2026-05-27`

Issue: `#3375`

## Status

Completed the Terraform-managed AWS static-site substrate for CodeFriend and
published the coming-soon page live over HTTPS.

## External repository state

- Repository: `https://github.com/agent-logic/codefriend.ai`
- Infrastructure/publication commit: `fcf95f8` `infra: launch CodeFriend static site`

## Live endpoints

- `https://codefriend.ai`
- `https://www.codefriend.ai`
- CloudFront distribution domain: retained in private infrastructure records

## What changed

The private CodeFriend product/site repository now contains the first live
static-site substrate:

- private S3 origin bucket
- CloudFront distribution with origin access control
- ACM certificate for `codefriend.ai` and `www.codefriend.ai`
- Route 53 alias records for apex and `www`
- reuse of the existing shared GitHub Actions OIDC provider in the target AWS
  account
- narrow content publication scripts:
  - `scripts/deploy_site.sh`
  - `scripts/verify_site.sh`
  - `scripts/rollback_site.sh`
- expanded deployment documentation in `docs/DEPLOYMENT.md`

## Validation and live proof

Infrastructure proof:

- `terraform init -input=false`
- `terraform validate`
- `terraform plan -out=tfplan`
- `terraform apply -auto-approve tfplan`

Publication proof:

- `scripts/deploy_site.sh`
  - synced `site/` to the private S3 origin bucket
  - requested CloudFront invalidation
- `scripts/verify_site.sh`
  - verified the CloudFront distribution domain over HTTPS
  - verified `https://codefriend.ai`
  - verified `https://www.codefriend.ai`
- direct `curl` checks against both custom domains returned `HTTP/2 200` and
  served the approved coming-soon page body

Additional focused checks:

- `bash -n scripts/deploy_site.sh scripts/verify_site.sh scripts/rollback_site.sh`
- `git diff --check`
- Route 53 record inspection confirmed apex and `www` alias records plus ACM
  validation CNAMEs

## Acceptance summary

- S3 is the private origin and not the public proof surface: yes
- CloudFront is the public HTTPS delivery layer: yes
- ACM certificate is in `us-east-1`: yes
- Route 53 apex and `www` alias records are live: yes
- no forms, analytics, signup systems, or app runtime were introduced: yes
- no credential paths or secrets were committed: yes

## Next step

`#3376` should record the publication safety, verification, rollback, and
handoff truth for the live CodeFriend coming-soon surface.

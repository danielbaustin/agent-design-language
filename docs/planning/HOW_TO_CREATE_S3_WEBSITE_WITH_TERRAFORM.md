# How To Quickly Create An S3 Website With Terraform

## Purpose

This guide captures the reusable ADL/CodeFriend pattern for launching a small
static website quickly and safely:

- private S3 bucket as the asset origin
- CloudFront as the public HTTPS delivery layer
- CloudFront Origin Access Control for S3 access
- ACM certificate in `us-east-1`
- Route 53 aliases for the domain or subdomain
- Terraform for infrastructure mutation
- separate deploy script for S3 sync and CloudFront invalidation
- focused verification against the real public proof surface

Use this pattern when the site is static, the public claim boundary is narrow,
and the goal is to stand up a credible HTTPS surface without application
runtime, forms, analytics, signup, authentication, or customer data.

## Source Pattern

This guide is extracted from the CodeFriend pre-alpha site launch:

- `docs/milestones/v0.91.4/CODEFRIEND_RAPID_WEBSITE_LAUNCH_DEMO.md`
- `docs/planning/codefriend/CODEFRIEND_PRE_ALPHA_REPO_AND_S3_WELCOME_MINI_SPRINT.md`
- `docs/milestones/v0.91.4/CODEFRIEND_AWS_PUBLICATION.md`
- `docs/milestones/v0.91.4/CODEFRIEND_PUBLICATION_HANDOFF.md`

The important lesson from that launch is that the static-site path is fast only
when the architecture is already known and the scope stays small. Do not use
this guide as a shortcut for application runtime work.

## Inputs

Before creating resources, decide and record:

| Input | Example | Notes |
| --- | --- | --- |
| Site name | `strategic-cognitive-reserve` | Human-readable project/site name. |
| Domain or subdomain | `scr.agent-logic.ai` | The requested public hostname. |
| Parent hosted zone | `agent-logic.ai` | Must exist in Route 53 or be explicitly delegated. |
| AWS region for S3 | `us-west-2` | Default used by the CodeFriend pattern. |
| ACM region for CloudFront | `us-east-1` | Required for CloudFront alternate-domain HTTPS. |
| Repository owner/name | `agent-logic/strategic-cognitive-reserve` | Product/site repo, usually private at first. |
| Deployment actor | GitHub Actions or operator CLI | Prefer OIDC/deploy role over long-lived keys. |
| Public claim boundary | `coming soon`, `project home`, etc. | Must be approved before publication. |

For the Strategic Cognitive Reserve project, the intended hostname is
`scr.agent-logic.ai`. The parent domain is `agent-logic.ai`.

## Apex Domain, Subdomain, And `www`

The Terraform shape is the same for apex domains and subdomains, but the DNS and
certificate inputs differ.

For a single subdomain such as `scr.agent-logic.ai`:

- hosted zone: `agent-logic.ai`
- CloudFront alternate domain: `scr.agent-logic.ai`
- ACM certificate SANs: `scr.agent-logic.ai`
- Route 53 alias records: one record for `scr.agent-logic.ai`
- typical redirect/canonical decision: not required unless the project also
  wants another hostname

For an apex site such as `example.ai`:

- hosted zone: `example.ai`
- CloudFront alternate domains usually include both `example.ai` and
  `www.example.ai`
- ACM certificate SANs usually include both `example.ai` and `www.example.ai`
- Route 53 alias records usually include both apex and `www`
- choose and document which hostname is canonical, and whether the other
  hostname redirects or serves the same static content

For a delegated subdomain zone such as `project.example.ai`:

- either create records in the parent zone `example.ai`, or delegate
  `project.example.ai` to its own hosted zone and record the NS delegation
- do not assume delegation exists just because the parent domain exists
- verify DNS resolution before treating publication as live

The CodeFriend source pattern used an apex-plus-`www` launch. The Strategic
Cognitive Reserve setup uses the simpler single-subdomain path:
`scr.agent-logic.ai` inside the existing `agent-logic.ai` hosted zone.

## Repository Shape

Use a small, public-ready shape even when the repository starts private:

```text
README.md
docs/
  DEPLOYMENT.md
  PUBLICATION_BOUNDARY.md
  ROLLBACK.md
site/
  index.html
  styles.css
  assets/
infra/
  terraform/
    versions.tf
    providers.tf
    variables.tf
    locals.tf
    s3.tf
    cloudfront.tf
    acm.tf
    route53.tf
    iam_deploy_role.tf
    outputs.tf
scripts/
  deploy_site.sh
  verify_site.sh
  rollback_site.sh
```

Keep implementation changes in the product/site repository. Keep ADL lifecycle,
review, and closeout records in ADL unless the project repo has its own adapter
and project-space records.

## Architecture

The standard shape is:

```text
viewer
  -> Route 53 alias for domain/subdomain
  -> CloudFront distribution
  -> CloudFront Origin Access Control
  -> private S3 origin bucket
```

Rules:

- S3 is the private origin and should not be the public proof surface.
- CloudFront is the public delivery layer.
- The ACM certificate used by CloudFront alternate domains must live in
  `us-east-1`.
- Route 53 should alias the domain or subdomain to the CloudFront distribution.
- The first public proof surface is the CloudFront distribution URL or the
  custom HTTPS domain after DNS is ready.
- Do not route reviewers to a direct S3 website endpoint as the launch proof.

## Terraform Resource Checklist

Create or adapt Terraform for these resources.

### Providers And Regions

Use one default AWS provider for regional resources such as S3, plus a
`us-east-1` provider alias for the ACM certificate:

```hcl
provider "aws" {
  region = var.aws_region
}

provider "aws" {
  alias  = "use1"
  region = "us-east-1"
}
```

Recommended variables:

```hcl
variable "aws_region" {
  type    = string
  default = "us-west-2"
}

variable "domain_name" {
  type = string
}

variable "hosted_zone_name" {
  type = string
}

variable "site_bucket_name" {
  type = string
}

variable "github_repository" {
  type = string
}
```

For `scr.agent-logic.ai`, set:

```hcl
domain_name      = "scr.agent-logic.ai"
hosted_zone_name = "agent-logic.ai"
```

### Private S3 Origin

Use a private bucket, block public access, and let CloudFront read through OAC.

Expected controls:

- bucket public access block enabled
- object ownership configured
- no public website endpoint as the primary publication path
- bucket policy grants read only to the CloudFront distribution/OAC path

### CloudFront Distribution

Configure:

- S3 origin
- Origin Access Control
- HTTPS viewer certificate from ACM
- alternate domain name matching the requested domain/subdomain
- default root object such as `index.html`
- sensible default cache behavior for static assets
- custom error response if single-page app routing is needed

For a simple static page, avoid SPA routing unless the site actually needs it.

### ACM Certificate

Request or reference the certificate in `us-east-1`:

```hcl
resource "aws_acm_certificate" "site" {
  provider          = aws.use1
  domain_name       = var.domain_name
  validation_method = "DNS"

  lifecycle {
    create_before_destroy = true
  }
}
```

Create Route 53 validation records and wait for validation before relying on
the distribution.

### Route 53 Alias

Use the existing hosted zone for the parent domain:

```hcl
data "aws_route53_zone" "parent" {
  name         = var.hosted_zone_name
  private_zone = false
}
```

Create an alias record for the requested domain/subdomain pointing at the
CloudFront distribution.

For `scr.agent-logic.ai`, the record name is `scr.agent-logic.ai` in the
`agent-logic.ai` hosted zone.

For an apex-plus-`www` site, create both alias records and include both names in
the CloudFront alternate-domain and ACM certificate configuration. Record the
canonical-hostname decision before publication so verification knows whether to
expect redirects or equivalent content on both hostnames.

### GitHub Deploy Role

Prefer GitHub OIDC and a narrow deploy role over static AWS keys.

Before creating an OIDC provider, check whether the AWS account already has the
shared GitHub Actions OIDC provider. In the CodeFriend launch, the only real
infrastructure defect was trying to create a duplicate provider that already
existed.

Recommended posture:

- read/reuse an existing account-level GitHub OIDC provider when present
- create a narrow role scoped to this repository and deployment path
- grant only the permissions needed for S3 sync and CloudFront invalidation
- do not commit long-lived AWS keys

## Deployment Script Responsibilities

Keep infrastructure mutation and content deployment separate.

Terraform should own:

- S3 bucket
- CloudFront distribution
- ACM certificate
- Route 53 records
- IAM deploy role

`scripts/deploy_site.sh` should own:

- local preflight checks
- sync `site/` to the S3 origin bucket
- remove stale deleted files when appropriate
- request CloudFront invalidation

Typical deploy command shape:

```sh
aws s3 sync site/ "s3://${SITE_BUCKET}/" --delete
aws cloudfront create-invalidation \
  --distribution-id "${CLOUDFRONT_DISTRIBUTION_ID}" \
  --paths "/*"
```

Read configuration from environment variables or checked-in non-secret Terraform
outputs, not from local credential paths.

Recommended environment variables:

```text
AWS_PROFILE
AWS_REGION
SITE_BUCKET
SITE_DOMAIN
CLOUDFRONT_DISTRIBUTION_ID
```

Project-specific scripts may prefix these names, for example
`SCR_SITE_BUCKET` or `SCR_CLOUDFRONT_DISTRIBUTION_ID`.

## Verification

Run infrastructure validation before mutation:

```sh
terraform -chdir=infra/terraform init
terraform -chdir=infra/terraform fmt -check
terraform -chdir=infra/terraform validate
terraform -chdir=infra/terraform plan -out=tfplan
```

After approved apply and deploy:

```sh
terraform -chdir=infra/terraform apply -auto-approve tfplan
scripts/deploy_site.sh
scripts/verify_site.sh
```

Verify the real public proof surface:

```sh
curl -I "https://${SITE_DOMAIN}"
curl -s "https://${SITE_DOMAIN}" | head
```

For `scr.agent-logic.ai`:

```sh
curl -I https://scr.agent-logic.ai
curl -s https://scr.agent-logic.ai | head
```

Acceptance checks:

- HTTPS returns `200` after DNS and CloudFront propagation.
- The response body contains the approved page copy.
- Route 53 record points to CloudFront, not raw S3.
- S3 bucket remains private.
- CloudFront invalidation completed or is explicitly in progress.
- No secrets, local paths, account IDs, or unsupported claims appear in docs or
  public content.

## Rollback

For content rollback, keep a known-good artifact or commit and re-sync it to the
origin bucket, then invalidate CloudFront.

For infrastructure rollback:

- prefer reverting the Terraform change and applying the reviewed plan
- do not manually mutate CloudFront/S3/Route 53 unless recording emergency
  operator action
- record any public outage or DNS propagation caveat in the handoff

## Publication Boundary

Before publication, write down what the site is allowed to claim.

Allowed for a small project landing page:

- project name
- owner/company name
- coming-soon or project-home posture
- non-sensitive contact or placeholder copy if approved

Avoid unless separately reviewed:

- product availability
- customer promises
- benchmarks
- security/compliance certifications
- medical, financial, legal, or investment claims
- analytics, forms, tracking, signup, or customer-data collection

For Strategic Cognitive Reserve, do not publish cognitive, medical,
performance, financial, or investment claims from the static site without a
separate claim-boundary review.

## Handoff Record

After launch or deployment-ready setup, record:

- repository URL
- commit used for the site content
- Terraform workspace/backend location, without secrets
- S3 origin bucket name if safe for the project record
- CloudFront distribution ID/domain in the private infrastructure record
- public URL
- verification commands and results
- rollback command
- known non-claims
- remaining work before product or project launch

## Strategic Cognitive Reserve Website Note

For `scr.agent-logic.ai`:

- use `scr.agent-logic.ai` as `domain_name`
- use `agent-logic.ai` as `hosted_zone_name`
- use a private S3 origin and CloudFront OAC
- request/validate the CloudFront certificate in `us-east-1`
- create a Route 53 alias for `scr.agent-logic.ai`
- deploy only approved static content
- verify `https://scr.agent-logic.ai`
- keep project claims narrow until the separate Strategic Cognitive Reserve
  project setup issue records its own reviewed publication boundary

Private repository creation, ADL adapter installation, and project-space
migration are owned by the separate Strategic Cognitive Reserve setup issue, not
by this reusable website guide.

## Fast Path Checklist

1. Confirm domain/subdomain and parent hosted zone.
2. Confirm AWS profile/account and shared OIDC provider posture.
3. Create private product/site repo.
4. Add `site/` content and publication-boundary docs.
5. Add Terraform static-site scaffold.
6. Run `terraform fmt`, `validate`, and `plan`.
7. Review plan for S3 privacy, CloudFront OAC, ACM `us-east-1`, and Route 53
   alias correctness.
8. Apply Terraform after approval.
9. Sync `site/` to S3.
10. Invalidate CloudFront.
11. Verify CloudFront/custom-domain HTTPS and page body.
12. Record handoff and rollback truth.

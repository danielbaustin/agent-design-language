# CodeFriend Rapid Website Launch Demo

Date: `2026-05-27`

Issue: `#3423`

## Purpose

Capture the completed CodeFriend pre-alpha site mini-sprint as a reusable,
truthful pattern for launching a small static website quickly and reliably.

This is a process/demo artifact, not a product-marketing claim.

## Repo model used in this demo

This was a two-repo workflow.

- `agent-design-language` owned the sprint, issue lifecycle, milestone notes,
  and closeout truth
- `agent-logic/codefriend.ai` owned the actual product/site implementation,
  Terraform, publication scripts, and live website surface

That distinction matters for reuse:

- implementation changes belonged in the product/site repo
- lifecycle records and proof packets belonged in ADL

This demo is reusable, but it is not a single-repo launch pattern by default.

## What this demo proves

This demo proves that we can take a bounded site idea from planning to a live
HTTPS landing page quickly when all of the following are true:

- the public surface is intentionally small
- the infrastructure shape is already known
- the claim boundary is narrow and explicit
- Terraform owns infrastructure mutation
- content deployment is kept separate from infrastructure mutation
- verification is focused on the real public proof surface

In the CodeFriend case, the result was:

- a new private product/site repository
- a reviewed coming-soon page
- Terraform-managed AWS static-site infrastructure
- live HTTPS delivery on:
  - `https://codefriend.ai`
  - `https://www.codefriend.ai`
- deployment, verification, rollback, and publication-handoff documentation

## What this demo does not prove

This demo does not prove:

- that the underlying product is ready
- that application runtime work can be done as quickly as static-site work
- that every future website launch will take the same amount of time
- that a benchmark or marketing outcome has been achieved

The CodeFriend sprint created a real live coming-soon surface, not a CodeFriend
alpha product.

## Demo outcome summary

Elapsed time for the substantive mini-sprint execution was about `23m4s`.

Why that was possible:

- the scope stayed narrow
- the copy was already approved
- AWS and DNS access were already available
- we reused a proven Terraform/site pattern from `agent-logic.ai`
- we only had one real infrastructure defect to fix

## Ordered execution sequence

### 1. Bootstrap the product/site repository

Issue: `#3373`

Outcome:

- created private repo `agent-logic/codefriend.ai`
- seeded a narrow scaffold for:
  - `README.md`
  - deployment/security/source-map docs
  - `site/` static assets
  - `infra/terraform/` static-site infrastructure

Key principle:

- start with a public-ready repository shape even if the repository itself is
  still private

### 2. Build the minimal welcome page

Issue: `#3374`

Outcome:

- refined the static page to the approved copy:
  - `CodeFriend`
  - `because your code needs a friend.`
  - `Coming soon from Agent Logic, Inc.`
- kept it intentionally small:
  - no forms
  - no analytics
  - no signup hooks
  - no product-availability claims

Key principle:

- finish the page before the live publication step so infrastructure work is
  validating a real surface, not a placeholder-to-be-reworked later

### 3. Provision the infrastructure and publish

Issue: `#3375`

Outcome:

- initialized and applied Terraform for:
  - private S3 origin bucket
  - CloudFront distribution with OAC
  - ACM certificate in `us-east-1`
  - Route 53 apex and `www` alias records
  - narrow GitHub deploy role
- published the static site to the private origin bucket
- invalidated CloudFront
- verified live HTTPS delivery

Key principle:

- keep infrastructure mutation and content deployment separate

### 4. Record publication safety and handoff

Issue: `#3376`

Outcome:

- confirmed the live surface stayed within the approved claim boundary
- verified no secrets, local credential paths, unsupported claims, forms, or
  analytics slipped into the publication packet
- wrote the handoff for later alpha work

Key principle:

- a live site is not the same thing as a product launch; the handoff must keep
  that distinction explicit

### 5. Close out the mini-sprint truthfully

Issue: `#3372`

Outcome:

- recorded the sidecar as complete
- preserved the correct boundary:
  - real live static site: yes
  - CodeFriend alpha product: not yet

## What we reused from `agent-logic.ai`

The biggest accelerator was reusing a pattern that had already worked.

Reusable pieces:

- Terraform file split and resource model
- private S3 origin + CloudFront OAC pattern
- ACM in `us-east-1` for CloudFront custom-domain TLS
- Route 53 alias-record pattern
- narrow GitHub OIDC deploy-role scope
- separation of duties:
  - Terraform owns infra mutation
  - deploy path owns S3 sync + invalidation only
- staged/coming-soon claim posture

This reuse mattered because it removed design uncertainty. We were adapting a
known-good shape, not inventing a new infrastructure model under time pressure.

## What broke and how we fixed it

The main real defect was account-level OIDC-provider duplication.

Problem:

- the initial CodeFriend Terraform scaffold tried to create a GitHub Actions
  OIDC provider
- the target AWS account already had the shared GitHub OIDC provider in place

Why that matters:

- trying to create a duplicate provider would have turned a healthy launch path
  into a preventable infrastructure error

Fix:

- changed Terraform to read and reuse the existing GitHub OIDC provider instead
  of creating a new one

Lesson:

- for repeatable static-site launches in the same AWS account, shared account
  primitives should be treated as reusable dependencies, not always-created
  resources

## Proof and validation sequence that was enough

This mini-sprint went well partly because the proof surface stayed tight.

### Docs-only ADL issues

For the ADL-side writeups and handoff issues, the smallest useful proof was:

- `git diff --check`
- focused text/hygiene scans
- truthful milestone notes and SOR updates

We did not need broad code or runtime test cycles for those docs-only issue
surfaces.

### Real infrastructure issue

For the actual publication issue, the useful proof was:

- `terraform init`
- `terraform fmt`
- `terraform validate`
- `terraform plan -out=tfplan`
- `terraform apply -auto-approve tfplan`
- `scripts/deploy_site.sh`
- `scripts/verify_site.sh`
- direct `curl` checks on:
  - the CloudFront distribution domain
  - `https://codefriend.ai`
  - `https://www.codefriend.ai`
- Route 53 record inspection

That was enough because it verified the actual public proof surface instead of
stopping at “Terraform plan looks reasonable.”

## Why this launch was fast and reliable

This launch was fast because we avoided common sources of drag:

- no app runtime
- no forms or customer state
- no unclear product messaging
- no new infrastructure architecture exploration
- no hidden dependency on marketing, compliance, or analytics work
- no broad validation expansion for docs-only steps

It was reliable because:

- the claim boundary was explicit
- the infrastructure pattern was already proven elsewhere
- we separated infra mutation from content publication
- we used the real public proof surface for verification
- we corrected the one real account-level defect immediately

## Reusable checklist for future static-site launches

### Preconditions

- choose a narrow public claim boundary
- confirm the surface is static-only
- confirm the target domain and hosted zone already exist
- confirm AWS access is available
- confirm the site copy is approved before live publication work starts
- confirm whether the account already has shared GitHub OIDC primitives

### Repository bootstrap

- create the private product/site repo
- add a public-ready scaffold
- seed `site/`
- seed Terraform split
- seed deployment/security/source-map docs

### Page prep

- finish the actual landing page copy and styling first
- keep the page free of forms, analytics, and overclaims

### Infrastructure

- use Terraform for infra mutation only
- provision private S3 origin
- provision CloudFront + OAC
- provision ACM in `us-east-1`
- provision Route 53 apex + `www`
- provision the narrow deploy role
- decide whether shared account primitives must be created or reused before the
  first `terraform plan`
- explicitly check whether a shared GitHub Actions OIDC provider already exists
  in the target AWS account before trying to create one

### Publication

- sync `site/` to the origin bucket
- invalidate CloudFront
- verify the CloudFront domain
- verify apex and `www` over HTTPS
- inspect returned content, not just status codes

### Handoff

- write a publication-boundary document
- write deployment/rollback/verification notes
- record what is live
- record what is still outside scope

## Future multiagent split points

This specific sprint was already fast, but it could go faster in a future
multiagent mode.

Good split points:

### Agent 1: repo scaffold and docs boundary

Owns:

- repo bootstrap
- README/deployment/security/source-map docs
- claim-boundary framing

### Agent 2: landing page implementation

Owns:

- `site/index.html`
- `site/styles.css`
- local preview and content checks

### Agent 3: Terraform adaptation

Owns:

- adapting the known-good Terraform pattern
- account-primitive checks
- plan/apply readiness

### Agent 4: publication and verification

Owns:

- content sync
- invalidation
- live URL checks
- header/body verification

### Agent 5: handoff and closeout

Owns:

- publication handoff
- milestone note consolidation
- sprint closeout packet

Why this would help:

- the page work and Terraform adaptation are only lightly coupled
- verification and handoff can start as soon as live publication evidence
  exists
- the main conductor would only need to reconcile one live path and one docs
  path rather than doing every step serially

## Recommendation

Treat this as a reusable “rapid static-site launch” demo pattern, but only for
bounded sites with these properties:

- static content
- narrow claims
- known infrastructure pattern
- available AWS/DNS access
- no customer-data or runtime complexity

That keeps the demo honest and makes it genuinely reusable.

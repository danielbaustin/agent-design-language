# CodeFriend Pre-Alpha Repo And S3 Welcome Mini-Sprint

## Status

Planning document for issue `#3254`.

This plan does not create the CodeFriend repository, touch AWS, publish DNS, or
claim CodeFriend alpha readiness. It defines a bounded pre-alpha mini-sprint
that can be executed before the full `v0.93.x` CodeFriend alpha milestone.

## Relationship To Existing CodeFriend Plan

The tracked CodeFriend setup plan already says CodeFriend alpha likely belongs
in a dedicated `v0.93.x` product milestone, with a working alpha ready for
testing as the exit bar. This mini-sprint is smaller than that alpha milestone.

It pulls forward one safe setup lane:

- establish the product repository
- establish the AWS S3-backed static-site machinery
- add the CloudFront distribution, ACM certificate, and Route 53 DNS setup
- publish or prepare a minimal welcome page for `CodeFriend.ai`
- keep the claim boundary clear: "coming soon", not "product available"

This fits the existing setup-plan sequence because it addresses the product
repo / site decision and public-facing landing-page brief before the full alpha
package is seeded.

## Goal

Create the first operational CodeFriend product surface:

> CodeFriend - because your code needs a friend. Coming soon from Agent Logic,
> Inc.

The result should be small, polished, reversible, and auditable. It should make
the product real enough to point at without pretending the alpha exists yet.

## Operating Principle

This is pre-alpha product work, but it is not throwaway infrastructure.

The first CodeFriend site should be treated as a real product surface while
staying deliberately small:

- S3 stores the landing page assets.
- CloudFront is the public delivery layer.
- ACM provides the public HTTPS certificate.
- Route 53 provides DNS for the already-owned domain.
- No application runtime, forms, analytics, signup machinery, or customer data
  is required for this slice.
- Deployment, rollback, and verification steps are documented.
- Public copy is reviewed before launch.
- Deployment records are auditable and free of secrets or local paths.

The page may say only "coming soon", but the operational substrate should be
credible enough to keep using as CodeFriend grows.

## Mini-Sprint Boundary

### In Scope

- Create a new private GitHub repository for CodeFriend product/site work.
- Add a minimal repo scaffold with README, license posture, contribution note,
  site source, deployment docs, and no ADL runtime dependency.
- Add a static welcome page with the approved coming-soon copy.
- Add AWS S3 static-site asset hosting machinery in `us-west-2`.
- Add CloudFront distribution planning and deployment machinery.
- Add ACM certificate planning for CloudFront HTTPS, with certificate material
  in `us-east-1`.
- Add Route 53 DNS planning for `codefriend.ai` and `www.codefriend.ai`.
- Add deployment and rollback runbooks.
- Add review and publication-safety checks before any public exposure.
- Record provenance back to ADL planning docs and ADR 0025.

### Out Of Scope

- Building the CodeFriend alpha product application.
- Migrating ADL review skills into the CodeFriend repo.
- Renaming all historical `CodeBuddy` / `codebuddy` surfaces.
- Creating customer claims, pricing, signup flows, or product availability
  promises.
- Depending on GWS or C-SDLC as canonical CodeFriend infrastructure.
- Publishing secrets, AWS account IDs, private local paths, or operator-specific
  credential paths.
- Broad marketing site design beyond the minimal welcome page.
- Building AWS resources beyond the static-site path: no application runtime,
  database, forms, analytics, signup machinery, queues, or customer-data
  systems.

## Recommended Repository Shape

Candidate repository name:

- `danielbaustin/codefriend`

Alternative if the owner namespace should be product/company-first later:

- `agentlogic/codefriend`

Initial visibility:

- private

Public-readiness posture:

- write every doc as if the repo may eventually become public
- keep secrets and local credential paths out of the repo
- use repo-relative paths in docs
- keep license posture explicit

Recommended first tree:

```text
README.md
LICENSE.md
docs/
  DEPLOYMENT.md
  SECURITY_AND_PUBLICATION_BOUNDARY.md
  SOURCE_MAP.md
site/
  index.html
  styles.css
  assets/
infra/
  aws/
    README.md
    s3-origin-bucket-policy.template.json
    cloudfront-distribution-notes.md
    acm-certificate-notes.md
    route53-records.md
scripts/
  deploy_s3.sh
  invalidate_cloudfront.sh
  verify_site.sh
```

## Welcome Page Copy

Primary page copy:

```text
CodeFriend
Because your code needs a friend.
Coming soon from Agent Logic, Inc.
```

Tone:

- calm
- credible
- warm
- minimal
- no hype

Non-claims:

- do not say the product is available
- do not say it performs autonomous code review
- do not claim SOC, compliance, security certification, or customer readiness
- do not imply CodeFriend replaces human engineering judgment

## AWS Static Site Shape

First-pass hosting shape:

- An S3 bucket in `us-west-2` stores the static welcome page assets.
- CloudFront serves the public site over HTTPS.
- CloudFront uses the S3 bucket as the static asset origin.
- ACM provides the CloudFront viewer certificate.
- The CloudFront viewer certificate is requested or imported in `us-east-1`
  because CloudFront requires ACM certificates for alternate-domain HTTPS in
  AWS US East / N. Virginia.
- Route 53 owns the DNS records for `codefriend.ai` and `www.codefriend.ai`.
- Route 53 aliases the domain records to the CloudFront distribution.
- No application runtime, database, form handler, analytics service, signup
  machinery, customer workflow, or customer data is created in this
  mini-sprint.
- The page contains no forms, scripts, analytics, customer data, or sensitive
  content.

This is acceptable for a pre-alpha coming-soon page because there is no dynamic
application, no authentication, no user data, and no customer workflow.

Important first-pass requirement:

- The first public proof surface should be the CloudFront HTTPS URL or the
  custom domain after Route 53 cutover.
- The S3 bucket is the asset origin; it is not the final public proof surface.

Fallback only:

- If DNS is not live yet, record the CloudFront distribution URL or an explicit
  "not publicly reachable yet" blocker.
- Do not route reviewers to a direct S3 website endpoint as the launch proof.

Recommended environment variables for deployment scripts:

```text
CODEFRIEND_AWS_PROFILE
CODEFRIEND_AWS_REGION
CODEFRIEND_SITE_BUCKET
CODEFRIEND_SITE_DOMAIN
CODEFRIEND_WWW_DOMAIN
CODEFRIEND_ACM_CERTIFICATE_ARN
CODEFRIEND_CLOUDFRONT_DISTRIBUTION_ID
```

These names let the repo avoid checked-in credential paths or local machine
assumptions.

## Proposed Mini-Sprint Work Packages

### CF-PRE-01: Product Repo Bootstrap

Outcome:

- private CodeFriend repo exists
- README explains the pre-alpha boundary
- source map points back to ADL CodeFriend planning and ADR 0025
- license posture is explicit

Acceptance:

- repo has no secrets or local absolute paths
- repo can be cloned cleanly
- docs say `CodeFriend` / `CodeFriend.ai`, not `CodeBuddy`
- README does not overclaim product readiness

Validation:

- `git status --short`
- Markdown link check for repo-local docs
- secret/path scan for absolute home-directory paths, AWS keys, private IPs,
  and local credential paths

### CF-PRE-02: Static Welcome Page

Outcome:

- `site/index.html` and `site/styles.css` render the welcome page
- page copy matches the approved pre-alpha message
- page works locally from a file or simple static server

Acceptance:

- page displays:
  `CodeFriend - because your code needs a friend. Coming soon from Agent Logic, Inc.`
- no analytics, trackers, form capture, or external scripts
- responsive layout works on desktop and mobile
- no product-availability claims

Validation:

- local static render check
- HTML/CSS lint if lightweight tooling is added
- screenshot or browser check before deployment

### CF-PRE-03: AWS S3, CloudFront, ACM, And Route 53 Machinery

Outcome:

- deployment docs and scripts describe how to publish the static page
- S3 static asset bucket setup is represented in docs or lightweight templates
- CloudFront distribution setup is represented in docs or lightweight templates
- ACM certificate requirements are documented
- ACM certificate region requirements for CloudFront are documented
- Route 53 records for `codefriend.ai` and `www.codefriend.ai` are documented
- `us-west-2` is the regional default for S3
- deployment remains credential-path-free and operator-configurable

Acceptance:

- deploy script uses environment variables or AWS profile names only
- no secrets or account-specific credentials are committed
- rollback command is documented
- CloudFront invalidation command is documented
- Route 53 record values are documented before DNS mutation
- S3 bucket is the static asset origin
- CloudFront is the public access path
- CloudFront alternate-domain HTTPS uses an ACM certificate in `us-east-1`
- no application runtime, analytics, forms, signup resources, or customer-data
  systems are created

Validation:

- shell syntax check for scripts
- dry-run or no-op validation where available
- AWS identity/account check before any live mutation
- AWS region check, defaulting S3 to `us-west-2`
- deployment log captured outside secrets
- S3 bucket existence/configuration check
- ACM certificate status check
- ACM certificate region check for CloudFront HTTPS
- CloudFront distribution status check
- CloudFront invalidation check after deployment
- Route 53 DNS resolution check after public DNS is changed

### CF-PRE-04: Publication Safety And Handoff

Outcome:

- review packet records what was deployed, where, and what it claims
- DNS/public exposure is either complete or explicitly pending
- handoff records next alpha-milestone tasks

Acceptance:

- public-facing copy passes redaction and claim-boundary review
- final URL is recorded if live
- if DNS is not live, the CloudFront distribution URL or a clear "not publicly
  reachable yet" blocker is recorded
- next steps route to the full CodeFriend alpha plan, not hidden scope

Validation:

- URL check if live
- `curl -I` or equivalent header check if live
- browser screenshot if live
- final leakage scan

## Suggested Execution Order

1. Confirm repo owner/name and license posture.
2. Create the private repo.
3. Add the minimal repo scaffold.
4. Add welcome page source.
5. Add S3 origin deployment docs and scripts.
6. Add CloudFront distribution and ACM setup docs.
7. Add Route 53 DNS setup docs and record plan.
8. Review copy, redaction, and claim boundary.
9. Deploy assets to S3 or stop at deployment-ready if AWS approval is pending.
10. Create or update the CloudFront distribution and invalidate the cache.
11. Point Route 53 to CloudFront if public exposure is approved.
12. Verify the live or staged HTTPS page.
13. Record the handoff and next alpha tasks.

## Review Gates

Before repository creation:

- confirm repo owner and name
- confirm license posture
- confirm repo remains private

Before AWS mutation:

- confirm AWS account/profile
- confirm S3 region is `us-west-2`
- confirm bucket name
- confirm ACM certificate request/validation path
- confirm CloudFront viewer certificate is in `us-east-1`
- confirm CloudFront distribution naming and aliases
- confirm Route 53 hosted zone and intended records

Before public exposure:

- review page copy
- scan for secrets and private paths
- confirm no unsupported product claims
- confirm Agent Logic, Inc. naming is acceptable
- confirm CloudFront is serving the page over HTTPS
- confirm Route 53 aliases target CloudFront

## Validation Plan

Minimum validation for the mini-sprint:

- `git status --short`
- Markdown link check
- shell syntax check for deployment scripts
- local render check for `site/index.html`
- secret/path scan
- AWS CLI identity check before live deployment
- AWS S3 region check for `us-west-2`
- S3 bucket configuration check
- CloudFront distribution status check
- ACM certificate status check
- ACM `us-east-1` certificate check for CloudFront alternate-domain HTTPS
- Route 53 hosted-zone and record check before DNS mutation
- HTTPS live URL check only after deployment is intentionally approved

Do not run broad ADL test suites for this mini-sprint. This is product repo and
static-site setup work, not ADL runtime work.

## Open Decisions

- Final repository owner: `danielbaustin` vs company/product organization.
- Final repository name: `codefriend`, `codefriend-ai`, or another approved
  name.
- License posture: private all-rights-reserved draft vs open-source license.
- AWS account.
- S3 bucket name.
- ACM certificate ARN or certificate-request path, with CloudFront viewer
  certificates kept in `us-east-1`.
- CloudFront distribution ID.
- Route 53 hosted zone and exact DNS records.
- Whether `codefriend.ai`, `www.codefriend.ai`, or both are activated in the
  first mini-sprint.

## Exit Bar

The mini-sprint is complete when:

- CodeFriend has a private product/site repo.
- The repo contains a minimal, reviewable static welcome page.
- AWS S3, CloudFront, ACM, and Route 53 deployment machinery is documented and
  tested or ready to execute.
- If deployment is approved, the welcome page is live over HTTPS and verified.
- The public claim boundary remains "coming soon".
- The handoff points back to the full CodeFriend alpha milestone plan.

## Handoff To Full Alpha

After this mini-sprint, the full CodeFriend alpha milestone should own:

- review-packet runner packaging
- specialist lane packaging
- product-report polish
- sample repo / sample packet / sample report
- redaction and publication-safety gates
- architecture-cognition first slice
- alpha demo and testing workflow

This mini-sprint should make the product home real. It should not pretend the
product alpha has already arrived.

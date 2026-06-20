# Agent Logic AWS Account Decision Record

## Metadata

- Issue: `#3902`
- Milestone: `v0.91.6`
- Account name: `Agent Logic Primary`
- Status: complete; AWS Activate review pending as an external follow-up
- Record date: 2026-06-19

## Scope

`#3902` establishes the first company-owned AWS account foundation for Agent
Logic. It records the account identity, administrative access posture, billing
guardrails, AWS Activate application state, and Terraform/bootstrap boundaries.

This record intentionally excludes AWS account IDs, AWS Activate offer
identifiers, recovery codes, credentials, billing screenshots, credit balances,
and private application details.

## Current State

The account setup work is operationally complete. AWS Activate review is an
external follow-up that may take several days and does not block closing
`#3902`.

## Decisions

### D-01 Root Identity

- Root email: company-controlled AWS root alias selected
- Billing contact: company billing contact selected
- Recovery contact: company recovery contact selected
- Root account authentication: passkey and authenticator application configured
- Passkey custody: Apple iCloud Keychain
- Root account policy: emergency use only; not used for daily administration

The exact root, billing, and recovery contact values are treated as private
account-custody details and are not recorded in this tracked milestone packet.

### D-02 Account Architecture

- Current architecture: standalone company-owned AWS account
- Future direction: compatible with later AWS Organizations, IAM Identity
  Center, centralized billing, and multi-account enterprise structure
- Current boundary: no AWS Organizations rollout under `#3902`

### D-03 Administrative Access

- Administrative user: daily-use administrative identity created
- Purpose: daily AWS administration
- Permissions: `AdministratorAccess`
- Authentication: passkey and authenticator application configured
- Usage policy: normal AWS administration uses the administrative user; the
  root account remains emergency-only

The exact administrative username is treated as private account-custody detail
and is not recorded in this tracked milestone packet.

### D-04 AWS Activate

- Application status: submitted
- Source: Clerky AWS Activate offer
- Review status: pending AWS review
- Expected response window: 5-7 business days from submission
- Repository rule: no AWS Activate identifiers, offer codes, credit balances,
  screenshots, or private application details are stored in repository
  artifacts

### D-05 Billing Foundation

- Monthly budget: configured
- Budget amount: operator-approved monthly budget configured
- Budget alert thresholds: tiered budget alerts configured
- Cost controls configured: Cost Anomaly Detection and budget notifications
- Governance rule: monthly billing review is required
- Credit tracking rule: credits, balances, applicable services, and expiration
  dates are tracked privately and are not committed to GitHub

The exact budget name, alert recipients, and private credit values are treated
as private billing-governance details and are not recorded in this tracked
milestone packet.

### D-06 Terraform Bootstrap Boundary

`#3902` is planning-only for Terraform. No Terraform execution occurs under this
issue, and no infrastructure resources are deployed under this issue.

Future Terraform topics are routed to follow-on work:

- remote state
- state locking
- KMS integration
- IAM baseline
- logging baseline

## Completed Security Foundation

- Root MFA enabled
- Root passkey enabled
- Administrative MFA enabled
- Administrative passkey enabled
- Administrative access validated
- Root and administrative accounts separated

## Explicit Non-Goals Confirmed

- No workloads deployed
- No DNS migration
- No Route 53 migration
- No Strategic Cognitive Reserve deployment
- No production infrastructure
- No AWS Organizations rollout

`#3902` stops before infrastructure deployment.

## Follow-On Routing

The following work remains out of scope for `#3902` and is routed to later
tracked work before any infrastructure execution depends on it:

| Follow-on | Route | Closeout meaning for `#3902` |
| --- | --- | --- |
| Terraform bootstrap | Open a dedicated `v0.91.6` AWS bootstrap issue before any Terraform apply. | Planned only here; no state bucket, lock table, KMS, IAM, or logging resources were created. |
| AWS security baseline | Route into the AWS bootstrap/security issue wave before workloads. | Root/admin separation and MFA/passkeys are complete; workload security baselines are separate. |
| AWS logging baseline | Route into the AWS bootstrap/logging issue wave before workloads. | No CloudTrail, log archive, or account-wide logging baseline is claimed here. |
| Website hosting | Route through the existing S3 website/Terraform guide and a separate hosting issue. | No website, S3 bucket, CloudFront distribution, ACM certificate, or Route 53 record was created here. |
| Strategic Cognitive Reserve infrastructure | Route through the SCR project/repo execution path. | No SCR infrastructure was deployed under `#3902`. |
| Route 53 review | Open a separate DNS review/migration issue before DNS changes. | `agent-logic.ai` DNS remains unchanged by this issue. |
| AWS Organizations strategy | Open a later account-architecture issue when multi-account structure is ready. | The account remains standalone for now. |
| AWS Activate response | Operator-private post-close follow-up. | Approval, credit balance, applicable services, and expiration remain private and are not GitHub closure blockers. |

## Closure Criteria

`#3902` may be closed after:

- this sanitized decision record is committed; and
- issue closeout records AWS Activate approval and private credit visibility as
  a post-close external follow-up.

All other operational objectives described by the issue are satisfied.

## Post-Close External Follow-Up

AWS Activate review remains pending. When AWS responds, the operator should
verify any approved credits privately in AWS Billing and Cost Management and
keep credit balances, expiration dates, applicable services, account IDs, and
offer identifiers outside repository artifacts.

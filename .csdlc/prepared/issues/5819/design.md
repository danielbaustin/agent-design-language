# WP-02 Agent Logic Repository Migration Design

## Outcome And Authority

Issue #5819 executes the reviewed #5815 migration plan by transferring exactly
five company repositories, in order, from `danielbaustin` to `agent-logic`:

1. `cognitive-sdlc-paper`
2. `godel-hadamard-bayes-paper`
3. `general-intelligence-paper-private`
4. `universal-tool-schema`
5. `agent-design-language`

Repository names, visibility, and Git history remain unchanged. `asksifu`
remains personal and Horust is excluded. The GitHub organization owner retains
authority over transfer windows, billing, policies, and recovery decisions.

## Preconditions

Execution is blocked until WP-01B is merged, #5815/PR #5816 remains the reviewed
plan, destination owners and recovery access are confirmed, organization
billing/security policy is ready, destination names are free, and a complete
five-repository inventory is approved.

## Serial Transfer Protocol

For each repository, the implementation session must:

1. Capture a redacted before-manifest at an exact HEAD, including issue and PR
   assignees, releases, rulesets, Actions, Pages, packages, LFS, OIDC subjects,
   integrations, and collaborator/team mappings.
2. Verify every assignee is an organization member or has an explicit,
   approved reassignment plan.
3. Pre-stage current operational URL, package, OIDC, Pages, webhook, App, and
   consumer updates without exposing secret values.
4. Recheck manifest drift, execute the owner-authorized GitHub transfer, and
   record its timestamp and canonical destination URL.
5. Compare the destination with the before-manifest and repair or explicitly
   disposition every difference before the next transfer starts.

ADL transfers last. Its window includes the prepared `agent-logic.ai` link
cutover, ADL clone/remote updates, and bounded CI/publication smoke after the
destination exists.

The website cutover is owned in the separate `agent-logic/agent-logic.ai`
repository and changes exactly `site/index.html` and `site/beta/index.html`.
Those two files contain the four current `danielbaustin/agent-design-language`
links identified by the reviewed migration plan. Their publication remains an
`agent-logic.ai` deployment responsibility; this issue records the source and
deployed-link receipts after that repository's own reviewed change lands.

## Invariants And Negative Controls

- No destination is recreated and no history is rewritten.
- Secret and variable names/scopes may be recorded; values never enter evidence.
- Redirects are compatibility only, not durable configuration.
- `danielbaustin/asksifu` and Horust receive no transfer or settings mutation.
- Any unexplained manifest drift stops the wave.
- WP-02A CI redesign and downstream milestone implementation remain separate.

## Recovery

Preserve both manifests and repair in place first. Transfer-back requires an
organization-owner decision and does not count as automatic restoration of
teams, assignments, packages, Pages, secrets, or external integrations. The
complete verification gate reruns before the wave resumes.

## Proof Design

Retain gate receipts, before/after manifest digests, exact HEADs, assignee
membership/reassignment evidence, transfer observations, integration checks,
the two negative controls, final old-owner reference disposition, and one
exact-revision migration report. A typed evidence validator must fail on a
missing repository, reordered transfer, unexplained drift, leaked secret value,
absent negative control, missing destination verification for any of the five
repositories, or missing production/beta website cutover receipt. The
issue-local validator is
`.csdlc/prepared/issues/5819/validate-migration-evidence.rb`.

Each before/after manifest is retained as a separate SHA-256-bound JSON
artifact. Both manifests contain actual data for issues, pull requests,
assignees, rulesets, releases, Actions, Pages, packages, LFS, and integrations.
The validator recomputes both file digests and every canonical per-surface
digest. Differences require a digest-bound verified-disposition artifact; a
boolean preservation assertion is never sufficient. The live verifier queries
all GitHub API-backed surfaces at the destination and binds LFS to a retained
`git lfs fsck` receipt.
## Owned Paths

- `.csdlc/evidence/5819`
- `.csdlc/prepared/issues/5819/validate-migration-evidence.rb`
- `.csdlc/prepared/issues/5819/verify-live-repositories.rb`

## Read-Only Inputs

- Every repository path cited outside `## Owned Paths` is read-only unless it is repeated exactly in that section.
- Dependency records, sibling issue outputs, historical evidence, and external systems remain read-only inputs.

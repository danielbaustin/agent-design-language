# Agent Logic GitHub Repository Migration Plan

## Status

Final execution plan for v0.92 WP-01A. This document defines the migration
sequence and proof required to move five company repositories from Daniel's
personal GitHub account to the existing `agent-logic` organization.

This plan does not authorize a transfer by itself. Each transfer requires the
operator to start its named window after the preflight gate passes. Secret
values must never be copied into plans, logs, issues, chat, or evidence.

## Decision

Move these five repositories from `danielbaustin` to `agent-logic` without
renaming them or changing visibility:

| Order | Source repository | Destination repository | Reason for order |
| --- | --- | --- | --- |
| 1 | `danielbaustin/cognitive-sdlc-paper` | `agent-logic/cognitive-sdlc-paper` | Private paper with limited operational coupling |
| 2 | `danielbaustin/godel-hadamard-bayes-paper` | `agent-logic/godel-hadamard-bayes-paper` | Private paper with limited operational coupling |
| 3 | `danielbaustin/general-intelligence-paper-private` | `agent-logic/general-intelligence-paper-private` | Private paper; verify publication collaborators before transfer |
| 4 | `danielbaustin/universal-tool-schema` | `agent-logic/universal-tool-schema` | Shared schema dependency; transfer only after consumers are inventoried |
| 5 | `danielbaustin/agent-design-language` | `agent-logic/agent-design-language` | Central repository with the widest CI, package, website, and integration surface |

Explicit exclusions:

- `danielbaustin/asksifu` remains personal and must never appear in a transfer
  request.
- `danielbaustin/Horust` remains an inactive upstream-contribution fork. ADL
  forked it to propose a bounded restart fix in upstream PR `#319`; Runtime v3
  no longer uses Horust. It is not an Agent Logic product repository and is not
  migrated.

Repositories already owned by `agent-logic` are inventory and verification
surfaces only:

- `agent-logic/agent-logic.ai`
- `agent-logic/codefriend.ai`
- `agent-logic/strategic-cognitive-reserve`

## Success Criteria

The migration is complete only when:

1. all five destination repositories exist with the expected visibility,
   default branch, exact source HEAD, tags, releases, issues, pull requests,
   wiki, LFS objects, and settings;
2. required checks, Actions, environments, variables, secret names, OIDC trust,
   packages, Pages, webhooks, deploy keys, and external integrations have been
   verified or repaired;
3. durable operational references use `agent-logic/<repository>` rather than
   relying on GitHub redirects;
4. the production and beta `agent-logic.ai` pages link directly to
   `agent-logic/agent-design-language`;
5. `asksifu` remains personal and Horust remains outside the migration;
6. no unexplained inventory drift remains.

## Operating Rules

- Transfer one repository at a time. Finish its verification before beginning
  the next transfer.
- Use GitHub's repository-transfer operation. Do not recreate repositories or
  rewrite history.
- Keep repository names and visibility unchanged.
- Freeze only operations that can race the active transfer, such as release
  publication or destructive settings changes.
- Record secret and variable names and scopes only. Never export their values.
- Treat redirects as temporary compatibility, not a durable configuration.
- Treat packages, GHCR coordinates, OIDC subjects, Pages URLs, and external
  callbacks as possible hard breaks.
- Stop the migration wave on any unexplained difference. Repair or make an
  explicit operator decision before continuing.

## Gate 0: Organization Readiness

Before the first repository window:

1. Confirm `agent-logic` is the legal company destination.
2. Confirm at least two organization owners, a billing owner, and a recovery
   contact have working access and required 2FA.
3. Confirm the organization plan preserves required private-repository,
   protected-branch, Actions, Pages, package, and security features.
4. Configure repository creation, deletion, transfer, visibility, outside-
   collaborator, Actions, and private-fork policies.
5. Confirm Actions, Packages, and Git LFS storage/bandwidth budgets, payment
   methods, quotas, and alert owners.
6. Confirm the operator can create repositories in the organization and
   administer all five source repositories.
7. Confirm no destination repository or fork conflicts with any of the five
   names.
8. Confirm the five-item migrate list and the two exclusions above.

Exit condition: organization ownership, recovery, billing, policy, transfer
permissions, destination names, and exact scope are verified.

## Gate 1: Repository Manifest

Create a redacted before-manifest for the active repository window containing:

- source owner/name, visibility, default branch, exact HEAD, tags, and releases;
- open issues and pull requests, milestones, projects, discussions, wiki,
  collaborators, and outside collaborators;
- branch protections, rulesets, required checks, CODEOWNERS, environments, and
  approvals;
- workflows, schedules, runner labels, cache/artifact expectations, variables,
  secret names, and OIDC subjects;
- packages and GHCR coordinates, their permission model and repository links,
  LFS usage, Pages/custom domains, DNS ownership, webhooks, deploy keys,
  GitHub Apps, OAuth integrations, and release callbacks;
- forks, upstream network, private-fork owners, submodules, badges, import paths,
  clone URLs, and downstream consumers;
- code scanning, secret scanning, Dependabot, and other security feature state.

The manifest must identify an owner and same-window update procedure for every
critical old-owner dependency. Historical evidence may retain historical URLs;
operational configuration may not.

Exit condition: the manifest is timestamped, redacted, reviewed, and unchanged
at transfer time.

## Gate 2: Pre-Staged Consumer Changes

Before transferring the active repository:

1. Search the five migration repositories and the three existing company
   repositories for current `danielbaustin/<repository>` references.
2. Classify each reference as operational, documentation, badge, historical
   evidence, package, workflow, deployment, webhook, Pages, or clone-only.
3. Prepare reviewed changes for operational references, but do not merge a
   destination URL before that destination exists.
4. Prepare local remote updates for maintained clones and active worktrees.
5. Map source individual collaborators to destination organization members and
   teams; personal repositories have no source teams to preserve.
6. Classify each GitHub package before transfer. Repository-scoped package
   registries may transfer with the repository. Granular user/organization-
   scoped packages, including GHCR, retain their original account scope and
   lose their repository link and inherited Actions access when the repository
   transfers. For those packages, pre-stage destination publication or rebuild,
   consumer coordinate changes, repository linkage, and permissions.
7. Where the external provider permits it, pre-authorize both old and new OIDC
   subjects for the transfer window, verify the new subject after transfer, and
   remove the old subject only after consumers pass.
8. Have an organization administrator provision required organization-level
   secrets and variables through an approved secret-handling path. Record names
   and scopes only; do not export source values into migration evidence.
9. Prepare GitHub App organization approval, webhook, deploy-key, OAuth, and
   deployment callback updates for the same transfer window.
10. For any Pages site, pre-stage destination organization domain verification,
    DNS changes, expected default-URL changes, and a bounded maintenance notice.
    Custom-domain routing and TLS reprovisioning may introduce downtime.

Exit condition: every critical consumer has a prepared update, owner, test, and
rollback instruction.

## Gate 3: Transfer Window

For the active repository:

1. Recheck that source HEAD and the before-manifest have not drifted.
2. Recheck destination name and fork-network compatibility.
3. Pause only release, destructive-settings, and other transfer-racing work.
4. Transfer the repository through GitHub's owner-authorized transfer flow.
5. Verify ownership at `agent-logic/<repository>` and record the transfer time.
6. Do not create a replacement repository at the old location; doing so can
   destroy GitHub's redirect.
7. Apply the prepared critical consumer updates.
8. Update maintained local clones with the new canonical remote URL.
9. Reauthorize GitHub Apps and organization access where destination policy
   requires explicit owner approval.

The next repository may not start until Gate 4 passes.

## Gate 4: Per-Repository Verification

Compare the destination against the before-manifest and verify:

- default branch and exact HEAD;
- tags, releases, issues, pull requests, milestones, wiki, projects, stars,
  watchers, forks, and LFS objects;
- visibility, rulesets, branch protections, required checks, CODEOWNERS, teams,
  collaborators, and outside access;
- workflow enablement plus a bounded push or pull-request smoke appropriate to
  the repository;
- environments, approvals, runner labels, variable names, secret names, and
  updated OIDC trust;
- repository-scoped package transfer state and granular package account scope,
  repository links, permissions, destination publication, and consumer
  coordinates;
- asynchronous LFS object completion plus destination quota and billing state;
- Pages URL, custom-domain ownership, DNS/TXT state, and TLS where applicable;
- webhooks, deploy keys, GitHub Apps, OAuth integrations, submodules, badges,
  callbacks, and external consumers;
- old repository URL redirect and new canonical URL;
- absence of secret values from logs and evidence.

Exit condition: zero unexplained drift. A failure stops the wave and names the
exact repair owner.

## ADL and Website Cutover Window

`agent-design-language` transfers last. Before its window, all preceding
repositories must have passed Gate 4 and organization Actions policy must be
known to work.

Current `agent-logic.ai` `origin/main` contains four public links to
`https://github.com/danielbaustin/agent-design-language`:

- `site/index.html`, header;
- `site/index.html`, footer;
- `site/beta/index.html`, header;
- `site/beta/index.html`, footer.

Prepare a dedicated `agent-logic.ai` PR that changes only those current links
to `https://github.com/agent-logic/agent-design-language`. Do not merge it before
the ADL destination exists.

Immediately after ADL transfers:

1. verify the ADL destination and critical CI/integration surfaces;
2. merge and deploy the prepared `agent-logic.ai` link update;
3. verify production and beta pages link directly to the new canonical URL;
4. verify the old ADL URL redirects without relying on that redirect in current
   site source;
5. update ADL clone remotes, Actions/OIDC subjects, package references,
   webhooks, Apps, badges, documentation, and external consumers;
6. run the bounded ADL CI and publication smoke defined by the manifest.

## Organization-Wide Final Verification

After all five repositories pass their individual gates:

1. List source and destination inventories and confirm exactly five transfers.
2. Confirm `danielbaustin/asksifu` is unchanged and personal.
3. Confirm `danielbaustin/Horust` was not transferred or modified.
4. Search current operational surfaces for old owner references and disposition
   every remaining match.
5. Verify organization teams, outside collaborators, policies, Actions budget,
   runner access, package access, security features, and recovery ownership.
6. Verify production and beta `agent-logic.ai` links.
7. Record before/after manifest digests, source and destination URLs, exact
   HEADs, transfer times, validation outcomes, repairs, and accepted residual
   risks.

Do not remove old local configuration or rollback artifacts until the agreed
retention window ends.

## Failure and Recovery

When a transfer or verification fails:

1. stop the migration wave;
2. preserve the destination repository and both manifests;
3. diagnose settings, access, package, URL, Pages, workflow, or integration
   drift without deleting or recreating repository data;
4. prefer in-place repair;
5. transfer back only after an organization owner determines that in-place
   repair is unsafe;
6. rerun the complete Gate 4 verification before resuming.

GitHub transfer-back is not assumed to restore every organization setting,
assignment, package association, Pages binding, secret value, alert history, or
external integration. Each manifest must name a repair owner for those surfaces.

Where OIDC dual authorization is impossible, the manifest must declare the
expected authentication interruption and sequence transfer, trust update, and
verification as one bounded maintenance window.

## Evidence Package

Retain one compact migration package containing:

- approved organization and owner roles;
- the five migrate repositories and two exclusions;
- before/after manifest digests and exact HEADs;
- transfer timestamps and canonical URLs;
- per-repository verification outcomes;
- website deployment verification;
- repairs and accepted residual risks;
- billing/runner owner and first post-migration review date.

The evidence package is operational history, not a dependency required for
ordinary repository use after migration.

## Sources

- GitHub, `Transferring a repository`:
  <https://docs.github.com/en/repositories/creating-and-managing-repositories/transferring-a-repository>
- GitHub, `Repository roles for an organization`:
  <https://docs.github.com/en/organizations/managing-user-access-to-your-organizations-repositories/managing-repository-roles/repository-roles-for-an-organization>
- GitHub, `Forks`:
  <https://docs.github.com/en/pull-requests/reference/forks>
- Current live GitHub inventory for `danielbaustin` and `agent-logic`, observed
  2026-08-04.
- Current `agent-logic/agent-logic.ai` `origin/main` link inventory, observed
  2026-08-04.
- Repository operating contract: `AGENTS.md`.

## Gemini 3.1 Pro Review

Gemini 3.1 Pro reviewed this final plan through the repository's Rust provider
adapter using OpenRouter route `google/gemini-3.1-pro-preview`. The result was
`pass_with_findings`.

Findings and dispositions:

1. **GHCR/package ownership:** incorporated with a correction grounded in
   GitHub's registry-specific transfer rules. Granular packages remain scoped
   to their original account and lose repository linkage/access; repository-
   scoped registries may transfer. The plan now requires classification and a
   destination publication/consumer strategy rather than assuming one behavior
   for every registry.
2. **Git LFS quota and billing:** incorporated in organization readiness and
   post-transfer verification.
3. **Pages custom-domain and TLS interruption:** incorporated as an explicit
   pre-staged DNS/domain-verification task and possible maintenance window.
4. **Personal-account teams:** corrected. The source manifest records
   individual collaborators, and the destination preparation maps them to
   organization membership and teams.
5. **Organization secrets:** incorporated as an administrator-owned,
   secret-safe provisioning step that records names and scopes only.

Gemini's simplification suggestion to generate machine-readable manifests is
accepted. Execution should use the repository-native C-SDLC GitHub surfaces and
GitHub API readback where supported, with only unsupported settings recorded
manually. A new migration-only CI framework is not required; existing focused
repository workflows provide the post-transfer smoke proof.

## Review Boundary

Gemini review is required to check GitHub-specific transfer prerequisites,
scope accuracy, website cutover, Actions/OIDC/packages/Pages behavior,
rollback, evidence proportionality, and opportunities to simplify this plan.
The review is advisory and does not authorize migration.

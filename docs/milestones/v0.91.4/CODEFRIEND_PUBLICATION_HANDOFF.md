# CodeFriend Publication Handoff

Date: `2026-05-27`

Issue: `#3376`

## Status

Completed the publication-safety review and handoff for the live CodeFriend
coming-soon surface.

## External repository state

- Repository: `https://github.com/agent-logic/codefriend.ai`
- Handoff commit: `cdc2f88` `docs: add CodeFriend publication handoff`

## Live publication truth

- `https://codefriend.ai` is live over HTTPS
- `https://www.codefriend.ai` is live over HTTPS
- CloudFront is the public delivery layer
- the S3 bucket remains the private origin

## Publication-safety result

Public claim boundary is still clean:

- allowed:
  - CodeFriend exists as a coming-soon product surface from Agent Logic, Inc.
- not supported:
  - product availability claims
  - autonomous code-review authority claims
  - compliance/certification claims
  - customer-data, signup, or analytics claims

## Verification performed

- `scripts/verify_site.sh`
- `curl -I https://codefriend.ai`
- `curl -s https://codefriend.ai`
- `curl -I https://www.codefriend.ai`
- `curl -s https://www.codefriend.ai`
- focused text scans of the handoff and publication-boundary docs for secrets,
  local paths, and unsupported product claims

## Handoff outcome

The CodeFriend pre-alpha sidecar now has:

- a private product/site repository
- a reviewed coming-soon page
- Terraform-managed AWS static-site infrastructure
- live HTTPS delivery on apex and `www`
- deployment, verification, rollback, and publication-handoff documentation

What remains for the later CodeFriend alpha milestone:

- real product runtime and user flows
- any authenticated/customer-facing application state
- product-specific release processes beyond the static site
- broader product messaging beyond the current coming-soon posture

## Next step

The CodeFriend mini-sprint umbrella `#3372` can now close out truthfully as a
completed pre-alpha landing-surface sprint.

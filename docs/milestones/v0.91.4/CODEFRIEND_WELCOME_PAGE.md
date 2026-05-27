# CodeFriend Welcome Page

Date: `2026-05-27`

Issue: `#3374`

## Status

Completed the minimal static CodeFriend coming-soon page in the private
`agent-logic/codefriend.ai` repository.

## External repository state

- Repository: `https://github.com/agent-logic/codefriend.ai`
- Page refinement commit: `2526d98` `site: refine CodeFriend coming-soon page`

## Final page copy

- `CodeFriend`
- `because your code needs a friend.`
- `Coming soon from Agent Logic, Inc.`

## What changed

The initial bootstrap page from `#3373` was refined into the reviewed
coming-soon surface for the pre-alpha product site:

- cleaner page structure
- restrained visual polish
- clearer typographic hierarchy
- local preview instructions in `docs/DEPLOYMENT.md`

The issue stayed intentionally narrow:

- no extra pages
- no scripts
- no forms
- no analytics
- no product-availability claims

## Preview evidence

The issue produced truthful local preview evidence through the local static
server path:

- preview command:
  - `python3 -m http.server 4173`
- preview URL:
  - `http://127.0.0.1:4173/site/`
- served HTML confirmed the expected final copy
- text-browser render confirmed the visible text content and ordering

Browser automation in this environment was unreliable, so this issue records
the local served-output checks truthfully rather than pretending a stronger
browser screenshot proof than we actually obtained.

## Acceptance summary

- approved copy present: yes
- responsive static page source: yes
- no external scripts, trackers, forms, or customer-data hooks: yes
- no overclaiming product readiness: yes

## Next step

`#3375` should provision the Terraform-managed AWS static-site stack and make
the page live over HTTPS at `codefriend.ai` and `www.codefriend.ai`.

# Issue #5653: README status, release, CI, and homepage refresh

## Boundary

Update only the root `README.md`. The change documents current repository truth;
it does not create a release, tag, release approval, or runtime cutover.

## Contract

The README will identify the active v0.91.8 release-tail posture, link to the
ADL homepage at `https://agent-logic.ai`, retain CI and coverage badges for
`main`, and avoid stale v0.91.5-only language or unsupported release claims.
The release plan and release-notes draft remain the source of truth for the
unreleased status.

## Proof

Focused proof checks the required homepage URL, current milestone wording,
absence of stale v0.91.5-only status, badge branch targets, and Markdown link
syntax. A bounded exact-head review precedes publication.

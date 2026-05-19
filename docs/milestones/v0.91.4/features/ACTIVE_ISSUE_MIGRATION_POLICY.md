# Active Issue Migration Policy

## Status

Planned `v0.91.4` feature.

## Purpose

Define how open and future ADL software-development issues move onto C-SDLC
without corrupting in-flight work.

The end state is simple: future ADL software-development issues use C-SDLC by
default. The migration path needs care because some active issues may already
have older cards, open PRs, or partially completed closeout records.

## Scope

The policy must classify active issues as:

- migrate now
- migrate at next lifecycle boundary
- leave unchanged and close out under the old contract
- no-op close or fold into another issue
- block until operator judgment

## Acceptance Criteria

- The policy names the decision criteria for each migration class.
- A sampled active-issue audit demonstrates the classification process.
- Future issue creation defaults to the canonical C-SDLC card sequence.
- In-flight PRs are not forced through unsafe card rewrites.
- Historical records keep their truth while new records stop reproducing old
  drift.

## Non-Goals

- This feature does not rewrite all historical issue records.
- This feature does not hide old drift by renaming it complete.
- This feature does not bypass editor skills for card normalization.

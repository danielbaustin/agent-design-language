# Structured Intent Prompt

Template: 1.0.0

Issue: 5717

Repository: danielbaustin/agent-design-language

Card: sip

Status: ready

## Goal

Make the integrated Synthetic Minds Podcast studio page launch-ready with the operator-requested copy, logo, episode numbering, contact, FAQ, video, spacing, and footer fixes.

## Required Outcome

The studio route shows the corrected Synthetic Minds Podcast copy and Agent Logic logo, uses proposed episode topics numbered from 1, preserves audio/RSS behavior, and remains generated from the committed studio reference bundle.

## Scope

- podcast studio reference HTML and digest
- generated podcast studio served HTML and digest
- correct logo asset copied into the studio bundle
- issue-local C-SDLC lifecycle evidence

## Authority

- GitHub issue #5717
- PR #5716 integrated studio route foundation
- operator-provided copy-fix list

## Assumptions

- none

## Operator Constraints

- Use only FastWork for tracked issue work.
- Do not write tracked changes on main.
- Use the correct Agent Logic logo.
- Name the page Synthetic Minds Podcast.
- Replace the guest wording with 'Special guests join us occasionally.'
- Correct podcast numbers to start at 1.
- Reduce the gap below the Listen now button.
- Use podcast@agent-logic.ai as the contact email link.
- Answer that there is no video.
- Keep FAQ capitalized.
- Replace fake episodes with proposed topics starting at 1.
- Add a line break after copyright.
- Keep audio and RSS working.

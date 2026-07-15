# Structured Task Prompt

Template: 1.0.0

Issue: 5383

Repository: danielbaustin/agent-design-language

Card: stp

Status: ready

## Task

Planning/setup only: issue routing repair plus milestone documentation package. No product implementation, deletion, deployment, or release approval claims.

## Deliverables

- docs/milestones/v0.91.8/README.md
- docs/milestones/v0.91.8/VISION_v0.91.8.md
- docs/milestones/v0.91.8/DESIGN_v0.91.8.md
- docs/milestones/v0.91.8/DECISIONS_v0.91.8.md
- docs/milestones/v0.91.8/WBS_v0.91.8.md
- docs/milestones/v0.91.8/WP_ISSUE_WAVE_v0.91.8.yaml
- docs/milestones/v0.91.8/feature and review planning surfaces
- v0.91.7/v0.92 handoff truth updates if needed

## Acceptance

1. AC-1: #4641 is restored to v0.91.7 WP-14 title/body/version label and new WP-14A issue preserves the overwritten v0.91.8 content
2. AC-2: Complete v0.91.8 milestone planning package exists in planned posture
3. AC-3: WP_ISSUE_WAVE_v0.91.8.yaml maps the live v0.91.8 issues, including WP-14A
4. AC-4: Feature docs cover first-class v0.91.8 tracks and do not claim execution evidence
5. AC-5: Focused Markdown/YAML/link/placeholder validation and git diff --check pass

## Dependencies

- Existing live v0.91.8 issue wave
- Restored #4641 source truth from v0.91.7 records
- New WP-14A preservation issue

## Inputs

- docs/milestones/v0.91.7/WBS_v0.91.7.md
- docs/milestones/v0.91.7/WP_ISSUE_WAVE_v0.91.7.yaml
- docs/milestones/v0.91.7/V092_HANDOFF_v0.91.7.md
- docs/milestones/v0.92/FIRST_BIRTHDAY_LAUNCH_PACKET_v0.92.md
- GitHub issues labeled version:v0.91.8

## Non Goals

- Implement v0.91.8 product/runtime/tooling changes
- Delete ADL incumbent code
- Start v0.92 birthday implementation
- Claim v0.91.8 release approval

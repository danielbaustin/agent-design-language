# Milestone README - v0.90.1

## Metadata

- Milestone: v0.90.1
- Version: v0.90.1
- Status: early planning lane open
- Planning source: `.adl/docs/v0.90.1planning/`
- Promotion target: `docs/milestones/v0.90.1/`
- Planning issue: #2090

## Purpose

v0.90.1 is the follow-on milestone that turns the v0.90 long-lived-agent
runtime into the first bounded Runtime v2 foundation prototype.

The milestone should prove the substrate, not the full citizen birth story.
It creates inspectable runtime surfaces that later moral, emotional, and
identity milestones can inherit.

## Planning Model

The full planning package is being developed first under the local planning
lane at `.adl/docs/v0.90.1planning/`, using the same directory structure as the
v0.90 planning package:

- root planning docs and WP YAML in the package root
- implementation-facing feature contracts under `features/`
- reader-facing context and later-band backgrounders under `ideas/`

When v0.90 reaches its release-tail planning gate, the package should be
reviewed against final v0.90 truth and then promoted into this tracked
milestone directory by a dedicated promotion issue.

## Scope Summary

### In Scope

- kernel service loop for a bounded citizen-like process
- minimal citizen record with explicit provisional status
- snapshot and rehydration path
- manifold links across identity, memory, task, and temporal surfaces
- invariant violation artifacts
- operator inspect, pause, resume, and terminate controls
- security-boundary evidence for defending the polis

### Out Of Scope

- first true Gödel-agent birthday
- full moral/emotional civilization
- complete cross-polis migration semantics
- red/blue/purple security ecology as CSM core
- v0.91 affect, kindness, morals, humor, wellbeing, and civic substrate
- v0.92 identity/capability rebinding and birthday semantics

## Acceptance Boundary

The milestone succeeds when Runtime v2 can be demonstrated as a bounded,
reviewable substrate with explicit continuity handles and failure artifacts.

It must not claim that a provisional citizen record is the first true Gödel
agent. That event remains reserved for v0.92.

## Planned Proof Surfaces

- manifold manifest
- kernel service-loop report
- provisional citizen records
- snapshot and rehydration report
- invariant violation artifact
- operator control report
- security-boundary proof packet
- integrated Runtime v2 prototype demo packet

## Roadmap Boundary

v0.90.1 adds a foundation slice between v0.90 and v0.91. It does not reduce the
existing v0.91 or v0.92 milestone stories.

- v0.90: long-lived runtime, cycles, continuity handles, operator safety
- v0.90.1: Runtime v2 foundation prototype
- v0.91: affect, kindness, morals, humor, wellbeing, civic substrate
- v0.92: first true Gödel-agent birthday, identity/capability continuity,
  richer memory and rebinding

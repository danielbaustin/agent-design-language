# CSM Observatory Operator Report

## Purpose

The CSM Observatory operator report is the text proof surface for the
visibility packet. It lets an operator, reviewer, or third-party reader inspect
the manifold state without opening the visual console.

The report is intentionally not a packet dump. It renders the same
adl.csm_visibility_packet.v1 fixture into a compact Markdown brief that answers:

- What is the manifold state right now?
- What requires judgment?
- Which citizens and invariants are safe, pending, or deferred?
- Which evidence is fixture-backed, missing, or future Runtime v2 scope?

## Source Contract

Input packet:

- demos/fixtures/csm_observatory/proto-csm-01-visibility-packet.json

Packet contract:

- docs/milestones/v0.90.1/features/CSM_OBSERVATORY_VISIBILITY_PACKET.md
- adl/schemas/csm_visibility_packet.v1.schema.json

Renderer:

- adl/tools/render_csm_observatory_report.py

Generated proof artifact:

- demos/v0.90.1/csm_observatory_operator_report.md

Validation:

- bash adl/tools/test_demo_v0901_csm_observatory_operator_report.sh

## Report Sections

The report has stable sections so later CLI and review tooling can compare
outputs:

- Report Identity
- Operator Summary
- Attention Items
- Manifold And Kernel
- Citizens
- Freedom Gate Docket
- Invariant Review
- Resources
- Trace Tail
- Operator Action Boundary
- Evidence And Caveats
- Next Consumers
- Reviewer Use

## Attention Model

The renderer promotes operator attention items from packet data instead of
expecting the reviewer to infer them from raw JSON.

Attention sources include:

- manifold health attention items
- deferred or missing snapshot state
- non-active citizens
- citizen alerts
- non-healthy invariants, sorted by severity
- causal gaps
- open Freedom Gate questions
- disabled mutation actions and their future issue links

This model keeps the report useful even when the packet grows. Reviewers should
see the judgment surface first, then supporting tables.

## Truth Boundary

Demo classification: fixture_backed.

The first v0.90.1 report proves deterministic packet-to-report rendering. It
does not prove a live CSM run, live Runtime v2 capture, live operator mutation,
snapshot/wake completion, or v0.92 identity rebinding.

The operator action section must preserve the read-only boundary. Mutation
actions remain disabled until command packets route through the Runtime v2
kernel/control plane.

## Reviewer Use

Use the generated report as a third-party review proof surface when the reviewer
needs to inspect the Observatory state without running a browser. It is also a
good companion artifact for the static console because both surfaces are derived
from the same packet contract.

The report is especially useful for checking:

- whether evidence labels and caveats are visible
- whether attention routing is clear
- whether the packet avoids private paths, endpoints, and raw transient dumps
- whether the Observatory distinguishes fixture, missing, deferred, and future
  live-runtime claims

# v0.91.8 Demo Matrix

Status vocabulary:

- `proven`: exact tracked proof exists for the bounded row claim.
- `retained_proof`: exact tracked proof exists, but the row is consumed as
  retained input rather than rerun by this matrix.
- `closed_planning`: the owner issue closed with planning or source-route
  preparation, not live product proof.
- `open_gate`: the owner issue remains an open release-tail gate.
- `deferred`: the row is intentionally queued to a later milestone or owner.

This matrix consumes accepted proof from owner issues. It does not rerun those
demos, replace their evidence, or promote planning, screenshots, metadata, or
fixtures into runtime proof.

| Surface | Owners | Status | Evidence / disposition | Claim boundary |
| --- | --- | --- | --- | --- |
| Fresh install and selector smoke | #5345, #5344 | `retained_proof` | Stable install and selector proof is retained under `.csdlc/evidence/5345/implementation-proof.log`; rollback/soak ownership is #5344. | Selector and install behavior only; this row does not prove whole-release completion. |
| ADL v2 compile to canonical plan | #5338, #5339, #5340, #5354 | `proven` | #5354 consumes ADL v2 plan/run output in `.csdlc/evidence/5354/convergence-proof.v1.json`, with supporting ADL v2 implementation logs under `.csdlc/evidence/5339/implementation-validation` and `.csdlc/evidence/5340/engine-focused/engine-focused.log`. | Bounded compiler and engine path at cited revisions; not all historical ADL behavior. |
| Runtime v3 canonical ingress and Observatory path | #5341, #5361, #5591, #5590, #5354 | `proven` | #5354 records live canonical submit, TLS observation, and full-duplex WSS in `.csdlc/evidence/5354/convergence-proof.v1.json`; Runtime v3 acceptance summary is `.csdlc/evidence/5361/acceptance-proof-summary.json`. | Runtime v3 path only; Runtime v2, cloud deployment, and every Observatory UX are explicit non-claims. |
| C-SDLC v2 lifecycle | #5358, #5354 | `proven` | #5354 records selected C-SDLC v2 phase/doctor truth in `.csdlc/evidence/5354/convergence-proof.v1.json`; C-SDLC v2 acceptance owns broader lifecycle deployment. | Typed lifecycle governance only; this row does not prove every tool or historical record is terminal. |
| Distributed C-SDLC workcell | #5497, #5499, #5498, #5500, #5502, #5501 | `retained_proof` | Live workcell proof is retained in `.csdlc/evidence/5501/retained-live-proof.json` and `.csdlc/evidence/5501/live-run-manifest.json`, with dashboard/convergence evidence referenced there. | Workcell acceptance only; no autonomous merge, closeout, or unbounded agent authority is claimed. |
| Integrated three-product stack | #5384, #5354 | `proven` | `.csdlc/evidence/5354/convergence-proof.v1.json` proves the bounded ADL v2 plan/run, Runtime v3 canonical ingress, TLS live API, full-duplex WSS, typed C-SDLC v2, and accepted Unity chain. | Bounded integrated path at the recorded revisions; not Runtime v2, arbitrary cloud operation, or whole-release completion. |
| Unity Observatory readiness | #4739, #4741, #5332, #5683, #5354 | `retained_proof` | #5683 retains live Unity evidence in `.csdlc/evidence/5683/LIVE_UNITY_PROOF.md`; #5354 consumes it through `.csdlc/evidence/5354/convergence-proof.v1.json`. | Editor, Play Mode, and presentation proof only; retained images do not prove player-build readiness or live Runtime/cloud authority. |
| Synthetic Minds Podcast launch route | #5605, #5702, #5708, #5711, #5715, #5717 | `retained_proof` | #5717/#5720 merged the source routes under `demos/podcast/`, including `demos/podcast/feed.xml`, `demos/podcast/audio/meet-the-ai-coworkers.wav`, `demos/podcast/LAUNCH_READINESS.md`, and the checked validator `adl/tools/validate_podcast_launch_packet.py`; #5605 remains the planning packet. | Hidden/local source route, RSS/feed shape, and smoke audio are ready for review; public hosting, directory approval, mailbox verification, final audio, video, and durable weekly cadence remain non-claims. |
| v0.92 handoff and birthday-prep package | #5362, #5355, #4758, #4759, #4760, #4761, #4762, #4763, #5007, #5107 | `retained_proof` | Capability-envelope input exists at `.csdlc/evidence/4761/capability-envelope/envelope.v1.json`; live GitHub truth on 2026-08-04 shows #5362 and #5355 closed through merged PRs #5807/#5808, with #5359 and #5348 remaining open downstream gates. The named preparation children are closed retained inputs. | Handoff and preparation only; not v0.92 activation, birthday implementation, WP-22 approval, or WP-23 release ceremony. |

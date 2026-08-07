# Continuous Adversarial Verification for Continuous Security

Security reviews are snapshots. Agent systems are moving targets.

Models change, tools change, policies change, dependencies change, and the system accumulates new state. A penetration test or design review can provide valuable evidence, but its conclusions begin aging as soon as the environment moves.

ADL's continuous adversarial verification direction asks how a governed agent runtime can examine its own security repeatedly, retain what it learns, and improve defenses without turning “self-attack” into uncontrolled harm.

## Red, blue, and purple are responsibilities

ADL models three security roles.

The **red role** develops bounded offensive hypotheses against a declared target. It asks how a policy, interface, parser, tool boundary, or state transition might fail.

The **blue role** interprets exploit evidence, proposes mitigations, and tests defensive outcomes.

The **purple role** coordinates scope, prioritization, replay, escalation, regression, and durable learning across both sides.

These are not character prompts or theatrical personalities. They are accountable responsibilities with declared authority and visible artifacts. The red role does not gain permission to attack arbitrary systems. The blue role does not get to mark a mitigation effective without proof. The purple role does not erase disagreement to produce a tidy report.

## The verification loop

The core loop is compact:

> surface → hypothesis → exploit attempt → defense → replay → learning

Each arrow carries governance requirements.

The surface must be explicitly in scope. A hypothesis needs a threat model and safe execution boundary. An exploit attempt must preserve target identity, authorization, resource limits, and evidence. A defense must address the observed mechanism rather than only the example input. Replay must prove the mitigation without reintroducing harm. Learning must retain the result in a form that later runs can use.

This is why “continuous” means more than running a scanner frequently. The system needs a durable relationship between attack evidence, defensive change, regression proof, and runtime state.

## Why agents change the threat model

Capable models lower the cost of generating and adapting attack hypotheses. They can inspect large surfaces, combine patterns, and iterate quickly. Defenders can use the same capabilities, but only if their systems turn generated ideas into bounded experiments rather than uncontrolled action.

The asymmetric risk is straightforward: if vulnerability discovery becomes cheap, a real weakness should be assumed discoverable by someone. Occasional review remains useful, but it cannot be the entire defensive posture.

An agent runtime also introduces new boundaries. Model output may contain instructions rather than data. Tool calls can cause side effects. Memory can preserve poisoned state. Delegation can expand authority. Logs can leak secrets. Checkpoint restore can revive revoked capability if continuity is wrong.

Continuous adversarial verification therefore has to examine runtime governance, not only application code.

## Freedom Gate for security work

Security testing is itself a powerful capability. ADL's ordinary governance principles still apply.

A red action should pass through scoped authority and policy. Dangerous payloads should stay inside approved fixtures or isolated targets. Results should be redacted according to audience. Resource ceilings and stop conditions should bound iteration. Refusals and quarantines should remain evidence.

This design avoids a common contradiction: building a safety system that disables safety controls whenever it performs security research.

The same gate also protects defensive mutation. A proposed mitigation is not automatically adopted. It needs review, focused validation, and replay against the original evidence. Broader regressions remain possible, so the change must enter the normal software lifecycle.

## Durable security memory

A report that cannot influence the next run has limited value. ADL's longer-range design connects adversarial evidence to runtime memory and governed learning.

The retained record should distinguish the original surface, hypothesis, exploit status, mitigation, replay result, residual risk, and scope limits. A later agent can retrieve that record, but it should not inflate a fixture success into a universal claim.

Negative results matter too. An exploit attempt that failed under one configuration is evidence about that bounded attempt, not proof that the class of vulnerability is absent.

This posture makes security knowledge cumulative while preserving uncertainty.

## Current proof and future work

ADL has documented and exercised bounded red/blue architecture, security bridge surfaces, threat models, source packets, and review evidence. Those artifacts support the architecture and selected integration claims.

They do not constitute external certification, comprehensive production protection, or permission for unrestricted autonomous attack. A broader adversarial-security issue wave remains later work.

Continuous adversarial verification is therefore both a concrete engineering direction and an unfinished integration challenge. The central idea is already clear: security should become a governed learning loop whose evidence survives every iteration.

In an environment where offensive capability can accelerate continuously, defense needs continuity too.

## Repository Sources

- [`docs/explainers/RED_BLUE_SECURITY.md`](../../../../../explainers/RED_BLUE_SECURITY.md)
- [`docs/milestones/v0.89.1/features/CONTINUOUS_VERIFICATION_AND_EXPLOIT_GENERATION.md`](../../../../v0.89.1/features/CONTINUOUS_VERIFICATION_AND_EXPLOIT_GENERATION.md)
- [`docs/milestones/v0.91.5/features/CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SOURCE_PACKET_v0.91.5.md`](../../../../v0.91.5/features/CAV_THREAT_MODEL_AND_CODEFRIEND_SECURITY_SOURCE_PACKET_v0.91.5.md)
- [`docs/milestones/v0.91.6/features/SECURITY_BRIDGE_AND_CAV_v0.91.6.md`](../../../../v0.91.6/features/SECURITY_BRIDGE_AND_CAV_v0.91.6.md)
- [`docs/milestones/v0.93/RED_BLUE_ADVERSARIAL_SECURITY_ISSUE_WAVE_v0.93.md`](../../../../v0.93/RED_BLUE_ADVERSARIAL_SECURITY_ISSUE_WAVE_v0.93.md)

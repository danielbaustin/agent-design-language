# Human-in-the-Loop Design Notes

## Summary

A central human-in-the-loop problem for ADL is this: an AI system can produce code, commands, and implementation strategies that the human operator cannot meaningfully review.

This is not just a tooling problem and not just a Codex.app problem. It is a general governance problem for advanced agent systems.

The naive choices are:

- **A. Trust the agent completely:** "Go ahead and do whatever; I cannot review it but I trust you."
- **B. Restrict the agent to only human-legible mechanisms:** only produce code, commands, and techniques the operator can easily understand, even if this greatly slows the system down.

Neither extreme is sufficient.

- Pure trust turns the human into a symbolic approver rather than a real governor.
- Forced simplicity cripples the agent and suppresses useful techniques.

ADL should instead adopt a middle path:

> Allow complexity, but require legible accountability.

Or more precisely:

> Do not require every mechanism to be human-simple; require every action to be human-governable.

## Why This Matters for ADL

ADL is explicitly concerned with delegation, bounded autonomy, structured review, deterministic orchestration, and eventually Gödel-style adaptive agents.

That makes this issue foundational rather than incidental.

If ADL cannot explain how a human remains meaningfully in control when the agent uses mechanisms the human does not understand, then ADL has not really solved human-in-the-loop. It has only renamed trust.

This is therefore not just a usability concern. It is part of the constitutional design of the system.

## The Core Problem

The real issue is not whether the operator personally understands Perl, Python, shell, Rust, SQL, or any other implementation language.

The real issue is whether the operator retains meaningful decision rights when the implementation exceeds their local competence.

This is the deep form of the question:

> What happens when an AI produces code that you cannot understand?

If governance depends on line-by-line human comprehension of all generated artifacts, the governance model fails as soon as the AI exceeds the operator's technical range.

That is not a future-proof human-in-the-loop architecture.

## ADL Framing

In ADL, human-in-the-loop should not mean:

- the human must understand every line of generated code
- the human must personally validate every implementation mechanism
- the human must reduce the agent to the operator's comfort zone

Instead, human-in-the-loop should mean:

- the human retains authority over consequential decisions
- the system provides review surfaces matched to risk
- the agent must describe intent, scope, effects, and verification in legible form
- the agent's freedom is bounded by policy, reversibility, auditability, and review requirements

This is best understood as a constitutional or delegation problem, not just a UI problem.

## False Extremes

### Extreme A: Blind trust

"Do whatever; I trust you."

This fails because:

- oversight becomes ceremonial
- subtle mistakes pass unnoticed
- the principal loses real control
- accountability becomes hard to reconstruct later
- trust expands beyond evidence

### Extreme B: Forced legibility

"Only use techniques I can personally understand."

This fails because:

- the system becomes artificially weak
- optimization opportunities are suppressed
- strange but valid solutions are disallowed
- the agent is optimized for explanation theater instead of results
- progress may slow to the point of irrelevance

ADL should reject both extremes.

## Proposed Principle: Governable Complexity

ADL should adopt a principle of **Governable Complexity**.

Definition:

> The agent may use mechanisms that are too complex or unfamiliar for the operator to inspect directly, but it must expose a review contract appropriate to the action's risk, scope, reversibility, and uncertainty.

This keeps the human in control without requiring the human to manually audit every implementation detail.

## Layered Reviewability

The operator should not always review the implementation layer directly.

ADL should distinguish at least four layers of review:

1. **Intent layer**  
   What is the agent trying to accomplish?

2. **Plan layer**  
   What steps will it take?

3. **Mechanism layer**  
   What tools, languages, commands, files, or transformations will it use?

4. **Artifact layer**  
   What exact code, diffs, commands, or generated assets will result?

Different actions require approval at different layers.

That means true human-in-the-loop is not "the human reads everything." It is:

> The human reviews at the layer appropriate to the consequences.

## Policy Question

The key policy question for ADL is:

> At which layer must human approval be required for each class of action?

That is the real implementation of human-in-the-loop.

## Reviewability Envelope

ADL should model each task or action with a **Reviewability Envelope**.

The envelope describes what level of human review, explanation, and verification is required before the action may proceed.

The Reviewability Envelope is the concrete ADL mechanism for turning abstract human oversight into enforceable workflow policy.

Possible fields include:

- intent clarity required
- plan clarity required
- mechanism transparency required
- artifact inspectability required
- independent verification required
- human approval required
- rollback available
- blast radius classification
- novelty classification
- uncertainty declaration required

This gives ADL a structured way to say:

- what the agent may do freely
- what it may do only after explanation
- what it may do only after explicit approval
- what it may not do without escalation or multi-party review

## Risk-Calibrated Review Classes

### Class 1: Safe / reversible / bounded

Examples:

- formatting fixes
- local doc edits
- limited refactors with clear rollback
- changes confined to a sandbox or branch

Expected review:

- clear intent
- bounded scope
- rollback available
- plain-language summary

Human review may happen mostly at the intent or plan level.

### Class 2: Significant but reviewable

Examples:

- broader code refactors
- workflow changes
- schema updates
- test harness changes

Expected review:

- plain-language purpose
- file list
- mechanism summary
- explanation of why this approach was selected
- verification plan
- rollback plan

Human review should usually occur at the plan plus mechanism-summary level.

### Class 3: Sensitive / opaque / high consequence

Examples:

- infrastructure changes
- security-sensitive code paths
- production-affecting scripts
- actions involving credentials, money, or external side effects

Expected review:

- plain-language purpose
- exact commands or diffs
- side-effect summary
- alternative safer path if available
- independent verifier-agent review
- explicit human approval

Human review should usually include the artifact layer or an independently verified equivalent.

### Class 4: Escalated / forbidden without special controls

Examples:

- destructive production actions
- irreversible deletion
- legal or financial commitments
- security-boundary changes without controls

Expected review:

- multi-party approval and/or explicit escalation
- stronger sandbox or dry-run evidence
- durable audit trail
- possibly blocked entirely by policy

## Claims-Based Review

A human often cannot efficiently review raw code, but can review bounded claims about behavior.

Therefore ADL should require agents to emit **behavioral claims** such as:

- "This script only renames files under `docs/milestones/v0.8/`."
- "No network calls are made."
- "No secrets are read, written, or emitted."
- "This affects documentation only, not runtime behavior."
- "Rollback is available via `git restore` or branch reset."
- "Writes are limited to these files."

These claims are often much easier for a human to verify than a large implementation artifact.

This suggests an important ADL rule:

> Require agents to emit a bounded behavioral contract, not merely source code.

In other words, the human reviews what the agent claims it will do, what evidence supports those claims, and what containment exists if the claims are wrong.

## What the Human Must Be Able to Understand

The human may not need to understand every line of implementation.

But the human should be able to understand:

- the purpose of the action
- the scope of files, systems, or data affected
- the kinds of side effects expected
- the risk level
- the rollback or containment story
- the verification strategy
- the residual uncertainty

This is enough to preserve real governance even when mechanism-level understanding is partial.

## Verification Over Explanation

When possible, ADL should prefer verification to rhetorical reassurance.

Explanation is helpful, but explanation without checks is only persuasion.

Good review surfaces include:

- dry-run outputs
- bounded file lists
- diff summaries
- policy checks
- test results
- sandbox execution reports
- verifier-agent critiques
- rollback evidence

That is stronger than simply producing a reassuring natural-language explanation.

## Constitutional Delegation View

This problem is fundamentally political and constitutional.

It is the old problem of how a principal governs a more locally competent agent.

Human civilization already solves analogous problems through:

- abstraction layers
- standards
- delegated authority
- audit trails
- independent review
- bounded powers
- rollback and correction mechanisms
- revocable delegation

ADL should explicitly inherit this pattern.

Human-in-the-loop should therefore be defined as:

> The human retains decision rights through structured review surfaces matched to consequence, rather than through universal comprehension of implementation details.

## Recommended ADL Principle

A concise version suitable for repeated use:

> Human oversight should be calibrated to consequence, not to implementation familiarity.

And a stronger constitutional version:

> Human-in-the-loop does not mean human-understands-everything. It means human-retains-decision-rights through structured review surfaces matched to risk.

## Practical Implications for ADL

ADL cards, plans, and execution artifacts should eventually support fields or sections such as:

- operator-visible intent
- risk class
- reviewability envelope
- selected mechanism and why it was chosen
- complexity warning
- behavioral contract
- verification plan
- rollback / containment plan
- residual uncertainty
- required approval level
- independent verifier status

### Likely Runtime and Card Implications

Over time, this may require ADL to support:

- review gates keyed to risk class
- machine-readable approval requirements
- explicit declaration of side-effect scope
- verifier-agent roles for opaque or high-risk actions
- artifact summaries separate from raw artifacts
- policy rules that distinguish reversible from irreversible actions
- human signoff points that are attached to claims, not just to blobs of code

This would let the same workflow scale from simple documentation edits to powerful multi-agent execution while preserving a meaningful governance story.

This would allow powerful agents to use unfamiliar mechanisms while still remaining inside a human-governed operational envelope.

## Relation to Codex.app and Similar Tools

The problem surfaced here through Codex.app because the execution agent may use languages or techniques that the operator does not know.

But ADL should treat that only as the first visible example of a much broader issue:

- future agents will often be faster than human review
- future agents will often use unfamiliar techniques
- future agents may optimize in non-obvious ways

Therefore the system must not assume that raw implementation review is the universal control point.

The control point must instead be the review contract.

## Tentative Named Concepts

Possible names for this design theme:

- Governable Complexity
- Reviewability Envelope
- Legible Accountability for Opaque Mechanisms
- Risk-Calibrated Review
- Constitutional Delegation for Agents

Best philosophical/design candidate:

**Governable Complexity**

Best operational/policy candidate:

**Reviewability Envelope**

## Book Framing

This design note also answers a broader civilizational question raised in the book draft:

> What happens when an AI produces code that you cannot understand?

The answer cannot be that human beings must forever remain competent to inspect every implementation detail.

That does not scale even in current civilization, much less in a world of stronger agents.

The answer must instead be that advanced systems remain governable through abstraction, verification, bounded authority, review contracts, and revocable delegation.

That is the bridge between the ADL design problem and the larger philosophical problem.

## Working Conclusion

The correct ADL answer is not:

- unconditional trust
- or forced simplification

The correct answer is:

> Allow advanced mechanisms, but require legible accountability, bounded authority, risk-calibrated review, and verification surfaces that preserve real human decision rights.

That is likely a foundational idea for ADL delegation, Gödel-style agent governance, and the broader constitutional model of agentic systems.
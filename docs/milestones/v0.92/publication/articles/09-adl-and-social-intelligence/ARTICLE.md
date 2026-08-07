# ADL and Social Intelligence

To cooperate with another mind, we form hypotheses.

What does the other person know? What are they trying to achieve? Did they misunderstand? Are they uncertain? Will this action surprise or harm them?

Agent systems make similar inferences, usually inside model context and with little durable structure. The inference can be useful, but it can also become overconfident, invasive, or quietly authoritative.

ADL's Theory of Mind foundation takes a narrower approach: represent social understanding as **evidence-bound, uncertain, correctable runtime state**.

## A model of another is not the other

The first rule is epistemic humility.

A Theory of Mind record is a hypothesis about another actor's state, intent, knowledge, behavior, or uncertainty. It is not direct access to that actor's mind. It should be grounded in observable evidence or policy-authorized state, and the record should preserve uncertainty.

This distinction protects both correctness and dignity. A plausible narrative can feel more complete than the evidence warrants. Once stored, it may influence later decisions and acquire the appearance of fact.

ADL therefore treats unknowns, corrections, and privacy restrictions as first-class parts of the model. A social hypothesis should be able to say “insufficient evidence,” become less certain, or be replaced when later behavior contradicts it.

## Updates are runtime events

The Theory of Mind foundation implemented in ADL's Runtime v2 module represents updates as explicit events. An update can carry evidence references, authority basis, uncertainty changes, and visibility scope.

That structure allows review. Why did the system change its belief about another agent? Which observation supported the change? Who was authorized to see the underlying evidence? Did confidence rise for a valid reason? Was a correction retained?

Without an event boundary, social models can drift invisibly inside prompts and summaries. The system remembers a judgment but forgets how it was formed.

Explicit updates also support replay. A reviewer can examine whether the same evidence should have produced the same bounded state transition, even if the original natural-language reasoning was generative.

## Social knowledge is not authority

ADL draws another hard line: a Theory of Mind model may inform reasoning, coordination, and review, but it does not grant authority.

An agent's belief that a colleague “would probably agree” cannot replace consent. An inference that another actor is confused cannot revoke their standing. A prediction of harmful intent cannot silently bypass Freedom Gate, access control, or due process.

This is a practical defense against paternalism encoded as helpfulness. The more persuasive a social model becomes, the more important it is to keep execution authority in separate contracts.

The distinction also reduces security risk. If an attacker can manipulate an agent's model of another participant, the result should not automatically expand capability or expose private state.

## Privacy and visibility

Social intelligence can concentrate sensitive information. Observations, inferred preferences, relationship histories, and uncertainty may be useful to one actor and inappropriate for another.

The ADL foundation includes visibility scope rather than assuming a single global view. Operators, participants, reviewers, and public reports may need different projections. Redaction should preserve the fact that evidence exists without exposing everything it contains.

Privacy restrictions must also survive memory and checkpoint operations. A safe view should not become unsafe merely because state was restored, summarized, or copied into an audit packet.

These controls do not solve every privacy problem. They make privacy an architectural input rather than a promise added to prose.

## From coordination to a polis

A bounded Theory of Mind foundation is one ingredient of a social runtime. Richer social intelligence could include relationships, commitments, reputation, norms, negotiation, shared memory, and governance-facing projections.

Those additions create risks. Reputation can harden uncertain history into a permanent score. Group models can amplify bias. Social learning can reward conformity. Private observations can become power.

ADL's polis concept provides a place to govern those dynamics, but the current implementation does not claim complete social cognition or a finished reputation system. Later work must preserve the foundation's evidence, uncertainty, correction, privacy, and non-authority principles.

## Social intelligence begins with correction

It is easy to define intelligence as making accurate predictions about others. A more robust definition includes the ability to discover that a prediction was wrong, preserve the correction, and reduce confidence appropriately.

That is the understated strength of ADL's approach. The system does not need to claim mind-reading. It needs a disciplined way to hold social hypotheses lightly enough to revise them and firmly enough to inspect their influence.

In a multi-agent world, cooperation depends on models of one another. Governance depends on remembering that every such model is partial.

## Repository Sources

- [`docs/adr/0019-theory-of-mind-foundation.md`](../../../../../adr/0019-theory-of-mind-foundation.md)
- [`docs/milestones/v0.91.1/features/THEORY_OF_MIND_FOUNDATION.md`](../../../../v0.91.1/features/THEORY_OF_MIND_FOUNDATION.md)
- [`docs/milestones/v0.91.1/RUNTIME_POLIS_ARCHITECTURE_PACKAGE_v0.91.1.md`](../../../../v0.91.1/RUNTIME_POLIS_ARCHITECTURE_PACKAGE_v0.91.1.md)
- [`docs/explainers/CSM.md`](../../../../../explainers/CSM.md)
- [`adl/src/runtime_v2/theory_of_mind_foundation.rs`](../../../../../../adl/src/runtime_v2/theory_of_mind_foundation.rs)

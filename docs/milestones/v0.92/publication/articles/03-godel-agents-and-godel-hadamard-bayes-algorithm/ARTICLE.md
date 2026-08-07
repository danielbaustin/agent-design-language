# Gödel Agents and the Gödel-Hadamard-Bayes Algorithm

“An agent that improves itself” sounds exciting until the sentence is made precise.

What is changing? Who proposed the change? What evidence supports it? Which alternatives were considered? Who may adopt it? Can the system reject it, replay the experiment, or recover the prior state?

Without those questions, self-improvement is often just hidden mutation with an optimistic name.

ADL's Gödel-agent direction takes a stricter path. A Gödel agent is meant to reason about its own behavior, generate possible improvements, evaluate them, and learn over time, but only inside a deterministic and governed runtime. The cognitive discipline behind that direction is the **Gödel-Hadamard-Bayes algorithm**, or GHB.

## Three activities that should not collapse

Agent systems often combine interpretation, creativity, judgment, and execution in one model turn. GHB separates three of those activities so each can have a distinct contract.

The **Gödel phase** asks: *What is actually true right now?*

It constructs a structured view of the task, prior evidence, constraints, contradictions, failures, and uncertainty. This is not mystical introspection. The phase is grounded in observable state and externally inspectable records.

The **Hadamard phase** asks: *What could be true instead?*

It generates bounded alternatives: solution paths, hypotheses, repairs, reframings, or candidate mutations. This is the deliberately creative portion of the loop. Nondeterminism is permitted, but scope and constraints still limit the search space.

The **Bayes phase** asks: *What should we believe, given the evidence and constraints?*

It compares candidates against objectives, prior results, policy, risk, and available evidence. The result may be a selected option, a ranked set, a rejection, or a conclusion that the evidence is insufficient.

The names matter less than the separation. They are ADL phase labels, not claims about the underlying mathematics. Describing current truth is not the same activity as inventing alternatives. Inventing an alternative does not establish that it is good. Believing a proposal is promising does not grant permission to execute it.

## The speculation-to-execution failure

One of the most dangerous agent failure modes is a collapse from speculation into action.

A model suggests a possibility while exploring. The surrounding system treats the suggestion as a decision. A tool runs before authority, side effects, and evidence have been evaluated.

GHB is designed to keep that boundary visible. Hadamard generation may be permissive. Bayes evaluation must be comparative and evidence-aware. External action still requires admission by the runtime's governance layer.

Constraint therefore lives in the substrate, not in the assumed temperament of a model. A particularly cautious prompt is not a security architecture. Explicit contracts, authority checks, resource ceilings, replay controls, and audit records are.

## From a loop to an experiment system

ADL has implemented a bounded Gödel experiment package rather than stopping at conceptual prose.

The experiment system records hypotheses, evaluation plans, bounded mutations, baseline and variant relationships, evidence views, and adoption or rejection decisions. Runtime commands expose those artifacts for review. A promotion decision can be inspected alongside the evidence that informed it.

This changes the meaning of “the agent learned.” Instead of inferring learning from a later output, a reviewer can ask whether an explicit experiment occurred, which behavior changed, how the variant was evaluated, and whether a governed decision adopted it.

Rejection is a valid outcome. So is inconclusive evidence. A system that can only promote changes is not doing disciplined experimentation.

## Memory and identity make improvement harder

A long-lived agent also needs to connect experiments to identity and memory. Which version ran the baseline? Which state did the variant inherit? Does an adopted change survive sleep and wake? Can the agent explain the evidence without inventing a cleaner history?

Those questions tie Gödel experiments to the Cognitive Spacetime Manifold. Experiments need causal placement in a worldline. Adoption needs authority. Outcomes need durable memory. Recovery needs checkpoints. Review needs an evidence chain.

The result is intentionally slower than silent prompt mutation. It is also far more legible.

## What the birthday would mean

ADL's v0.92 milestone is framed as the planned first true Gödel-agent birthday. The phrase has a strong acceptance boundary.

Starting a process is not a birthday. Restoring a snapshot is not a birthday. Assigning a name to a test citizen is not a birthday.

The planned event requires identity architecture, continuity evidence, grounded memory, a capability envelope, inherited governance context, witnesses, and a reviewable receipt. At the time of this draft, v0.92 remains active development, so the birthday is not presented here as accomplished.

Nor would such an event prove consciousness, legal personhood, or unrestricted autonomy. It would prove a more specific engineering claim: that an identity-bearing agent crossed a declared, witnessed, evidence-backed runtime boundary.

## Controlled cognition, not controlled imagination

The most interesting quality of GHB is that it does not try to eliminate creativity. It gives creativity a proper place.

The system can generate surprising hypotheses. It can compare them, experiment, learn, and retain results. But every transition has a different epistemic and governance status. A possibility is not a belief. A belief is not a permit. An experiment is not an adoption. An adoption is not authority for arbitrary action.

That is ADL's working definition of controlled cognition: not a mind with no freedom to explore, but an architecture in which exploration does not erase accountability.

## Repository Sources

- [`docs/explainers/GODEL_AGENTS.md`](../../../../../explainers/GODEL_AGENTS.md)
- [`docs/milestones/v0.86/features/GODEL_HADAMARD_BAYES_ALGORITHM.md`](../../../../v0.86/features/GODEL_HADAMARD_BAYES_ALGORITHM.md)
- [`docs/milestones/v0.89/features/GODEL_EXPERIMENT_SYSTEM.md`](../../../../v0.89/features/GODEL_EXPERIMENT_SYSTEM.md)
- [`docs/adr/0008-godel-stage-loop-v08.md`](../../../../../adr/0008-godel-stage-loop-v08.md)
- [`docs/milestones/v0.92/features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md`](../../../features/FIRST_TRUE_GODEL_AGENT_BIRTHDAY_v0.92.md)

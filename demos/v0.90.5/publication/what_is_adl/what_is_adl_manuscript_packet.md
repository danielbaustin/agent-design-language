# Manuscript Packet: What Is ADL?

## Metadata

- Skill mode: `draft_from_source_packet`
- Issue: `#2639`
- Source packet: `demos/v0.90.5/publication/what_is_adl/what_is_adl_source_packet.md`
- Status: first serious draft for internal review
- Submission state: not submitted; not publication-ready

## Working Title

**What Is ADL? A Deterministic Cognitive Runtime and Reviewable Control Plane
for Agent Systems**

## Alternate Titles

1. What Is ADL? Building Agent Systems That Can Survive Review
2. Agent Design Language: Deterministic Runtime Truth for AI Workflows
3. From Prompt Theater to Runtime Truth: The Architecture of ADL

## Abstract

Agent systems are increasingly asked to perform work whose value depends not
only on output quality, but on whether the work can be inspected after the
fact. Engineering teams need to know what was intended, what ran, what failed,
what was retried, what policy boundaries intervened, and which artifacts justify
the resulting claim. Much contemporary agent practice still relies on prompt
choreography whose execution truth is difficult to reconstruct. Agent Design
Language (ADL) addresses this problem by treating agent work as a bounded
engineering substrate rather than an improvisational chat ritual. In the
current repository, ADL combines a deterministic Rust runtime, explicit
workflow and task contracts, a repository-scale control plane, trace and
artifact truth surfaces, bounded cognitive and adaptive-runtime layers, and a
review discipline tied to milestone proof. This paper explains what ADL is now:
not a generic schema set, not a benchmark claim, and not a finished theory of
digital personhood, but a real system for making agent execution legible,
governed, and reviewable.

## Keywords

- agent runtime
- deterministic orchestration
- reviewable AI systems
- workflow control plane
- bounded cognition
- governed adaptation

## Draft

### 1. Introduction

The modern agent conversation is crowded with systems that appear impressive in
demo form but become slippery under inspection. A session may produce useful
text or code, yet still leave unanswered the questions that matter most to an
engineering organization: What exactly was the intended workflow? Which steps
actually ran? Which branch of reasoning was selected? What artifacts were
produced? Which failures were bounded and which were silently swallowed? Where
did policy or risk constraints intervene? What can a reviewer verify without
access to private chat state or institutional memory?

ADL begins from the view that these questions are not secondary. They are part
of the product boundary. If an agent system cannot explain what happened in a
replayable, inspectable way, then it remains difficult to trust even when it
occasionally performs well. The central ADL move is therefore architectural:
represent intent, execution, adaptation, and review as explicit surfaces rather
than hidden model temperament or oral tradition.

This paper presents ADL as it actually exists in the repository today. It is a
deterministic runtime and control plane for reviewable agent work. It includes
a real Rust runtime and CLI, explicit task/workflow/provider/tool artifacts, a
structured issue-to-worktree-to-PR control plane, trace and artifact truth
surfaces, milestone proof packages, bounded cognitive and adaptive layers, and
a growing security model that assumes intelligent opposition as a normal
condition. The paper is intentionally strong in architectural claim and careful
in empirical claim. It does not argue that ADL is already the best agent system
or that every later philosophical layer has landed. It argues something both
more modest and more important: ADL is a serious engineering substrate for
agent execution that can survive review.

### 2. ADL As Runtime, Control Plane, And Review System

ADL is not only a notation for describing agents. It is a system with three
mutually reinforcing layers.

The first layer is the runtime. Providers, tools, agents, tasks, runs, and
workflows are represented as typed structures. Workflows are compiled into
explicit execution plans, then executed through bounded runtime semantics that
record scheduling, step execution, failures, and outputs. This layer answers
the question: what did the machine actually attempt?

The second layer is the control plane. ADL treats repository change as a
disciplined lifecycle rather than an unstructured human habit. GitHub issues
act as work intents. STP, SIP, and SOR cards define the bounded task contract,
the input-state packet, and the output record. Work is executed in issue-bound
worktrees, validated before publication, and carried through draft PR and
closeout flows. This layer answers the question: how was the work authorized,
executed, reviewed, and integrated?

The third layer is the review and proof layer. Milestone packages, demo
matrices, architecture docs, review packets, and artifact surfaces all exist so
that a claim about the system is not merely stated but inspectable. In ADL, a
feature is not considered fully real because it sounds coherent in prose. It is
considered real when there is a bounded runtime surface, proof surface, or
reviewable artifact set that justifies the claim.

Taken together, these layers define ADL as a runtime plus an epistemology of
runtime truth. The project is not content to generate outputs. It attempts to
make those outputs attributable to declared inputs, bounded execution, and
visible review surfaces.

### 3. Deterministic Execution And Artifact Truth

ADL's most important engineering commitment is determinism where determinism is
appropriate, and explicit explanation where it is not. A workflow is compiled
into an acyclic execution plan. Dependency resolution, duplicate-step
rejection, cycle rejection, and saved-state validation happen before execution.
Sequential and concurrent structure are explicit rather than hidden in prompt
text. Retry, failure, provider, and delegation policies are modeled at the
runtime layer rather than inferred after the fact.

Execution produces more than a success or failure bit. ADL records lifecycle
phases, scheduling policy, prompt-assembly hashes, step starts, streaming
output events, delegation decisions, nested workflow calls, and completion or
failure events. Artifacts are written to deterministic run-scoped locations.
The result is that a reviewer can distinguish between what was planned, what
was attempted, and what was actually emitted.

This artifact discipline matters because most AI workflow failure is not pure
model failure. It is truth failure. Teams are often left with merged code,
stale issue state, ambiguous validation status, or inconsistent stories about
what happened in the run. ADL treats traces, run manifests, review packets, and
closeout records as first-class product surfaces precisely so that postmortem
analysis is possible.

In this sense, ADL is not merely a workflow runner. It is a system for creating
evidence-bearing agent execution.

### 4. The Cognitive Stack: Instinct, Arbitration, Freedom Gate, And AEE

One of ADL's distinguishing features is that its cognitive story is bounded and
inspectable. The project does not begin with an unbounded claim about
consciousness or general agency. It begins with a loop that can be represented,
audited, and gradually deepened:

`instinct -> affect -> arbitration -> freedom_gate -> execution (AEE) ->
evaluation -> reframing -> memory`.

This matters because it turns "cognition" from a poetic label into an
architectural contract. Instinct provides fast priors and default tendencies.
Affect weights salience, urgency, and persistence pressure. Arbitration decides
whether the system should route to faster or slower reasoning, defer, refuse,
or reframe. Freedom Gate applies substrate constraints before commitment.
Execution happens through AEE. Evaluation and reframing then determine whether
the frame was adequate, whether progress occurred, and what should be recorded
to memory.

ADL's AEE line is especially important. In many agent systems, persistence is
just a prettier name for retry. ADL's convergence model instead demands visible
progress signals, bounded stop conditions, stable termination classes, and
reviewable strategy changes. Convergence, stall, bounded-out, policy-stop, and
handoff states are all part of the declared surface. This is a serious design
choice: it makes continuation a governed act rather than a stylistic flourish.

Freedom Gate plays a similarly central role. In ADL, the gate is not a moral
mood or a politeness preference. It is a substrate judgment boundary. The gate
can allow, defer, refuse, or escalate. Constraint is therefore supposed to live
in the runtime substrate and its records rather than in the transient
temperament of one model invocation. That makes refusal and escalation
legitimate outcomes of the system rather than embarrassing exceptions to be
explained away.

### 5. ADL's Adaptive Ambition: Godel Agents, AEE, And Experiment Evidence

ADL is not satisfied with static workflow execution alone. It also aims toward
bounded adaptive systems that can compare alternatives, justify improvements,
and accumulate structured evidence about what should change. That ambition is
visible in the bounded Godel and experiment-system surfaces already present in
the repository.

The important point is what kind of adaptive story ADL is telling. It is not
claiming unconstrained recursive self-improvement. It is not presenting a
mythic self-modifying agent that can silently rewrite itself into superiority.
Instead, it treats adaptation as an experiment discipline. Baseline and variant
pairing, bounded mutations, evaluation plans, adoption and rejection decisions,
and durable experiment evidence are all explicit surfaces.

This is one of the most intellectually important ADL design choices. If a
system improvement claim is real, it should become an artifact. If a mutation
was attempted, reviewers should be able to inspect the evaluation plan. If a
variant was adopted, the adoption should be a governed decision rather than a
vibe. That is why the existing Godel experiment package matters: it provides a
bounded home for learning and self-improvement that remains legible under
review.

The same philosophy helps explain why ADL cares about AEE, memory, and
convergence as part of the adaptive stack. Adaptation in ADL is not meant to be
a free-floating optimization urge. It is meant to be a constrained process that
produces inspectable records about why another step, another frame, or another
variant was justified.

### 6. Trust Under Adversary And The Security Model

Another place where ADL departs from casual agent practice is in its treatment
of opposition. ADL explicitly adopts the premise that valuable systems should
expect sustained intelligent pressure. Under that premise, trust cannot remain
a vague assumption imported from normal-path operation. It must be represented
as part of the runtime story.

The project therefore distinguishes trusted and reduced-trust surfaces under
contested conditions. Revalidation requirements, escalation paths, and reviewer
visibility are declared explicitly. The attack surface is modeled as dynamic
rather than static. This means that threat is understood as interaction over
state, time, interface availability, and bounded authority rather than as a
one-time checklist of known holes.

This security framing is important for two reasons. First, it is honest. Modern
agent systems live amid continuous automated reasoning pressure, and architectures
that do not acknowledge this will age badly. Second, it aligns security with
the rest of ADL's philosophy: adversarial claims should be visible, bounded,
traceable, and attributable.

Here again, ADL's emphasis is architectural rather than triumphant. The current
repository does not claim that every adversarial lane is complete. It claims
that the contested-runtime model, trust-language, and reviewer-facing
guarantees are explicit enough that later red, blue, purple, exploit, and
continuous-verification work can be judged against them.

### 7. Why ADL Is Different

ADL is different from a schema-only language because it has a real runtime,
real artifacts, and a real control plane. It is different from an
output-centered prompt workflow because it insists on execution truth,
validation, and review surfaces. It is different from an agent playground
because its strongest commitments are to boundedness, explicit contracts,
milestone proof, and architectural legibility.

It is also different in the way it combines cognition, runtime, and governance.
The cognitive stack is not floating above execution. It is wired into
arbitration, Freedom Gate, AEE, memory, and evaluation. The control plane is
not only project management. It is a substrate for making repository work
bounded and auditable. The security model is not stapled on from the outside.
It is part of the runtime claim about what a trustworthy agent system must make
visible.

Perhaps the shortest accurate description is this: ADL is an attempt to turn
agent work into a form of software engineering that remains inspectable even
when the system reasons, adapts, delegates, and operates under contest.

### 8. Current Boundaries, Limits, And Future Work

Because ADL is ambitious, it is important to say clearly what this paper does
not claim.

It does not claim that ADL has already completed every later layer in its own
roadmap. Identity-bearing citizens, full governance, richer wellbeing and moral
cognition, full economic or multi-polis society, and the first true
Gödel-agent birthday are all explicitly later or partially bounded lines.
Similarly, this paper does not claim empirical superiority over other agent
frameworks, elimination of error, or final correctness of every cognitive
construct used in ADL's internal vocabulary.

What it does claim is that ADL has crossed an important threshold. It is now a
substantial and coherent system with a deterministic runtime, a reviewable
control plane, bounded cognitive and adaptive surfaces, and an adversarially
aware trust posture. That is enough to justify serious technical attention.

Future work naturally follows from the system's own decomposition. One paper
can focus on determinism and trust in the runtime. Another can focus on Godel
agents and the experiment system. Another can develop the Cognitive Spacetime
Manifold as the larger architectural substrate. Still another can connect ADL
to broader intelligence and state-space-compression theory. The point of this
paper is not to absorb all of those manuscripts. It is to establish the front
door through which they become legible.

### 9. Conclusion

The most common failure mode in the current agent landscape is not lack of
flair. It is lack of truth. Systems are often impressive on contact and hard to
audit after impact. ADL proposes a different standard. Agent execution should
be explicit enough to inspect, bounded enough to govern, and durable enough to
review.

That standard has now become concrete in the repository. ADL is a deterministic
runtime and control plane for agent work; a cognitive stack whose boundaries
are inspectable; an adaptive substrate that prefers experiment evidence over
self-improvement mythology; and a security posture that treats adversarial
pressure as normal rather than exceptional. Whether one ultimately agrees with
every design choice, the project now stands as a real systems answer to the
question posed by its title.

## Claim Boundary Report

| Claim | Label | Notes |
| --- | --- | --- |
| ADL is a deterministic runtime and control plane with a real Rust implementation. | SUPPORTED | Grounded in `README.md`, feature list, and architecture doc. |
| ADL already includes a bounded cognitive stack, AEE convergence, and Freedom Gate substrate. | SUPPORTED | Grounded in `v0.86` and `v0.89` feature docs. |
| ADL treats adversarial pressure as a first-class runtime condition. | SUPPORTED | Grounded in `v0.89` and `v0.89.1` security/runtime docs. |
| ADL is categorically better than other agent systems. | REMOVE_OR_WEAKEN | No comparative evidence packet yet. |
| ADL has already delivered full identity, governance, and social-cognition completion. | REMOVE_OR_WEAKEN | Explicitly beyond current bounded claims. |
| ADL offers a novel architecture relative to all prior systems. | NEEDS_CITATION | Plausible, but requires related-work support. |

## Citation Gap Report

No external citations are invented in this packet. Before public submission, the
paper needs a focused related-work pass on:

- workflow/orchestration systems
- agent frameworks and tool-use runtimes
- provenance and artifact-traceability systems
- replayable and reviewable software execution
- broader cognitive-control and adaptive-agent literature

## Reviewer Questions

1. Is the launch-facing definition of ADL crisp enough, or should the title and
   opening sections lean even harder into "runtime truth"?
2. Should the paper describe AEE and Freedom Gate as core ADL differentiators
   this early, or keep more of that weight for follow-on papers?
3. Does the manuscript balance boldness and caution correctly around Gödel
   agents and adversarial posture?
4. Which sections most need concrete examples or figure support before the next
   revision?

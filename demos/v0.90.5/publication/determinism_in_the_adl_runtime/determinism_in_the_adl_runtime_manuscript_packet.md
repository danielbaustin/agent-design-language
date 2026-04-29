# Manuscript Packet: Determinism in the ADL Runtime

## Metadata

- Skill mode: `draft_from_source_packet`
- Issue: `#2641`
- Source packet: `demos/v0.90.5/publication/determinism_in_the_adl_runtime/determinism_in_the_adl_runtime_source_packet.md`
- Status: first serious draft for internal review
- Submission state: not submitted; not publication-ready

## Working Title

**Determinism in the ADL Runtime: Execution Truth for Reviewable Agent Systems**

## Alternate Titles

1. Determinism in the ADL Runtime
2. Execution Truth, Replay Posture, and Reviewability in ADL
3. Why ADL Treats Determinism as a Trust Primitive

## Abstract

AI and agent systems often fail not only because of incorrect outputs, but
because their execution cannot be reconstructed clearly enough for review.
Agent Design Language (ADL) addresses this by treating determinism as a trust
primitive rather than a secondary optimization. In the current repository, ADL
uses deterministic plan compilation, explicit scheduler and dependency
semantics, structured traces, deterministic artifact paths, bounded provider
and remote-execution contracts, and explicit control-plane closeout records to
make execution legible. This paper explains what determinism means in ADL, why
it matters for runtime trust, and where the boundary lies between deterministic
structure and stochastic model behavior. The claim is not that ADL offers
perfect replay or formal proof of correctness. The claim is that ADL provides a
substantially stronger execution-truth substrate than prompt-only orchestration
by ensuring that what happened can be inspected, challenged, and debugged as
repository evidence.

## Draft

### 1. Why Determinism Matters In Agent Systems

Determinism matters in ordinary software because reproducibility improves
debugging, testing, and operational trust. It matters even more in agent
systems because those systems are often asked to reason, delegate, and call
tools in ways that are difficult to inspect after the fact. If the structure of
execution is unstable, then the surrounding review and governance claims become
unstable too.

Many contemporary agent workflows still depend on hidden orchestration logic:
prompt ordering, implicit retries, undeclared fallback behavior, or manual
operator interpretation of what probably happened. This creates a familiar
failure pattern. A result may appear useful, but the team still cannot explain
the run with enough precision to know whether the success is durable, whether
the failure can be reproduced, or whether a later regression has occurred.

ADL treats this as a systems problem, not merely a modeling problem. The system
must make execution legible. Determinism, in this context, is not an aesthetic
preference. It is part of the trust boundary between the runtime and the people
who must review it.

### 2. What ADL Means By Determinism

ADL does not use the word determinism to mean that every token emitted by every
model backend is guaranteed to be identical forever. That would be both
unrealistic and, in many practical settings, false. Instead, ADL's
determinism claim is layered.

At the structural level, the same schema-valid documents and configuration
should compile into the same plan. Dependency resolution, rejection of invalid
graphs, step identity, scheduler ordering rules, artifact layout, and control
surfaces should remain stable for identical inputs. The runtime should not
silently improvise orchestration logic.

At the review level, major control transitions should be explicit. If retries,
failures, delegation decisions, or policy boundaries occur, they should become
trace-visible and artifact-visible rather than living only in hidden runtime
state.

At the control-plane level, determinism also means that issue intent, execution
packet, validation, and closeout records stay aligned. A merged PR with stale
cards is a truth failure even if the code compiles. ADL therefore extends the
determinism story beyond the runner into the surrounding lifecycle that tells a
reviewer what happened.

### 3. Deterministic Planning And Bounded Execution

ADL grounds its determinism posture in the execution plan. Providers, agents,
tasks, workflows, and steps are parsed into typed structures. The planner then
builds an acyclic execution graph whose dependencies and order are explicit.
Duplicate step ids, duplicate saved-state keys, unknown saved-state references,
self-dependencies, and cycles are rejected before runtime execution begins.

This is not a small detail. It means that the runtime starts from a known,
checkable structure rather than from a prompt narrative about structure. The
runtime can then apply explicit scheduler semantics, including ready-step
ordering rules and bounded concurrency behavior, without leaving the reviewer to
guess why one step happened before another.

The accepted determinism ADR makes this concrete: ready-step ordering is
lexicographic by full step id, plan generation is deterministic for
canonicalized inputs, and bounded execution preserves stable output ordering.
These are intentionally engineering-facing guarantees. They sacrifice some
throughput opportunism in exchange for a more stable debugging and review
surface.

### 4. Trace And Artifacts As Execution Truth

Determinism would be of limited value if the system did not expose what it was
doing. That is why ADL treats trace and artifact surfaces as part of the same
truth model. Trace is not a narrative convenience layered on afterward. It is a
structured record of lifecycle, scheduling, control decisions, and execution
events. Artifacts carry the heavier payload truth referenced by trace.

This division is deliberate. Trace alone cannot reconstruct all payloads.
Artifacts alone cannot explain causality. Together they form the reconstruction
surface that reviewers, operators, and later automation can use to understand a
run.

The most important consequence is epistemic rather than aesthetic: if a
reviewer must infer a major transition, the truth surface is incomplete. ADL's
trace architecture therefore aims to preserve explicit lifecycle boundaries,
decision visibility, provider attribution, stable artifact references, and
deterministic structural ordering.

### 5. Determinism Beyond The Runner

One of ADL's more unusual claims is that runtime determinism is undermined if
the surrounding control plane is sloppy. A build may produce stable artifacts,
yet still leave the project in a non-reconstructable state if issue closure,
validation records, or worktree cleanup are ambiguous.

That is why the ADL control plane uses STP, SIP, and SOR as canonical packet
surfaces and enforces an issue -> worktree -> validation -> PR -> closeout
lifecycle. The output record must say what was run, what was intentionally not
run, what artifacts exist, and what integration state is real. The closeout
stage then reconciles GitHub state, local cards, and worktree cleanup.

This is an important extension of the determinism concept. ADL does not treat
execution truth as something that ends when the binary exits. It treats the
whole bounded engineering path as part of the proof surface. If the runtime is
deterministic but the review records drift, then the system has still failed to
be fully trustworthy.

### 6. Replay, Security, And Bounded Trust

ADL also links determinism to replay posture and security posture. Replay in
ADL is not framed as a magical guarantee that every external dependency will
always behave identically. Instead, it is framed as a disciplined attempt to
make structure, references, artifacts, and trust boundaries explicit enough
that a run can be reconstructed credibly.

This is why provider identity is kept separate from transport identity and why
bounded remote execution surfaces preserve local scheduler ownership. It is why
write-path validation and signing surfaces matter. It is why absolute host-path
leakage and undeclared side effects are treated as violations of the review
surface.

The security story fits naturally into the same frame. A system under
adversarial pressure cannot rely on hand-wavy narratives about what probably
happened. It needs attributable, traceable, policy-bounded runtime behavior.
Determinism does not solve security on its own, but it provides a substrate on
which security review becomes materially more credible.

### 7. What ADL Does Not Claim

This paper must stay disciplined about its boundary. ADL does not claim formal
verification of the full runtime. It does not claim that all provider outputs
are semantically identical under replay. It does not claim that every later
milestone layer is fully deterministic in the strongest mathematical sense. Nor
does it claim superiority over every competing framework without a comparison
study.

What it does claim is that determinism in agent systems is most valuable when
it is connected to review. Stable plan generation, bounded scheduler behavior,
explicit traces, deterministic artifact roots, and truthful closeout records
make a system more inspectable, more debuggable, and easier to trust than one
that leaves orchestration hidden.

### 8. Conclusion

In ADL, determinism is not a boast about perfection. It is a design discipline
for making runtime behavior legible enough to review. The runtime compiles
structured inputs into stable execution plans. The scheduler applies explicit
ordering and bounded concurrency rules. Trace and artifacts preserve the causal
record. The control plane extends truthfulness into validation, PR, and
closeout surfaces. Together, these choices turn determinism into a practical
trust primitive for agent systems.

That is the real contribution of the ADL runtime. It does not ask reviewers to
believe a story about what happened. It tries to leave behind enough structured
evidence that the story can be checked.

## Claim Boundary Report

| Claim | Label | Notes |
| --- | --- | --- |
| ADL treats deterministic planning and ordering as an accepted runtime contract. | SUPPORTED | Grounded in ADR 0001 and architecture docs. |
| Trace and artifacts together provide the execution-truth surface. | SUPPORTED | Grounded in trace and architecture docs. |
| Control-plane closeout truth is part of the determinism story. | SUPPORTED | Grounded in workflow and architecture docs. |
| ADL offers perfect replay of all runs including stochastic model outputs. | REMOVE_OR_WEAKEN | Unsupported and too strong. |
| ADL is formally verified. | REMOVE_OR_WEAKEN | Unsupported. |
| ADL is more deterministic than all alternatives. | NEEDS_CITATION | Comparative evidence required. |

## Citation Gap Report

No external citations are invented in this packet. Before public submission, the
paper still needs related work on:

- deterministic workflow and orchestration systems
- replay and provenance systems
- reproducibility/debugging literature
- adjacent agent runtime and orchestration platforms

## Reviewer Questions

1. Should this paper include one concrete end-to-end example of a run packet
   and closeout record, or keep the first draft more architectural?
2. Is the control-plane extension of determinism persuasive, or does it need a
   shorter, sharper argument?
3. Should the final title stay narrow on runtime determinism, or mention
   execution truth explicitly?

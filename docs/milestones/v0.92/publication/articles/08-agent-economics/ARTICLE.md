# Agent Economics

An agent that lives for one request can borrow resources from an application budget and disappear. A persistent population of agents cannot avoid the question of allocation.

Who receives compute when demand exceeds supply? Which memories deserve durable storage? Whose task gets attention first? How much bandwidth may one actor consume? What happens when urgency, fairness, risk, and cost point in different directions?

These are economic questions, even when no currency is involved.

ADL places them inside the **polis**: the governance, security, social, and economic layer of its runtime world. The project treats resource limits and evidence-bearing cost as governance concerns across current runtime and planning surfaces, but its richer market designs remain proposals. That distinction creates room to explore the subject without pretending the answers are settled.

## Four scarce goods

ADL's retained Runtime v2 economic source draft, historical lineage rather than implemented allocation, identifies four basic resource classes.

**Compute** includes model inference and execution cycles. It is the most visible cost, but not the only one.

**Memory** includes durable storage and retrieval bandwidth. Long-lived agents create pressure not merely to store more, but to decide what remains available and at what fidelity.

**Attention** includes scheduling priority and access to shared services. In a multi-agent system, attention determines which goals advance and which wait.

**Bandwidth** includes communication among agents and access to tools or external APIs. It affects both cost and the rate at which an agent can influence its environment.

Treating these as explicit resources changes system design. A runtime can record allocation, refusal, consumption, and remaining capacity. An agent can reason about alternatives when a preferred route is too expensive. Reviewers can see whether resource policy shaped an outcome.

## Budgets are part of agency

A budget is often described as a restriction. It is also a capability envelope.

An agent with a clear resource grant can choose among strategies inside known limits. It can estimate whether a large model call, extended search, additional experiment, or memory retrieval is justified. It can stop or seek approval before exhausting shared capacity.

This is preferable to hidden scarcity. When limits are implicit, agents encounter timeouts, throttling, or arbitrary termination without a model of why. Operators see costs after the fact. Other agents experience starvation with no visible policy.

Explicit budgets do not make allocation fair, but they make the choice inspectable.

## Price is one signal, not the sovereign

The historical design explores auctions, fixed prices, hybrid markets, bidding, and trade. Those mechanisms could help allocate scarce compute or attention, particularly when participants have different priorities and information.

But a market is not a neutral answer. The initial allocation shapes who can bid. A wealthy or highly rewarded agent can monopolize resources. Short-term prices may undervalue maintenance, safety, memory integrity, or the needs of less powerful participants.

ADL's own planning therefore allows policy, risk, and fairness to override a simple “highest bid wins” rule. Economic signals can feed cognitive arbitration without becoming the only form of value.

This is especially important for agents with delegated rather than intrinsic budgets. Spending power originates in human or institutional authority. A bid cannot legitimize an action that policy forbids.

## Memory has an economy

Persistent memory makes the allocation problem unusually personal.

Storage is finite. Retrieval has cost. Compression loses detail. Forgetting may improve performance in one context while damaging continuity in another. A system needs criteria for retaining evidence, summaries, commitments, relationships, and obsolete state.

If memory is allocated only by immediate utility, an agent may lose the history required to explain itself. If everything is retained forever, privacy, cost, and relevance deteriorate.

An evidence-aware memory economy would therefore distinguish raw artifacts, derived summaries, identity-critical records, private material, and disposable working state. Deletion and compression would be governed transitions rather than silent housekeeping.

ADL has architecture for evidence and continuity, but the long-run economics of memory remain an open research problem.

## Inter-polis exchange is later work

Once multiple governed worlds can interact, new questions appear. Can one polis buy compute from another? Which authority applies to a cross-boundary task? How are exchange, dispute, provenance, and resource transfer recorded?

ADL's planning requires explicit traces and governance on both sides of any future inter-polis exchange. It also defers payment and market implementation. There is no current claim of a deployed token economy, payment network, or x402 integration.

That restraint is healthy. Introducing money before identity, authority, dispute, security, and evidence boundaries are mature would multiply ambiguity rather than create a functioning economy.

## Economics as pressure on cognition

Resource limits influence what an agent thinks about and attempts. A costly strategy may be deferred. Scarcity may raise the value of a compact experiment. A safety-critical task may receive priority despite a poor economic return. A fair scheduler may preserve capacity for actors that would lose every auction.

Economics therefore sits inside cognition and governance, not beside them. It provides pressure, signals, and constraints. Policy supplies values that price alone cannot express. Evidence lets the system explain how the tradeoff was made.

ADL does not yet know the final mechanism. Neither does the broader field. But persistent agents make the question unavoidable: intelligence consumes shared resources, and the rules for allocating those resources become part of the society we are building.

## Repository Sources

- [`docs/milestones/v0.90.1/ideas/source_runtime_v2/ECONOMIC_AND_RESOURCE_MODEL.md`](../../../../v0.90.1/ideas/source_runtime_v2/ECONOMIC_AND_RESOURCE_MODEL.md)
- [`docs/milestones/v0.88/features/ADL_COST_MODEL.md`](../../../../v0.88/features/ADL_COST_MODEL.md)
- [`docs/milestones/v0.90.4/ideas/PAYMENT_AND_INTERPOLIS_DEFERRAL.md`](../../../../v0.90.4/ideas/PAYMENT_AND_INTERPOLIS_DEFERRAL.md)
- [`docs/explainers/CSM.md`](../../../../../explainers/CSM.md)
- [`docs/milestones/v0.92/README.md`](../../../README.md)

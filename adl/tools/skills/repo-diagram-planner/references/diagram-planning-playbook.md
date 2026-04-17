# Diagram Planning Playbook

Use this playbook when selecting diagram tasks from CodeBuddy evidence.

## Diagram Family Heuristics

- Use `system_context` when the evidence has top-level docs, user-facing
  README, or architecture overview surfaces and the audience needs orientation.
- Use `container_or_component` when architecture review evidence mentions
  modules, packages, services, components, adapters, tools, or runtime
  boundaries.
- Use `workflow` when evidence mentions lifecycle, queue, run, closeout,
  conductor, planner, review, or issue handoff behavior.
- Use `sequence` when evidence includes actor-to-system or service-to-service
  interactions over time.
- Use `state` when evidence includes issue states, runtime states, lifecycle
  states, cache states, or long-lived-agent cycles.
- Use `data_flow` when evidence includes trust boundaries, secrets, redaction,
  uploads, network calls, external APIs, or customer data movement.
- Use `dependency_graph` when evidence includes manifests, package managers,
  module graphs, skill relationships, imports, or dependency review artifacts.
- Use `responsibility_map` when evidence includes multiple agents, specialists,
  skills, handoffs, or review-lane ownership.

## Backend Heuristics

- Mermaid: first choice for GitHub issues, PRs, Markdown docs, and lightweight
  review artifacts.
- Structurizr DSL: use for C4 families or multiple related architecture views.
- D2: use for polished system maps or demo visuals when GitHub-native rendering
  is less important.
- PlantUML: use for strict UML sequence, component, deployment, or state models.
- Markdown outline: use when evidence supports a diagram need but not diagram
  source yet.

## Quality Bar

Good diagram tasks are:

- evidence-bound
- audience-specific
- narrow enough for one `diagram-author` run
- explicit about assumptions and unknowns
- clear about claims that must not be made
- honest about renderer expectations

Reject or block tasks that require guessing architecture, security boundaries,
data flows, deployment topology, or dependency direction from weak evidence.


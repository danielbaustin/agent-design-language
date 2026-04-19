# PR Stack Manager Playbook

## Detection Workflow

1. Resolve the target issue/worktree or branch context.
2. Read task surfaces and doctor state for the target:
   - source prompt
   - STP, SIP, SOR
   - local worktree metadata
   - PR metadata when available
3. Build deterministic stack topology:
   - branch ancestry from `codex/<issue>-...` naming
   - dependency declarations from `depends_on`
   - observed PR base chain from local and remote metadata
4. Compare expected dependency graph with actual stack/base truth.
5. Detect drift types:
   - base drift (base branch no longer expected),
   - ordering conflict (child branch claims dependency not present),
   - stale dependency claim (depends_on references merged or missing nodes),
   - open blocker risk (unmerged dependency PR still required).
6. Emit findings with severity and deterministic evidence.
7. Produce a plan-first repair list:
   - safe ordering sequence,
   - explicit prerequisite actions,
   - mutation-safe command(s) only when requested and deterministic.

## Planning Heuristics

- Prefer no mutation when stack truth is contradictory or evidence is partial.
- Prefer `status: blocked` only when an ambiguous parent/child dependency exists.
- Treat missing issue intent as `ambiguous` rather than inventing order.
- Never force branch surgery.
- Never infer PR targets without explicit PR number + base evidence.

## Repair Policy

Mutation is allowed only in plan/apply modes and when:

- dependency edges are explicit and unique,
- base expectation can be proven from issue graph or direct evidence,
- and actions are bounded to the target scope.

Safe actions include:
- emitting an execution-order plan,
- annotating ordering recommendations on SOR/local surfaces,
- preparing deterministic, scoped follow-up steps for follow-on PR stack execution.

Unsafe actions include:
- cross-stack forced rebases,
- rewriting unrelated PRs,
- changing issue intent or acceptance criteria,
- silent branch deletion/recreation.

## Validation

Before finalizing, validate:
- topology reconstruction is deterministic,
- action list is scoped and bounded,
- mutation commands are explicit and reversible where possible,
- all emitted paths are repository-relative and non-absolute.

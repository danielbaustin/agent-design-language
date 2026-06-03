# CLI Wrapper Migration Contract (#3597)

## Summary

This contract defines how the C-SDLC wrapper, conductor, prompt templates,
skills, generated cards, and portable project adapter migrate while the
`adl-csdlc` compatibility binary becomes the implementation owner.

The migration rule is conservative: `adl-csdlc` may own implementation routing,
but `adl/tools/pr.sh` remains the canonical agent-facing issue-work entrypoint
until a later tracked issue changes the public workflow spine.

## Source Inputs

- Issue `#3597`: wrapper migration contract for `pr.sh` and conductor.
- Issue `#3596`: introduced the `adl-csdlc` compatibility binary.
- `docs/milestones/v0.91.5/CLI_COMMAND_INVENTORY_3594.md`: command inventory
  and generated-card policy.
- `AGENTS.md`: repo-local workflow contract.
- `docs/templates/PR_INIT_INVOCATION_TEMPLATE.md`: caller-facing issue
  bootstrap template.
- `docs/templates/prompts/current.json`: active prompt-template registry.
- `adl/tools/skills/workflow-conductor/SKILL.md`: lifecycle routing contract.
- `adl/tools/skills/workflow-conductor/scripts/route_workflow.py`: conductor
  dispatch implementation.
- `adl/tools/skills/pr-init/SKILL.md`: issue bootstrap contract.
- Private planning source: `.adl/docs/TBD/ADL_CLI_DECOMPOSITION_PLAN.md`.
- Portable adapter planning source:
  `.adl/docs/TBD/PORTABLE_ADL_PROJECT_ADAPTER_PLAN.md`.

The private `.adl/docs/TBD/` files are design inputs only. Reviewable milestone
truth for this issue lives in this tracked document.

## Migration Spine

| Surface | Current canonical role | Migration rule |
|---|---|---|
| `adl/tools/pr.sh` | Agent-facing issue wrapper for create/init/run/doctor/finish/closeout. | Remains canonical for tracked issue work until a tracked migration issue updates `AGENTS.md`, prompt templates, skills, portable adapters, and generated-card validation together. |
| `workflow-conductor` | Lifecycle router that selects the next skill and dispatches bounded lifecycle commands. | Continues dispatching built-in lifecycle commands through `bash adl/tools/pr.sh ...` until wrapper equivalence is proven and the conductor contract is updated. |
| `adl-csdlc issue ...` | New C-SDLC compatibility binary and future implementation owner. | May be used by wrapper internals and tests, but is not yet the primary command taught to agents for issue work. |
| `adl pr ...` | Rust compatibility surface under the legacy `adl` umbrella. | Remains a shim while the split proceeds; generated cards should not prefer it over `adl/tools/pr.sh`. |
| prompt templates and generated cards | Durable C-SDLC state and operator instructions. | Continue using `adl/tools/pr.sh run <issue>` for issue binding until the generated-card command policy is explicitly changed. |
| card editor skills | Lifecycle-truth repair surfaces. | Do not become command routers and do not teach alternate workflow entrypoints by themselves. |
| portable project adapter | External-repo discovery and state contract. | Must resolve ADL tooling through `ADL_HOME` or declared config, and must not copy or fork the workflow spine. |

## Wrapper Delegation Contract

`adl/tools/pr.sh` is the public issue-work wrapper during this migration.

It may delegate internally to Rust command owners when tests prove equivalence,
but the wrapper must preserve:

- command names accepted by current cards and skills;
- exit status for successful and failing lifecycle commands;
- issue/worktree/card path behavior;
- dirty-main and wrong-checkout guardrails;
- open-wave and closeout guardrails;
- generated PR title/body/closing-linkage behavior;
- visible diagnostics for ambiguous `run` invocations.

Any wrapper delegation change must include old/new command equivalence tests
before public docs or prompt templates switch to a new primary command string.

## Workflow-Conductor Route Contract

`workflow-conductor` remains the lifecycle router.

During this migration, built-in dispatch commands must keep using
`adl/tools/pr.sh` for lifecycle execution:

- `pr-init` dispatches `bash adl/tools/pr.sh init ...`;
- `pr-ready` dispatches `bash adl/tools/pr.sh doctor ...`;
- `pr-run` dispatches `bash adl/tools/pr.sh run ...`;
- `pr-closeout` dispatches `bash adl/tools/pr.sh closeout ...`.

The conductor may mention `adl-csdlc` only as an implementation-owner or
future-route concept until dispatch equivalence is tested and the skill contract
is updated in a tracked issue.

## Generated-Card Command Policy

New generated C-SDLC cards must keep using the current issue-work wrapper:

```text
adl/tools/pr.sh run <issue>
```

They must not introduce:

- `adl pr run <adl.yaml>` runtime-through-PR commands;
- `adl-csdlc issue run <issue>` as the primary agent-facing issue-binding
  command before wrapper migration is complete;
- `adl-csdlc` prompt-template commands as the primary generated-card command
  before docs, skills, and validators move together.

Generated-card validation is stricter than terminal shim warnings. Deprecated
or premature command strings in new cards should fail or create an explicit
validation finding even while human-facing terminal warnings remain opt-in.

## Skills And Template Migration Gate

Before any skill or template teaches `adl-csdlc` as the primary workflow
command, the migration issue must prove:

- wrapper delegation equivalence for the relevant command family;
- conductor dispatch compatibility;
- prompt-template sample and schema compatibility;
- generated-card validator behavior for deprecated commands;
- portable adapter discovery and state-policy alignment;
- closeout and PR publication behavior through the new route.

Until then:

- `AGENTS.md` keeps `adl/tools/pr.sh run <issue>` as the issue-binding command;
- `docs/templates/prompts/current.json` continues pointing at templates that
  preserve current wrapper truth;
- `pr-init` and `workflow-conductor` skills keep current wrapper references;
- external repos use the portable adapter to discover the ADL checkout and
  wrapper, not a copied `adl-csdlc` command recipe.

## Portable Adapter Alignment

External repositories must not invent a second process model while ADL CLI
decomposition is in progress.

The portable adapter contract should require:

- `ADL_HOME` or `adl_project.json` tooling discovery;
- explicit issue tracker authority;
- explicit state policy for cards, worktrees, and public evidence;
- prompt-template registry discovery from the resolved ADL checkout;
- repo-local `AGENTS.md` that forbids tracked issue work on `main`;
- fail-closed doctor behavior if the resolved ADL checkout lacks `pr.sh`,
  skills, templates, or required versions.

The first external-repo adapter should call the wrapper through resolved ADL
tooling. It should not vendor shell scripts or make `adl-csdlc` independently
canonical before this repository completes the wrapper migration.

## Validation Surface

This issue is complete when focused validation proves:

- the tracked contract exists and names `adl/tools/pr.sh` as canonical;
- workflow-conductor built-in dispatch still routes through `adl/tools/pr.sh`;
- root `AGENTS.md` still teaches `adl/tools/pr.sh run <issue>`;
- active prompt templates do not teach `adl-csdlc` as the primary issue-work
  command prematurely;
- generated-card templates do not emit deprecated runtime-through-PR commands;
- portable-adapter alignment terms are present in the contract.

No runtime behavior movement is required for this issue.

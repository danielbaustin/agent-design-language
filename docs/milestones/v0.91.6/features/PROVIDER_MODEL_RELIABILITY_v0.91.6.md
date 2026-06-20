# Provider And Model Reliability

## Metadata

- Feature Name: Provider And Model Reliability
- Milestone Target: `v0.91.6`
- Status: `wp_05_provider_sprint_completed_with_v2_reconciliation_follow_on`
- Owner: ADL maintainers
- Doc Role: primary
- Feature Types: policy, architecture
- Proof Modes: tests, review, replay
- Related issues: `#3970`, `#4007`, `#4008`, `#4009`, `#4010`, `#4011`, `#4012`, `#4053`, `#4097`, `#4111`
- Primary issue packets: `docs/milestones/v0.91.6/review/provider/PROVIDER_ROLE_SUITABILITY_MATRIX_4008.md`, `docs/milestones/v0.91.6/review/provider/PROVIDER_RELIABILITY_CLOSEOUT_MATRIX_4012.md`, `docs/milestones/v0.91.6/review/provider/WP05_PROVIDER_MINI_SPRINT_CLOSEOUT_3970.md`, `docs/milestones/v0.91.6/review/provider/C_SDLC_AGENT_SUITABILITY_PANEL_4097.md`, `docs/milestones/v0.91.6/review/provider/PROVIDER_PROFILES_V2_RECONCILIATION_4111.md`
- Upstream catalog dependency: `#4007` / PR `#4063` merged (`PROVIDER_CAPABILITY_PROFILE_CATALOG_4007.md`)

## Purpose

Define role-scoped provider/model suitability for reliable multi-agent operation
before `v0.92` consumes provider, watcher, or birthday-demo claims.

## Scope

In scope:

- hosted, local, remote, OpenRouter, and Gemma role lanes;
- role suitability for planning, coding, review, summarization, orchestration,
  watcher/janitor, local/offline, and constrained fallback use;
- explicit separation between provider infrastructure, model capability, and
  role-routing policy;
- evidence-status fields, known limits, and failure-mode routing;
- bounded consumption notes for later capability-based delegation work.

Out of scope:

- model training;
- Aptitude Atlas productization;
- broad benchmark product claims;
- identity, citizen, or institutional authority modeling.

## Required Decisions

- Which provider/model lanes are supported, useful with limits, blocked, or
  only inventory-known for each role?
- Which lanes are proven through native provider surfaces versus historical or
  comparative packets?
- Which watcher and reviewer claims remain bounded instead of broad?
- How should `v0.92` consume role routing without guessing at missing proof?

## Dependencies

- `#4007` provider/capability catalog split and its merged proof note.
- `docs/milestones/v0.91.5/features/PROVIDER_MODEL_MATRIX_v0.91.5.md`
- `docs/milestones/v0.91.5/features/MULTI_AGENT_C_SDLC_OPERATION_v0.91.5.md`
- `docs/milestones/v0.91.5/review/multi_agent_matrix/PROVIDER_MODEL_ROLE_MATRIX_2026-06-05.md`
- `docs/milestones/v0.91.5/review/openrouter_matrix/OPENROUTER_MATRIX_PROOF_2026-06-14.md`
- `docs/milestones/v0.91.5/review/remote_gemma_watcher/REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md`

## Provider/Capability Catalog Boundary

WP-05 begins with one explicit split:

- provider profiles answer what infrastructure-backed services are available
- capability profiles answer what an entity or lane can do with those services

This milestone must not collapse providers, models, capabilities, identities,
citizens, or institutions into one profile object just because later routing
consumes all of them.

### Provider profile contract

Provider profiles are infrastructure/service descriptors. They are not actor,
identity, or authority records.

Required provider-profile fields for WP-05:

| Field | Meaning |
| --- | --- |
| `provider_family` | Stable provider family or substrate such as `openai`, `anthropic`, `google`, `ollama`, `openrouter`, `mock`, or bounded `http` profile families. |
| `profile_id` | Deterministic profile identifier used by config and routing. |
| `service_kind` | Runtime service substrate such as `http`, `ollama`, or `mock`. |
| `default_model` | Default model or model family when the profile owns one. |
| `endpoint_class` | Hosted HTTPS, local loopback, remote HTTPS, or placeholder/invalid until configured. |
| `economics_class` | Qualitative economics bucket for later routing, such as premium hosted, commodity hosted, local compute, or aggregator lane. |
| `latency_class` | Qualitative latency posture for later role/routing decisions. |
| `cost_class` | Relative cost posture for bounded routing decisions. |
| `tool_support_class` | Whether the provider surface is text-only, bounded tool-capable, or intentionally limited. |
| `lane_class` | Reliability lane such as hosted first-party, local model, remote open-weight, aggregator, or test/mock lane. |
| `locality_class` | `hosted`, `local`, or `remote` execution locality. |
| `auth_surface` | Operator-managed credential or local endpoint expectation; not embedded secrets. |

Provider profiles answer:

- what services exist
- how they are reached
- what default model and endpoint expectations they carry
- what qualitative latency/cost/tool/routing lane they belong to

Provider profiles do not answer:

- what one agent, citizen, or institution is allowed to do
- what role authority a C-SDLC lane has
- what identity continuity or civil record a system owns

### Capability profile contract

Capability profiles are provider-independent behavioral descriptors consumed by
role routing and later provider/model matrices.

Required capability-profile fields for WP-05:

| Field | Meaning |
| --- | --- |
| `capability_id` | Stable capability identifier. |
| `interaction_modes` | Chat, completion, tool use, review/synthesis, batch, or replay modes. |
| `structured_output_posture` | Expected reliability for machine-readable output. |
| `tool_orchestration_posture` | Whether the capability can safely act in tool-using or review-only lanes. |
| `context_class` | Qualitative context-window class, independent of one provider. |
| `reasoning_posture` | Qualitative reasoning/depth posture for later role suitability. |
| `determinism_posture` | How safely the capability supports repeatable review or routing surfaces. |
| `safety_limit_notes` | Named limitations or blocked cases that later matrices must preserve. |

Capability profiles answer:

- what a role or lane needs from a model surface
- how later matrices can compare providers without confusing provider identity
  with role authority

Capability profiles do not answer:

- which vendor hosts the model
- where credentials live
- which citizen/institution/identity record owns the action

### Identity and authority non-claims

The provider/capability catalog is intentionally not:

- a civil identity registry
- a citizen profile system
- an institution directory
- an authority or approval ledger
- the final C-SDLC role-provider matrix

Those layers may consume provider/capability catalogs later, but they are not
represented as provider config in WP-05.

## Current provider mapping notes

WP-05 uses the current deterministic provider-profile sources in the repository
to define the first catalog split.

| Provider family / lane | Current profile shape | Catalog notes |
| --- | --- | --- |
| OpenAI | bounded `http` preset plus ChatGPT-facing profile family | Hosted HTTPS lane; premium-first provider family with distinct ChatGPT-facing profile names. |
| Anthropic | bounded `http` preset plus Claude-facing profile family | Hosted HTTPS lane; provider family distinct from role authority. |
| Google | bounded `http` preset for Gemini | Hosted HTTPS lane; profile catalog owns provider/service identity, not capability claims. |
| Ollama | explicit local `ollama:*` presets | Local loopback or configured HTTP lane; locality matters independently of capability posture. |
| OpenRouter | bounded `http` preset | Aggregator lane; provider family is not equivalent to the downstream model capability. |
| DeepSeek remote | bounded `http` preset | Remote hosted lane; later reliability proof owns quality/resilience claims. |
| Mock/test | `mock:echo-v1` | Test-only substrate; useful for deterministic harnesses, not general role authority. |
| Local/remote open-weight lanes | represented through locality + endpoint class rather than identity objects | Locality and transport belong to provider profile; model-role suitability remains later matrix work. |

## Current tracked source truth

This issue was authored against `.adl/docs/TBD/ADL_PROFILES_PROVIDERS_V2.MD`,
but that path is not present in the current tracked repository state.

The live tracked sources consumed here are:

- [ADR 0004 provider profiles](../../../adr/0004-provider-profiles.md)
- `adl/src/provider/profiles.rs`

Remediation note:

- the stale issue-input path should be corrected or retired in future
  authoring/remediation work rather than silently treated as canonical

Tracked reconciliation note:

- `#4111` now records the live tracked replacement contract in
  `docs/milestones/v0.91.6/review/provider/PROVIDER_PROFILES_V2_RECONCILIATION_4111.md`
  so provider/profile V2 truth is no longer hidden behind the missing TBD path

## Provider profiles V2 reconciliation status

`#4111` makes the post-WP-05 truth explicit:

- provider profiles are partially implemented in runtime plus documented in the
  WP-05 boundary packets
- capability profiles are documented-only today
- role-provider profiles are documented-only today
- model identity is only partially documented and is not yet a first-class
  typed runtime surface
- citizen, institution, guild, continuity, and broader identity concepts remain
  routed out of the provider lane

This feature doc should therefore be read together with:

- `docs/milestones/v0.91.6/review/provider/PROVIDER_PROFILES_V2_RECONCILIATION_4111.md`
- `docs/milestones/v0.91.6/review/provider/PROVIDER_CAPABILITY_PROFILE_CATALOG_4007.md`
- `docs/milestones/v0.91.6/review/provider/ROLE_PROVIDER_PROFILES_4053.md`

## Downstream consumption

- WP-05 later issues consume this split for role suitability, Gemma/OpenRouter
  proof, failure-mode/resilience integration, and closeout.
- WP-06 should treat provider communication and access/catalog decisions as
  consuming provider-profile and capability-profile boundaries rather than
  collapsing them into one routing object.
- WP-08 may consume capability evidence later, but identity/continuity remains
  separate from provider config.

## Validation And Review

- Require bounded evidence links for every promoted role lane.
- Require self-validating proof bundles for provider claims.
- Use bounded review packets for provider/capability boundaries, role
  suitability, hosted/remote reliability, failure routing, sanitation, and
  role-provider policy.
- Keep historical weaker results visible when newer bounded proofs improve a
  lane.
- Separate provider-route support from broad role usefulness.
- Record credential-blocked and inventory-only lanes explicitly instead of
  inferring reliability.
- Keep unsupported or weakly-proven surfaces classified as blocked,
  candidate-only, or useful-with-limits instead of silently upgrading them.
- Preserve explicit non-claims for autonomy, benchmark, and training
  interpretations.

## Catalog Boundary Consumed From M-00

`#4008` consumes the `#4007` split between:

1. provider profiles: infrastructure and service identity
2. capability profiles: provider-independent behavioral posture

This issue adds role suitability on top of that split, but does not collapse:

- provider family
- model behavior
- role authority
- identity/citizen/institution state

## Evidence Inputs

| Evidence surface | Role in this matrix | Current truth |
| --- | --- | --- |
| `#4007` proof note | Defines provider/capability boundary | merged baseline truth via `#4063` |
| `PROVIDER_MODEL_ROLE_MATRIX_2026-06-05.md` | Historical v0.91.5 baseline | Useful for original lane availability and early non-claims |
| `OPENROUTER_MATRIX_PROOF_2026-06-14.md` | Strongest OpenRouter route proof | Five native requested-route probes are `supported_with_limits` |
| `MULTI_AGENT_C_SDLC_OPERATION_v0.91.5.md` | Multi-agent consumption baseline | Records that usefulness still depends on role/task shape |
| `REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md` | Strongest remote Gemma watcher proof | Larger Gemma4 watcher routes are now `useful_with_limits` |
| `DEEPSEEK_C_SDLC_SUITABILITY_PROOF_2026-06-18.md` | Direct hosted/local DeepSeek role probe | Hosted native DeepSeek is now `useful_with_limits`; local `deepseek-r1:8b` and `deepseek-r1:32b` remain candidate-only because closeout truth drift persisted in the bounded panel. |
| `CURRENT_MODEL_SUITABILITY_MINI_SPRINT_CLOSEOUT_4158.md` | Current-model suitability mini-sprint rollup | Direct-hosted OpenAI/Codex, Anthropic, and Gemini packets are now consumed as bounded role evidence without granting workflow authority. |

## Role Suitability Matrix

| Role / lane | Strongest currently evidenced lane | Evidence status | Strongest evidence | Known limits |
| --- | --- | --- | --- | --- |
| Planning | Direct-hosted OpenAI/Codex, Gemini, Anthropic Sonnet/Haiku, hosted DeepSeek, and OpenRouter `deepseek/deepseek-v4-flash` | `useful_with_limits` for the direct-hosted suitability packets; `supported_with_limits` for the prior OpenRouter route | `CURRENT_MODEL_SUITABILITY_MINI_SPRINT_CLOSEOUT_4158.md`; `OPENROUTER_MATRIX_PROOF_2026-06-14.md`; `DEEPSEEK_C_SDLC_SUITABILITY_PROOF_2026-06-18.md` | Structured route execution is proven; broad planner usefulness remains task-shaped rather than universal. Planner output stays advisory and cannot execute, merge, or close work. |
| Coding / worker | Direct-hosted OpenAI/Codex and prior OpenRouter worker routes | `useful_with_limits` for bounded OpenAI/Codex suitability packets; `supported_with_limits` for prior OpenRouter worker routes | `CURRENT_MODEL_SUITABILITY_MINI_SPRINT_CLOSEOUT_4158.md`; `OPENROUTER_MATRIX_PROOF_2026-06-14.md` | Worker outputs were bounded route/panel proofs, not broad code-quality certification. Local Qwen coder remains a candidate from prior matrix evidence rather than a newly proven default here. |
| Review / critic | Direct-hosted OpenAI/Codex, Gemini, Anthropic Sonnet/Haiku, hosted DeepSeek, and prior OpenRouter reviewer lanes | `useful_with_limits` for direct-hosted suitability packets; `supported_with_limits` for the prior OpenRouter route | `CURRENT_MODEL_SUITABILITY_MINI_SPRINT_CLOSEOUT_4158.md`; `OPENROUTER_MATRIX_PROOF_2026-06-14.md`; `DEEPSEEK_C_SDLC_SUITABILITY_PROOF_2026-06-18.md` | Findings remain advisory until synthesized and accepted through normal review. Local DeepSeek lanes remain candidate-only because they overclaimed closeout truth in the bounded panel. |
| Summarization / synthesis | Remote `gemma4:31b` via `adl-provider-adapter`; OpenRouter `google/gemini-2.5-flash-lite` | `useful_with_limits` for remote Gemma4 watcher-style summaries; `supported_with_limits` for OpenRouter Gemini watcher route | `REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md`; `OPENROUTER_MATRIX_PROOF_2026-06-14.md` | Proven on short structured prompts; not broad autonomy, not every Gemma size, not every prompt shape. Local Mistral/Gemma remains fallback-candidate inventory rather than a newly proven default here. |
| Orchestration / conductor assist | Direct-hosted suitability packets and prior OpenRouter planner/reviewer routes only as advisory inputs | `limited_advisory_only` | `CURRENT_MODEL_SUITABILITY_MINI_SPRINT_CLOSEOUT_4158.md`; `OPENROUTER_MATRIX_PROOF_2026-06-14.md`; `MULTI_AGENT_C_SDLC_OPERATION_v0.91.5.md` | Role suggestions may assist routing, but no model lane gains merge, closeout, or authority-bearing control. |
| Watcher / janitor | Remote `gemma4:31b` via `adl-provider-adapter`; remote `gemma4:26b` and `gemma4:e4b`; OpenRouter `google/gemini-2.5-flash-lite` | `useful_with_limits` for larger remote Gemma4 routes; `supported_with_limits` for OpenRouter Gemini watcher route; historical `gemma4:e2b` lane remains weaker | `REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md`; `OPENROUTER_MATRIX_PROOF_2026-06-14.md`; historical baseline in `PROVIDER_MODEL_ROLE_MATRIX_2026-06-05.md` | `gemma4:e2b` is not promoted; watcher usefulness remains bounded to short structured prompts and does not prove janitor autonomy. |
| Local / offline fallback | No single lane promoted; local Qwen, DeepSeek, Gemma, Mistral, and Llama-family Ollama remain inventory-known candidates, with DeepSeek local specifically constrained by the bounded panel | `inventory_and_bounded_candidate_only` | `DEEPSEEK_C_SDLC_SUITABILITY_PROOF_2026-06-18.md`; `PROVIDER_MODEL_ROLE_MATRIX_2026-06-05.md` | Inventory proves availability, not full role reliability. The direct local DeepSeek lanes produced useful bounded outputs in places, but they still overclaimed closeout truth and therefore are not promoted beyond candidate-only status here. |
| Constrained lanes / negative controls | Missing-credential and fail-closed routes | `proven_fail_closed` where tested | `OPENROUTER_MATRIX_PROOF_2026-06-14.md` | Negative controls prove auth failure behavior, not productive role output |

## Provider / Model Lane Register

| Provider/model lane | Provider boundary | Capability/role posture | Current status | Evidence |
| --- | --- | --- | --- | --- |
| Native OpenRouter `deepseek/deepseek-v4-flash` | Aggregator/provider route identity preserved | planner candidate | `supported_with_limits` | `OPENROUTER_MATRIX_PROOF_2026-06-14.md` |
| Native OpenRouter `openai/gpt-4o-mini` | Aggregator/provider route identity preserved | worker candidate | `supported_with_limits` | `OPENROUTER_MATRIX_PROOF_2026-06-14.md` |
| Native OpenRouter `anthropic/claude-3.5-haiku` | Aggregator/provider route identity preserved | reviewer candidate | `supported_with_limits` | `OPENROUTER_MATRIX_PROOF_2026-06-14.md` |
| Native OpenRouter `google/gemini-2.5-flash-lite` | Aggregator/provider route identity preserved | watcher/summarizer candidate | `supported_with_limits` | `OPENROUTER_MATRIX_PROOF_2026-06-14.md` |
| Native OpenRouter `qwen/qwen3.6-flash` | Aggregator/provider route identity preserved | worker candidate | `supported_with_limits` | `OPENROUTER_MATRIX_PROOF_2026-06-14.md` |
| Remote Ollama `gemma4:31b` through `adl-provider-adapter` | Remote Ollama substrate | watcher/summarizer candidate | `useful_with_limits` | `REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md` |
| Remote Ollama `gemma4:26b` raw HTTP | Remote Ollama substrate | watcher/summarizer candidate | `useful_with_limits` | `REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md` |
| Remote Ollama `gemma4:e4b` raw HTTP | Remote Ollama substrate | watcher/summarizer candidate | `useful_with_limits` | `REMOTE_GEMMA_WATCHER_PROOF_2026-06-15.md` |
| Remote Ollama `gemma4:e2b` historical watcher lane | Remote Ollama substrate | watcher candidate | `historical_empty_output` | `PROVIDER_MODEL_ROLE_MATRIX_2026-06-05.md`; `multi_agent_workcell/V0915_PARALLEL_C_SDLC_WORKCELL_PROOF_PACKET_2026-06-14.md` |
| Direct hosted OpenAI / Codex | Native hosted provider | planner/worker/reviewer/closeout-checker candidates | `useful_with_limits` for `gpt-5.5`, `gpt-5.4`, `gpt-5-codex`, and `gpt-5.3-codex`; `gpt-5.3-codex-spark` was `runtime_unsuitable_for_this_panel` because the provider returned `model_not_found` | `CURRENT_MODEL_SUITABILITY_MINI_SPRINT_CLOSEOUT_4158.md`; `OPENAI_CURRENT_MODEL_SUITABILITY_PROOF_2026-06-18.md`; `OPENAI_CODEX_AND_SPARK_SUITABILITY_PROOF_2026-06-18.md`; `OPENAI_GPT53_CODEX_SUITABILITY_PROOF_2026-06-18.md` |
| Direct hosted Anthropic | Native hosted provider | reviewer/planner/closeout-checker candidates | `useful_with_limits` for `claude-sonnet-4-6` and `claude-haiku-4-5`; `claude-opus-4-8` remains `candidate_only_format_repair_needed` | `CURRENT_MODEL_SUITABILITY_MINI_SPRINT_CLOSEOUT_4158.md`; `ANTHROPIC_CURRENT_MODEL_SUITABILITY_PROOF_2026-06-18.md` |
| Direct hosted DeepSeek API | Native hosted provider | watcher/reviewer/planner candidate | `useful_with_limits` | `DEEPSEEK_C_SDLC_SUITABILITY_PROOF_2026-06-18.md` |
| Gemini direct native provider | Native hosted provider | watcher/reviewer/planner/closeout-checker candidates | `useful_with_limits` for `gemini-2.5-pro`, `gemini-2.5-flash`, and `gemini-2.5-flash-lite` | `CURRENT_MODEL_SUITABILITY_MINI_SPRINT_CLOSEOUT_4158.md`; `GEMINI_CURRENT_MODEL_SUITABILITY_PROOF_2026-06-18.md` |
| Local Ollama `deepseek-r1:32b` | Local provider substrate | watcher/reviewer/planner candidate | `candidate_only_truth_repair_needed` | `DEEPSEEK_C_SDLC_SUITABILITY_PROOF_2026-06-18.md` |
| Local Ollama `deepseek-r1:8b` | Local provider substrate | watcher/planner candidate | `candidate_only_truth_repair_needed` | `DEEPSEEK_C_SDLC_SUITABILITY_PROOF_2026-06-18.md` |
| Local Ollama Qwen / DeepSeek / Mistral / Gemma family | Local provider substrate | local/offline role candidates | `inventory_and_bounded_candidate_only` | `PROVIDER_MODEL_ROLE_MATRIX_2026-06-05.md` |

## Known Failure Modes And Non-Claims

| Surface | Failure mode or risk | Truthful classification |
| --- | --- | --- |
| Historical or untested hosted direct lanes | Credentials absent in the earlier proving shell, or specific requested model IDs unavailable at runtime | blocked or runtime-unsuitable for the named lane, not failed implementation |
| OpenRouter routes | Route support proven on five requested IDs only | supported with limits, not universal gateway proof |
| Remote Gemma watcher | Older `gemma4:e2b` lane completed with empty output | historical weak result remains binding for that lane |
| Remote Gemma recovery | Larger Gemma4 routes succeeded on short structured prompts | useful with limits, not broad autonomy |
| Local inventory | Model presence visible without role probe | candidate only, not reliable role proof |
| Multi-agent consumption | Good lane output does not grant workflow authority | advisory only; merge/closeout authority remains serialized to team/tooling |

## Capability-Based Delegation Consumption Notes

- `v0.92` may consume this matrix only as a named role-routing table with
  explicit proof labels.
- Capability selection must preserve the provider/capability split from `#4007`
  instead of encoding role authority into provider identity.
- Watcher and reviewer routing may prefer `useful_with_limits` lanes for bounded
  prompts, but must preserve single-agent fallback when prompt shape, latency,
  or evidence quality is not a good fit.
- Inventory-only and credential-blocked lanes must remain selectable only as
  operator-known possibilities, not as proven defaults.

## v0.92 Consumption

`v0.92` may consume provider/model readiness only as:

- a role-scoped matrix with named evidence states;
- a bounded provider failure-routing contract;
- a bounded advisory routing surface;
- a bounded role-provider policy layer with advisory-only authority;
- a non-authority-bearing input to capability selection and birthday-demo
  planning.

`v0.92` must not infer:

- general intelligence;
- training readiness;
- product benchmark status;
- universal watcher autonomy;
- autonomous repo authority;
- authority to merge, close, or bypass C-SDLC review and closeout gates.

## Non-Goals

- No training claims.
- No Aptitude Atlas baseline.
- No unqualified "all models work" claim.
- No provider catalog that doubles as identity, citizen, or institution state.

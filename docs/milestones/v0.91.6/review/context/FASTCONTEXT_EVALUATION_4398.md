# FastContext Evaluation for `#4398`

Status: `defer_direct_adoption`
Issue: `#4398`
Sprint umbrella: `#4388`
Date: 2026-06-21

## Scope

This packet evaluates FastContext as a possible acceleration substrate for ADL
and C-SDLC workflows in `v0.91.6`.

It does not:

- add FastContext as a dependency
- send ADL repository data to a third-party endpoint
- replace ADL prompt-card, review-packet, or watcher formats
- implement watcher/session-registry automation

## Sources

Primary external sources:

- [microsoft/fastcontext README](https://github.com/microsoft/fastcontext)
- [FastContext arXiv abstract](https://arxiv.org/abs/2606.14066)

Repo evidence:

- `AGENTS.md`
- `docs/tooling/SESSION_COORDINATION_AND_ROOT_CHECKOUT_POLICY.md`
- `docs/tooling/CODEX_USAGE_WATCHER.md`
- `.adl/v0.91.6/bodies/issue-4398-v0-91-6-tools-context-evaluate-fastcontext-for-c-sdlc-workflows.md`
- `docs/milestones/v0.91.6/review/provider/C_SDLC_AGENT_SUITABILITY_PANEL_4097.md`

## Decision

Recommendation: defer direct FastContext adoption for ADL in `v0.91.6`.

If ADL wants to use the idea later, the right next step is a bounded
repo-native pilot that borrows the design pattern:

- separate exploration from solving
- keep the explorer read-only
- return compact citations instead of broad transcript residue

ADL should not treat the upstream FastContext project itself as a drop-in
workflow dependency yet.

The recommendation is now based on local proof as well as design fit:

- the exact Ollama-hosted FastContext model was non-proving in direct smoke
  checks on this host
- the same model was also non-proving in ADL's UTS benchmark harness: partial
  regular-lane competence, weak UTS conformance, and unstable failure behavior
- a control model on the same Ollama endpoint did work, which isolates the
  issue away from Ollama in general
- the upstream grep tool path also carries a host assumption that fails on this
  Mac

## What FastContext Is Good At

From the upstream README and paper abstract, FastContext is a dedicated
repository-exploration subagent:

- it uses read-only repository tools
- it can issue independent reads/searches in parallel
- it returns compact file and line citations
- it is meant to reduce main-agent token burn during broad repo exploration

That is a real fit with one recurring ADL pain point: too much expensive,
chat-local repo exploration before the agent reaches the bounded task.

The idea is especially attractive for:

1. locating relevant source/doc/test surfaces quickly
2. producing compact evidence for a main implementation or review agent
3. reducing solver-context pollution during issue work

## Fit Against ADL Use Cases

| ADL use case | Fit | Why |
| --- | --- | --- |
| Watcher support | `partial` | Watchers in ADL classify issue/PR/check state. FastContext is strongest on repository exploration, not live workflow-state truth. |
| Sprint handoff | `partial` | It can help locate relevant files for a handoff packet, but it does not solve session ownership, branch/worktree truth, or goal hierarchy. |
| Prompt/card context packets | `strong_pattern_fit` | A read-only explorer that returns citations could help build compact packet inputs before review/planning. |
| Local-agent delegation | `strong_pattern_fit` | ADL already benefits from role separation. A dedicated explorer aligns with that shape. |
| Review packets | `moderate` | It can help gather evidence, but it is not itself a findings-first reviewer or truth normalizer. |
| Runtime memory handoff | `weak` | FastContext is for repository exploration, not runtime memory/state continuity. |

## Where It Conflicts With Current ADL Priorities

ADL's active workflow pressure in `v0.91.6` is not just "find code faster."
The repo is explicitly pushing on:

- issue-bound goals
- worktree/session coordination
- fail-closed watcher mechanics
- durable prompt-card truth
- reviewable, repo-native evidence packets

FastContext does not directly solve those control-plane requirements.

Its strongest value is earlier in the chain:

- gather relevant repo evidence
- hand compact citations to the real ADL workflow actor

That means FastContext is better understood as a helper behind packet-building
or bounded issue exploration, not as a watcher, conductor, closeout agent, or
session registry substrate.

## Security And Privacy Assessment

FastContext has some appealing safety characteristics:

- upstream positions it as read-only
- the tool contract is repository exploration, not mutation
- the output format is compact citations rather than wholesale file dumps

But ADL has stricter privacy and locality expectations than the upstream
project assumes by default.

Key concerns:

1. Upstream expects an OpenAI-compatible endpoint and API key configuration.
   That is not automatically acceptable for ADL issue work that may expose
   local planning state, non-public cards, or private repository contents.
2. The quick-start path writes trajectory logs under `.fastcontext/`.
   ADL would need explicit ignored-path, retention, and redaction rules before
   treating that as acceptable local residue.
3. FastContext explores the live repository. In ADL, many important workflow
   truths live in tracked docs, ignored `.adl` state, worktree-local issue
   bundles, and review artifacts. A naive explorer could over-read or leak
   more than the bounded packet actually needs.

ADL policy consequence:

- no direct hosted-provider use for real repo evaluation without explicit
  approval
- no default durable trace logging without explicit hygiene rules
- no authority to read arbitrary local state outside a bounded packet contract

## Local Proof Run

This issue also ran a small local proof against the Ollama-hosted FastContext
model named by the operator:

- model:
  `hf.co/mitkox/FastContext-1.0-4B-RL-Q4_K_M-GGUF:latest`
- endpoint: local Ollama OpenAI-compatible chat surface at
  `http://127.0.0.1:11434/v1`
- worktree: the bound `#4398` issue worktree
- query target: ADL's UTS benchmark runner and task-panel surfaces

Observed results:

1. Direct model smoke through the OpenAI-compatible chat endpoint returned an
   empty assistant message with `finish_reason=stop` even for a trivial
   `READY` prompt.
2. Direct tool-call smoke through the same endpoint also returned an empty
   assistant message and no tool calls.
3. Native Ollama `/api/generate` and `/api/chat` calls for the same model also
   returned an empty string with `done_reason=stop`.
4. A one-model ad hoc UTS benchmark run, using the canonical
   `adl/tools/uts_benchmark_runner.py` with a temporary panel entry for the
   exact model tag, produced a `non_proving` result:
   - deterministic self-check passed
   - the first attempt was skipped because another Ollama model was resident
   - after clearing residency, the regular lane showed some valid ordinary
     tool-call behavior but still ended non-proving because timeout and output
     quality failures remained
   - the UTS-only lane remained weak and mostly non-conforming, with proposal
     quality problems and provider failures
5. A control model on the same Ollama endpoint, `qwen3-coder:30b`, returned
   `READY` normally and emitted a valid function tool call when tools were
   supplied.
6. A control FastContext CLI run with `qwen3-coder:30b` did begin normal
   multi-step exploration over the ADL repo, which shows the framework and
   endpoint can work even though the named FastContext model did not.

Interpretation:

- the exact local FastContext model is non-proving in this serving path
- the model is not completely inert; it can produce some ordinary tool-call
  answers in the benchmark regular lane
- that said, its UTS behavior is weak and its failure pattern is unstable, so
  ADL still cannot treat it as ready for governed repo exploration
- any future pilot should start with a model-and-serving compatibility proof
  plus a narrow UTS/tooling suitability proof, not with a workflow integration
  assumption

## Host-Portability Finding

The local proof also surfaced a host-specific portability issue in the upstream
tooling layer.

Observed behavior:

- the control FastContext CLI run called `Glob` successfully
- the same run's `Grep` tool failed because the implementation hardcodes
  `/usr/bin/rg`
- on this host, `rg` is available, but not at that hardcoded path

Implication for ADL:

- even if the model lane were fixed, the upstream explorer still carries host
  assumptions on this Mac that would need patching before ADL should treat it
  as a serious local workflow dependency

## Integration Effort

Direct adoption would still require meaningful adaptation work:

1. map FastContext invocation into ADL's repo-native workflow instead of ad hoc
   CLI usage
2. constrain exploration to approved packet roots or issue-local surfaces
3. normalize output into ADL citation/evidence conventions
4. define ignored/local trace handling and redaction policy
5. decide whether the serving model is sovereign/local or external

That is more than a small wiring task. It is a boundary-definition task, and
the local proof now adds model-serving compatibility and host-portability work
to that list.

## Failure Modes To Avoid

The main risks if ADL adopted this too casually are:

- context exploration becoming a hidden second workflow outside card truth
- non-public repo or local-state material being sent to an external endpoint
- trajectory logs accumulating sensitive local residue
- exploration outputs being treated as workflow truth instead of evidence
- using a repo explorer to paper over missing session-registry or watcher work
- assuming an Ollama-hosted FastContext model is usable before it proves basic
  response stability, tool-call behavior, and at least minimally credible UTS
  conformance
- assuming upstream local tools are host-portable without proving their command
  paths

## Recommended Route

For `v0.91.6`, the correct route is:

1. defer direct adoption
2. keep FastContext as a design reference
3. treat the current Ollama FastContext model as non-proving until it can
   answer reliably, avoid timeout/error collapse, and produce materially better
   UTS-form proposals on this host
4. finish the repo-native watcher/goal/session-control work first
5. only then consider a bounded pilot for a local or sovereign read-only
   explorer

That pilot should be explicitly narrower than "use FastContext everywhere."

## Follow-On Issue Candidates

If ADL wants to act on this later, the best next issue is:

1. `Pilot a repo-native read-only context explorer for bounded issue packets`
   - scope: one local-only or sovereign-hosted exploration lane
   - inputs: one bounded issue packet or review packet fixture
   - output: compact citation block plus trace-hygiene proof
   - proof gate: the chosen model must first pass a basic reply smoke and a
     basic tool-call smoke on the intended serving path
   - non-goals: no mutation authority, no watcher authority, no closeout
     authority

Optional later follow-on:

2. `Evaluate whether explorer citations improve packet-build quality and token
   use for ADL issue work`
   - compare a no-explorer baseline against one bounded explorer-assisted lane
   - keep evaluation local and fixture-backed

## Non-Claims

This packet does not claim:

- FastContext is bad technology
- FastContext cannot help ADL at all
- hosted-provider use is acceptable for ADL by default
- FastContext solves watcher/session-goal/control-plane needs
- ADL should vendor or depend on the upstream project in `v0.91.6`

## Conclusion

My view is that FastContext is a good idea in the right layer and the wrong
dependency at the wrong time.

The part worth keeping is the contract:

- delegated exploration
- read-only tools
- compact citations
- lower main-agent context burn

The part to defer is direct adoption of the upstream package and serving model
assumptions. The local proof made that recommendation firmer and more precise:
the current model is not dead on arrival, but it is still too unstable and too
weak on UTS-shaped behavior to justify direct adoption.

So the honest answer for `#4398` is:

- keep the idea
- do not adopt the dependency now
- do not treat the current Ollama FastContext model as proven
- route any future work as a bounded repo-native pilot with strict packet and
  privacy boundaries

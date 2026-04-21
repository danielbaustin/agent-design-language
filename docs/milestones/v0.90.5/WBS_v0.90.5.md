# WBS - v0.90.5

## Work Package Shape

v0.90.5 should use the standard 20-WP shape. It is an implementation milestone
for Governed Tools v1.0, not a documentation-only planning lane.

| WP | Issue | Title | Purpose | Primary Output | Depends On |
| --- | --- | --- | --- | --- | --- |
| WP-01 | planned | Promote v0.90.5 milestone package | Finalize this planning package and create the issue wave | tracked docs and issue cards | v0.90.4 closeout or roadmap approval |
| WP-02 | planned | Tool-call threat model and semantics | Define proposal/action boundary, side effects, dangerous categories, and non-goals | threat model and semantics doc | WP-01 |
| WP-03 | planned | UTS public compatibility and conformance plan | Define compatibility, examples, invalid examples, extension rules, and conformance | UTS conformance plan | WP-02 |
| WP-04 | planned | UTS v1 schema finalization | Finalize portable tool schema and validation rules | UTS schema and tests | WP-03 |
| WP-05 | planned | UTS fixture and conformance suite | Build valid, invalid, extension, and dangerous-category fixtures | UTS fixture suite | WP-04 |
| WP-06 | planned | ACC v1 authority schema | Define ADL-native authority, identity, policy, risk, and execution schema | ACC schema and authority fixtures | WP-02 |
| WP-07 | planned | ACC privacy, visibility, and delegation model | Define who may call, see, delegate, inspect, challenge, and receive redacted views | visibility/delegation matrix | WP-06 |
| WP-08 | planned | Tool registry and binding model | Register known tools and bind only approved adapters | registry and binding tests | WP-04, WP-06 |
| WP-09 | planned | UTS to ACC compiler | Compile validated UTS/proposals into ACC deterministically | compiler and mapping tests | WP-05-WP-08 |
| WP-10 | planned | Normalization and argument validation | Treat all tool arguments as untrusted input | normalization and rejection tests | WP-09 |
| WP-11 | planned | Policy injection and authority evaluation | Inject role, standing, environment, capability, and sensitivity policy | policy engine slice and tests | WP-07, WP-09 |
| WP-12 | planned | Freedom Gate integration | Evaluate candidate tool actions before execution | decision events and gate tests | WP-11 |
| WP-13 | planned | Governed executor | Execute only approved ACC-backed actions | executor and refusal behavior | WP-12 |
| WP-14 | planned | Trace, replay, redaction, and evidence contract | Emit accountable evidence without leaking private data | trace/redaction contract and tests | WP-13 |
| WP-15 | planned | Dangerous tool negative suite | Prove destructive, process, network, exfiltration, missing actor, unsafe replay, and delegation failures | negative safety suite | WP-08-WP-14 |
| WP-16 | planned | Model proposal benchmark harness | Test model tool proposals against schema, authority, privacy, and bypass rubrics | benchmark runner and report | WP-04, WP-15 |
| WP-17 | planned | Local model and Gemma-focused evaluation | Evaluate local models and Gemma-family candidates | local model scorecards | WP-16 |
| WP-18 | planned | Governed Tools v1.0 flagship demo | Show proposal, validation, ACC, policy, gate, execution/denial, trace, and redaction | flagship demo proof packet | WP-13-WP-17 |
| WP-19 | planned | Demo matrix, docs, review, and public-spec handoff | Align docs, feature docs, conformance, demo matrix, and review packet | review-ready package | WP-18 |
| WP-20 | planned | Release ceremony | Complete release closure | release evidence and next handoff | WP-19 |

## Compression Candidate

The milestone can compress after UTS, ACC, fixtures, and compiler contracts are
stable. It must not compress away threat modeling, public conformance, model
testing, negative security tests, or redaction/visibility review.


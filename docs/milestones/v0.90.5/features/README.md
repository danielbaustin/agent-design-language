# v0.90.5 Feature Contracts

These feature docs primarily define the implementation-facing contracts for
Governed Tools v1.0, with adjacent prerequisite feature surfaces added only
when later implementation work needs a tracked milestone contract instead of a
TBD note.

The root rule is stable across all feature slices: UTS describes tools; ACC
governs capability exercise.

The shared WP-02 boundary lives in
`TOOL_CALL_THREAT_MODEL_AND_SEMANTICS.md`: model tool calls are proposals, not
actions, and UTS validity is schema compatibility rather than execution
authority.

The Agent Communication and Invocation Protocol parent feature surface for the
parallel Comms sprint lives in `AGENT_COMMS_v1.md`. It is a governed-tools-
adjacent prerequisite surface, not a claim that ACIP has become part of the
core Governed Tools feature stack in full.

The demo-grade local model PR reviewer tool usage guide lives in
`LOCAL_MODEL_PR_REVIEWER_TOOL.md`.

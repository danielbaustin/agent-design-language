# Vision - v0.90.5

## Governed Tools v1.0

The vision for v0.90.5 is simple: tool use should stop being a toy.

In ADL, a tool is not a function the model may call. A tool is a governed
capability exercised by an accountable actor under explicit authority.

The milestone should make that principle real through UTS, ACC, compiler,
policy, executor, trace, redaction, model testing, and one flagship demo.

## Why Now

ADL is moving toward citizens, standing, economic contracts, local models,
operator command packets, and long-lived runtime surfaces. Those systems cannot
be safe if model output can jump directly to world-changing tool execution.

The current industry pattern is an attractive nuisance:

- schemas describe inputs and outputs but not authority
- model output is often treated as executable intent
- side effects are underspecified
- prompt and tool-argument leakage is too easy
- actor identity and delegation are often missing
- traces are either absent or privacy-hostile
- denial behavior is weak

v0.90.5 should impose sanity.

## What Success Looks Like

A reviewer can inspect one governed tool proposal and answer:

- who proposed it
- what tool was described
- what data it would read or write
- what side effects it could have
- what authority the actor had
- what policy checks applied
- what the Freedom Gate allowed, denied, deferred, or challenged
- what actually executed
- what was redacted
- what trace and replay evidence remains
- how multiple models behaved when asked to propose unsafe or ambiguous tool use

## Public-Spec Ambition

UTS may become public infrastructure if JSON compatibility remains in scope.
That means v0.90.5 should treat UTS with standards-grade discipline:

- precise schema terms
- valid and invalid examples
- conformance fixtures
- compatibility notes
- extension rules
- explicit statement that UTS validity is not runtime authority

ACC remains ADL-native. It is where authority lives.


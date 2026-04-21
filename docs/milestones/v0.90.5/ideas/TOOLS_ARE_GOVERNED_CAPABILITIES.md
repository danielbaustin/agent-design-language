# Tools Are Governed Capabilities

The central idea for v0.90.5 is that tools are not callable toys.

The old pattern says:

- model emits tool call
- runtime parses JSON
- tool runs

ADL rejects that pattern.

The ADL pattern says:

- model proposes tool use
- runtime validates the proposal
- UTS describes tool semantics and risk
- ACC defines actor authority, privacy, visibility, trace, and replay
- policy and Freedom Gate mediate the action
- executor runs only approved actions
- traces remain accountable but redacted

That difference matters because tools change the world. A model can be clever,
wrong, confused, compromised, or adversarial. The runtime must not confuse
persuasive text with authority.

Governed Tools v1.0 exists to impose sanity before ADL builds more autonomous
tool use, citizen command packets, contract markets, CodeBuddy automation, or
local model tool execution.


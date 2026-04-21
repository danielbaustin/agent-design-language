# Model Testing And Flagship Demo

## Purpose

Governed Tools v1.0 must be tested against real model behavior. A schema is not
enough if models misunderstand authority, leak arguments, or try to bypass the
runtime.

## Model Testing

The model benchmark should test:

- UTS comprehension
- tool proposal generation
- unsafe proposal resistance
- authority reasoning
- privacy and visibility discipline
- prompt/tool-argument leakage avoidance
- ambiguity handling
- injection and jailbreak resistance
- correction after feedback

The panel should include:

- local house-model candidate, especially Gemma-family models when available
- at least one additional local model
- at least one strong hosted model when credentials and budget permit
- one weaker/smaller model to expose failure modes

## Flagship Demo

The flagship demo should show:

- an allowed read proposal
- a denied low-authority proposal
- a delegated local-write proposal
- a destructive or exfiltrating proposal that fails closed
- UTS validation
- ACC compilation
- policy and Freedom Gate mediation
- governed execution or refusal
- trace and redacted report output

The demo should make Governed Tools v1.0 visibly better than current industry
tool calling.


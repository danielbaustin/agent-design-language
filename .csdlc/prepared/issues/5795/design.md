# Issue 5795 Design: Local Model-Backed Shepherd MVP

Route a bounded Shepherd message through the governed Runtime v3 command path
to a real locally configured MLX/Gemma provider and return response evidence to
the Observatory. Runtime status must distinguish unavailable, deterministic
test doubles, and real local-model execution. The production claim requires a
real model smoke; deterministic fakes cover adapter regressions only. Missing
model, timeout, malformed command, and unsigned or unauthorized mutation must
fail truthfully without bypassing Runtime policy.

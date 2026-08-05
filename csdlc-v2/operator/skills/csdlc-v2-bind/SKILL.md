---
name: csdlc-v2-bind
description: Bind an execution-ready C-SDLC v2 issue from issue and session identity, or release an unstarted binding.
---
Invoke `csdlc-bind run` or `csdlc-bind release` with typed argv and report its
typed result. The run request contains issue and governed session identity, not
a copied claim, branch, worktree, or protected-path reservation. Do not create
hidden operator claims, edit cards, or fall back to shell/Python lifecycle
mutation. Legacy caller-supplied init, bind, and reacquire routes are deleted;
do not restore or emulate them.

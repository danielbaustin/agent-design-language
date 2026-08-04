# ADL Workcell Conductor

`adl-workcell-conductor` is a pure ADL v2 planning library. It converts typed,
immutable lifecycle and execution-plan snapshots into deterministic task
assignments or a fail-closed refusal.

It does not create tasks, access the network or filesystem, mutate GitHub, run
a scheduler, or own review, publication, merge, validation, or closeout.

# Issue 5812 Design: Freedom Gate Clippy Repair

Replace only the two unnecessary lazy default closures in
`adl/src/csm_freedom_gate.rs`. Preserve the exact `true` and `false` JSON
defaults, touch no adjacent behavior, and prove focused tests, formatting,
Clippy `-D warnings`, and diff hygiene.

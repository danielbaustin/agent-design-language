# #5512 coverage-filter ownership repair

The bounded Runtime v3/CSM coverage route must split selectors by crate before invoking nextest. When the incoming impact expression identifies the closed bridge family, the ADL invocation will use the same CSM and long-lived-agent expression as the focused test lane, while Runtime v3 auth, supervision, and topology remain in the `adl-runtime` companion invocation.

The generic coverage path remains unchanged. The regression fixture uses the exact expression rejected by GitHub run 29644007246 and proves that no `adl::cli_smoke` or Runtime v3 selector reaches the ADL workspace.


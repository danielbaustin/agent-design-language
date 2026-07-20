# Issue 5514 design

Status: approved for bounded implementation.

The canonical Runtime v3/CSM risk expression spans two Cargo workspaces. The
coverage runner must recognize that exact expression and partition it without
discarding valid coverage. The ADL workspace receives every existing ADL CSM
test selector except the nonexistent `adl::cli_smoke` binary arm. The
`adl-runtime` workspace receives Runtime v3 authentication, supervision, and
topology selectors. Both summaries remain composed before the changed-source
coverage gate runs.

No production runtime behavior, Runtime v2 source, threshold, or AWS lane is
changed.

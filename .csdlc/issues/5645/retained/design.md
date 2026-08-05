# Typed `csdlc-merge` design

`csdlc-publish` remains responsible for publication intent. A separate
`csdlc-merge` binary performs one authorized merge only when the canonical
record is `merge_ready` and the remote PR still matches the reviewed identity.
It sends the expected head SHA to GitHub and returns the exact merge commit
SHA. Merged-publication reconciliation and terminal closeout remain separate.

The command uses the existing v2 store, claim validation, readiness evidence,
GitHub token resolver, and Octocrab client. It never invokes shell, Python, or
AWS, and it never merges an arbitrary PR.

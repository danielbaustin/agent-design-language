# #5655 Repo-Native GitHub Actions

Add one typed Rust `csdlc-github` command surface for GitHub issue and PR
operations. It uses the existing token resolver and Octocrab client, validates
requests before mutation, and reconciles every remote result by exact readback.

Every issue-create and comment mutation carries a caller-supplied operation
key rendered as a stable marker. Ambiguous results are searched by marker
before retry; missing or duplicated markers fail closed. No title/body guess,
connector, legacy wrapper, raw `gh`, shell/Python lifecycle logic, Runtime, or
AWS is permitted.

The bounded scope is the typed Rust command/schema/test surface and its operator
contract. Existing lifecycle records remain authoritative for claims, review,
publication, and closeout. Acceptance requires issue create/update/labels/
assignees/comments/close plus existing PR publication/ready/merge/check/readback
to be available through Rust with focused failure and reconciliation tests.

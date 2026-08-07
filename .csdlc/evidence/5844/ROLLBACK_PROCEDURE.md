# WP-24 Article Rollback Procedure

## Boundary

Rollback is a repository follow-up change, never an external publish, unpublish, or scheduling action. It removes only the ten issue-owned `ARTICLE.md` drafts, retains all ten source packets and editorial review records, retains provider and lifecycle evidence, and restores the series matrix and publication disposition to an operator-reviewed state appropriate to the reason for rollback.

## Verified Manifest

`rollback-manifest.json` declares the exact remove, retain, and restore sets. The issue validator checks that the sets are disjoint, complete, confined to WP-24, present in the repository, and require no external publication action:

```sh
ruby .csdlc/evidence/5844/validate-article-series.rb --rollback
```

## Execution Rule

No rollback is performed by this issue. If a reviewer later requires rollback, create a bounded follow-up change from the merged revision, apply only the manifest's remove set, update the two restore documents truthfully, preserve the retain set and evidence root, run the rollback validator before and after the proposed patch, and obtain ordinary review before merge.

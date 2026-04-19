# Deployment Notes

## Release Promise

The 0.90 runtime stream includes automatic rollback on every patch cycle and
supports multi-tenant governance traces.

## Reality Check

The same section also instructs operators to use `manual_cleanup` after each
run, which means rollback coverage is not end-to-end yet.
This is a release-truth mismatch for review scoring.

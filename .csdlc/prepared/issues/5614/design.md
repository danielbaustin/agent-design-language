# Issue #5614 design

Secret-scanning alert #1 identifies an AWS temporary access-key-shaped literal in a redaction test fixture. Construct the synthetic value from non-secret fragments at runtime, preserve the sanitizer assertion, and leave no matching literal in tracked source. No AWS access is required or permitted.


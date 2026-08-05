# Issue 5780 deletion metrics

Baseline: `a5a16509af05af441369e78eeacfd74017bcf63b`.
Measurement revision: `ce0994da07d2cdf349c658b1f8dee235814e5905`.

| Surface | Before | After | Net deleted | Reduction |
| --- | ---: | ---: | ---: | ---: |
| Production Rust | 28,722 | 18,194 | 10,528 | 36.65% |
| Test Rust | 15,092 | 9,276 | 5,816 | 38.54% |
| Combined Rust | 43,814 | 27,470 | 16,344 | 37.30% |
| Source binaries | 23 | 21 | 2 | 8.70% |

At the independently certified measurement revision, across every changed
tracked file, including lifecycle cards and proof logs,
the implementation adds 4,675 lines and deletes 18,751, for a net deletion of
14,076 lines.

The removed surface includes the standalone closeout and merge binaries, the
closeout skill, terminal
repair and reconciliation request schemas, readiness and merged-publication
writers, legacy receipt writers, and their writer-specific tests. The retained
compatibility boundary is read-only: all 114 tracked v0.91.8 terminal records
pass the census, historical terminal phases and receipts remain deserializable,
and the 314 existing Git-common receipt files are left immutable. New terminal
authority comes only from the minimal derived envelope written by
`csdlc-finish`.

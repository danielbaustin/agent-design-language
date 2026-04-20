# Decisions - v0.90.2

| ID | Decision | Status | Rationale |
| --- | --- | --- | --- |
| D-01 | Treat v0.90.2 as the first bounded CSM run milestone | Accepted for v0.90.2 | v0.90.1 built the substrate and visibility window; v0.90.2 should make a small governed world run in bounded local form. |
| D-02 | Keep hardening attached to the first run | Accepted for v0.90.2 | Invariant, recovery, quarantine, and security work should prove behavior around the first run rather than becoming disconnected negative tests. |
| D-03 | Audit v0.90.1 before implementation | Accepted for WP-02 | The first CSM run must inherit actual landed artifacts, not an imagined prototype. |
| D-04 | Front-load compression readiness | Accepted for WP-02 | Issue-wave generation, worktree-first execution, validation profiles, and continuous evidence should reduce milestone drag without weakening proof. |
| D-05 | Make violation artifacts stable | Accepted for WP-04 | Reviewers need repeatable evidence when invariants fail. |
| D-06 | Distinguish safe recovery from quarantine | Accepted for WP-11/WP-12 | Unsafe resume should preserve evidence and stop, not pretend recovery succeeded. |
| D-07 | Keep security ecology bounded | Accepted for WP-13 | Governed adversarial verification can defend the polis, but red/blue/purple roles do not define Runtime v2 or CSM. |
| D-08 | Preserve v0.91 and v0.92 scope | Accepted for v0.90.2 | v0.90.2 runs and hardens the substrate; v0.91 owns moral/emotional civilization and v0.92 owns first true birthday. |

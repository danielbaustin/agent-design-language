# Decisions - v0.90.5

| ID | Decision | Status | Rationale | Consequences |
| --- | --- | --- | --- | --- |
| D-01 | v0.90.5 is Governed Tools v1.0 | Accepted | Tool calling needs a full implementation milestone, not a late add-on | v0.90.5 issue wave should implement the first working tool suite |
| D-02 | Tool calling is proposal, not execution | Accepted | Model output is untrusted and cannot be treated as action | Runtime must validate, compile, mediate, and trace before execution |
| D-03 | UTS is portable and public-compatible | Accepted | JSON compatibility may make UTS useful outside ADL | Requires public-spec discipline, examples, invalid examples, and conformance |
| D-04 | ACC owns ADL runtime authority | Accepted | UTS metadata must not imply permission to execute | Authority, identity, delegation, privacy, trace, replay, and Freedom Gate stay in ACC |
| D-05 | Visibility and redaction are first-class | Accepted | Tool traces and errors can leak private data | Actor, operator, reviewer, public, and Observatory views must be defined |
| D-06 | Model testing is required | Accepted | A schema is not enough if models misuse or bypass it | Multi-model and local/Gemma tests are part of the milestone |
| D-07 | v0.90.3 owns the inhabited CSM demo | Accepted | Citizen-state demo and tools demo have different proof surfaces | v0.90.5 gets a governed-tools flagship demo instead |

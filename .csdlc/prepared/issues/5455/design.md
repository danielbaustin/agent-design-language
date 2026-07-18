# #5455 Design

Install receipts record the exact repository revision used to build the stable owner binaries. Coexistence verification compares that revision with the repository HEAD and fails closed with an explicit stale-provenance error before any lifecycle command can use an obsolete binary.

The change is limited to the independent Rust v2 installer/operator path and its Gate 10A proof.

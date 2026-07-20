# Runtime v3 Adapter

The Runtime v3 adapter connects ADL plans and engine events to Runtime v3 while
preserving Runtime v3 as the execution authority.

Required proof comes from `#5341` and `#5361`, with provider/tool adapter
support from `#5349`. Runtime v3 acceptance also consumes the WP-10A live
workcell output-contract proof from `#5501`; `#5361` must not close until that
contract is available or explicitly blocked with evidence.

No runtime deployment claim is valid until exact revision, install, operation,
and rollback evidence are recorded.

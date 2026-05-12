# Card Bundle Readiness - v0.91.2

## Status

Active. The v0.91.2 WP issue wave and sprint umbrella issue wave are open.

## Current State

`v0.91.2` now has an opened issue wave:

- WP-01 is `#3000`
- WP-02 through WP-24 are `#3001` through `#3023`
- Sprint umbrellas are `#3025` through `#3028`
- Sidecar issue `#3024` is an unscheduled docs cleanup issue, not part of the
  canonical WP/sprint sequence

The full STP/SIP/SPP/SRP/SOR bundle set exists in the durable primary-checkout
local bundle root at `.adl/v0.91.2/tasks/` for the opened WP issues and sprint
umbrella issues before execution binding. Those `.adl` bundles are local
workflow records, not tracked PR files.

## Opening Rule

Opening pass result:

- each WP received STP, SIP, SPP, SRP, and SOR cards
- each sprint umbrella received STP, SIP, SPP, SRP, and SOR cards
- bundle validation was recorded during WP-01
- the package was updated from planned to active state

## Validation Record

Command:

```bash
for issue in {3000..3028}; do
  if [[ "$issue" == "3024" ]]; then
    expected_kind="sidecar"
  fi
  d="$(find .adl/v0.91.2/tasks -maxdepth 1 -type d -name "issue-${issue}__*" | head -1)"
  test -n "$d"
  for t in stp sip spp srp sor; do
    f="$d/$t.md"
    test -s "$f"
    if [[ "$t" == sip || "$t" == sor ]]; then
      ADL_TOOLING_RUST_BIN=adl/target/debug/adl \
        bash adl/tools/validate_structured_prompt.sh \
          --type "$t" --phase bootstrap --input "$f" >/dev/null
    else
      ADL_TOOLING_RUST_BIN=adl/target/debug/adl \
        bash adl/tools/validate_structured_prompt.sh \
          --type "$t" --input "$f" >/dev/null
    fi
  done
done
```

Result: `PASS issue-30xx v0.91.2 task cards`

## Non-Claims

- this doc does not claim any WP implementation has started
- this doc does not claim sprint execution has started
- this doc does not claim sidecar issue `#3024` is part of the canonical WP or
  sprint issue wave

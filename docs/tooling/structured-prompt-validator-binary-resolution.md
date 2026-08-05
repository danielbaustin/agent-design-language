# Structured Prompt Validation Boundary

`adl/tools/validate_structured_prompt.sh` and the former
`adl-validate-structured-prompt` binary are deleted v1-era surfaces. They must
not be used by current bootstrap, review, or publication flows, and the wrapper
must not be recreated.

Current C-SDLC v2 card validation is owned by the typed Rust v2 command set:

```sh
csdlc-validate --root <worktree> --request <execution-request.json>
csdlc-validate --root <worktree> finalize --request <finalize-request.json>
```

For semantic card edits, use `csdlc-edit` first, then validate the resulting
typed issue state. New cards should come from the active prompt-template
registry through the v2 renderer/importer path; do not validate newly generated
work by shelling out to the deleted wrapper.

## Bootstrap Rule

Bootstrap or review-prep code that needs lifecycle validation must create or
load typed C-SDLC issue state and run `csdlc-validate`. If the issue is not yet
typed, initialize it through `csdlc-init`; do not call deleted ADL shell
wrappers as a precondition to lifecycle creation.

## Regression Surface

Gate 10A checks current v2 guidance for references to the deleted wrapper. It
does not rewrite historical milestone evidence or immutable legacy records.

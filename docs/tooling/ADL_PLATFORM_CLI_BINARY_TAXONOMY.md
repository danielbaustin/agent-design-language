# ADL Platform CLI Binary Taxonomy

ADL separates command families so each owner has a bounded operational and
validation surface.

## Platform Owners

| Owner surface | Command family | Current status |
|---|---|---|
| ADL language and compiler | `adl` | canonical |
| C-SDLC workflow lifecycle | typed `csdlc-*` binaries under `csdlc-v2/` | canonical under Gate 10D2 |
| Cognitive Spacetime runtime | `csm` | canonical |
| CSM administration | `csmctl` | planned; no C-SDLC commands belong here |
| Runtime compatibility | `adl-runtime` | compatibility surface outside C-SDLC v2 |
| Review compatibility | `adl-review` | compatibility surface outside C-SDLC v2 |

## C-SDLC V2 Authority

Gate 10D2 records `v1_sunset`. The sole current C-SDLC operational authority is
the independent Rust binary set in `csdlc-v2/`, routed through the eleven typed
skills in `csdlc-v2/operator/skills/`.

Current lifecycle owners are:

- `csdlc-init`
- `csdlc-bind`
- `csdlc-edit`
- `csdlc-doctor`
- `csdlc-validate`
- `csdlc-review`
- `csdlc-publish`
- `csdlc-finish`
- `csdlc-clean`
- `csdlc-shepherd`

Resolve the selected generation through `csdlc-install resolve`, then invoke
the typed owner selected by the matching operator skill. Stable generated v2
binaries belong under `.adl/bin/csdlc-v2/`; Cargo target directories are build
output, not operational authority.

There is no canonical monolithic `csdlc` lifecycle command. The removed v1
`pr.sh` wrappers, prompt-template wrappers, `csdlc-import`, and `adl-csdlc`
compatibility route are not valid operator paths.

## Validation Boundary

Validate the typed owner and contract touched by a change. Use the focused
C-SDLC owner lane when broader integration proof is required:

```bash
bash adl/tools/run_owner_validation_lane.sh csdlc
git diff --check
```

Do not restore sunset commands to make historical examples pass. Historical
Gate 10A-C artifacts remain evidence and do not override Gate 10D2 authority.

## Non-Claims

- This taxonomy does not make `csmctl` ready.
- It does not move runtime or review compatibility commands into C-SDLC v2.
- It does not authorize direct card Markdown mutation; typed card edits remain
  governed by `csdlc-edit` and `csdlc-validate`.

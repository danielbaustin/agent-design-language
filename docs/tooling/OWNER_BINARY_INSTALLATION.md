# ADL Owner Binary Installation

ADL operational owner binaries are generated tools. They are not committed to
Git, and they are not owned by Cargo `target/` directories.

The stable local install location is:

```text
.adl/bin/csdlc-v2/
```

For C-SDLC v2, use the reviewed installer:

```sh
csdlc-install install --repo . --destination .adl/bin/csdlc-v2
```

Then verify the installed generation against the checked-in coexistence
inventory:

```sh
csdlc-install verify --repo . --bin-dir .adl/bin/csdlc-v2 --inventory csdlc-v2/operator/coexistence.json
```

The v2 installer builds from reviewed source into a disposable Cargo target and
copies only the manifest-required binaries into the dedicated generation
directory. It fails closed when source inputs are dirty, required binaries are
missing, the destination is not named `csdlc-v2`, or the coexistence inventory is
not the embedded reviewed inventory.

Current C-SDLC v2 GitHub owner binaries include:

- `csdlc-github`
- `csdlc-github-issue`
- `csdlc-github-pr`
- `csdlc-pr-state`
- `csdlc-finish`
- `csdlc-clean`

`csdlc-github` remains a compatibility facade. New issue actions should route
through `csdlc-github-issue`; PR observation should route through
`csdlc-github-pr` or the dedicated `csdlc-pr-state` observer. `csdlc-finish` is
the sole terminal operator route: it recognizes an already-terminal issue or
performs the exact-head merge, then retains only a rebuildable derived terminal
cache.

`csdlc-clean` is the independent safe-cleanup and legacy compatibility route.
It never supplies merge or issue-closure authority, and it never force-removes
a dirty, missing, relocated, primary, or identity-drifted worktree.

Cargo `target/` directories remain build/cache output only. They may be deleted
or pruned without taking the operational command surface with them. C-SDLC v2
workflow commands should resolve through `.adl/bin/csdlc-v2/` before considering
Cargo `target/debug` output.

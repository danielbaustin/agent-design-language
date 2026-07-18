# GitHub Actions Runtime Pin Inventory

Issue #5463 upgrades the action revisions named by GitHub-hosted run
`29632957768`. Its check annotations reported that GitHub was forcing three
Node.js 20 actions to run on Node.js 24. The repository continues to use full,
immutable commit SHAs rather than floating tags.

| Action | Deprecated revision | Reviewed replacement | Runtime authority |
| --- | --- | --- | --- |
| `actions/checkout` | `34e114876b0b11c390a56381ad16ebd13914f8d5` (`v4`) | `9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0` (`v7.0.0`) | [`action.yml` declares `node24`](https://github.com/actions/checkout/blob/9c091bb21b7c1c1d1991bb908d89e4e9dddfe3e0/action.yml) |
| `actions/upload-artifact` | `ea165f8d65b6e75b540449e92b4886f43607fa02` (`v4`) | `043fb46d1a93c77aae656e7c1c64a875d1fc6a0a` (`v7.0.1`) | [`action.yml` declares `node24`](https://github.com/actions/upload-artifact/blob/043fb46d1a93c77aae656e7c1c64a875d1fc6a0a/action.yml) |
| `Swatinem/rust-cache` | `779680da715d629ac1d338a641029a2f4372abb5` (`v2`) | `c19371144df3bb44fab255c43d04cbc2ab54d1c4` (`v2.9.1`) | [`action.yml` declares `node24`](https://github.com/Swatinem/rust-cache/blob/c19371144df3bb44fab255c43d04cbc2ab54d1c4/action.yml) |

The focused contract `adl/tools/test_ci_runtime_contracts.sh` scans every
checked-in workflow and fails if any of these actions uses a noncanonical
revision, if a deprecated revision remains, or if an inventoried action
unexpectedly disappears. GitHub-hosted PR checks are the final proof that the
Node.js 20 deprecation annotation is absent.

The two AWS-named workflow files are included in the static pin inventory so
their checked-in action metadata stays current. Issue #5463 does not dispatch
them and performs no AWS-backed validation.

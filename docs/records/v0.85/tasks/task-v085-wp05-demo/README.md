# task-v085-wp05-demo

This tracked task bundle is the canonical public proof surface for the current v0.85 editor slice.

It demonstrates the real public-record layout:

- `stp.md`
- `sip.md`
- `sor.md`

And it proves the current bounded editor loop:

- STP can be authored in the linked workspace
- SIP can be refined in the linked workspace
- SOR can be reviewed in the linked workspace
- the editor can prepare a bounded `pr start` command
- reviewer handoff can be summarized from the SOR without rebuilding context manually

This bundle is intentionally bounded.

It does not claim that:

- the browser writes tracked files directly
- `pr run` or `pr finish` are launched from the browser
- the full long-term HTA platform already exists

Use this bundle together with `docs/tooling/editor/demo.md` as the auditable proof surface for the current editor tranche.

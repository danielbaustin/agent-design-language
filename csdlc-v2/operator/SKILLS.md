# C-SDLC v2 operator skills

The nine skills in `skills.json` are thin typed routes. Skills select a binary/subcommand, collect typed input, and display typed output. They never edit Markdown, mutate canonical state directly, invoke shell/Python lifecycle logic, or infer success from prose.

The tracked `generation-selector.json` is the sole default authority. After reviewed Gate 10C cutover it may select v2, while explicit v1 override, installation, and recovery remain mandatory. Install only into `.adl/bin/csdlc-v2/`, never shared `.adl/bin/`. `csdlc-install verify` fails unless the v1 coexistence inventory and regular executable v2 binary set are complete.

The read-only doctor route also installs `csdlc-eligibility` as an auxiliary
operator binary. It may write only its requested decision output; it has no
authority to mutate candidate v1 paths or execute an eligible manifest.

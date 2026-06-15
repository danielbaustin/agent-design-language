# Release And Watcher GitHub Gap Inventory For #3712

Issue: #3712
Date: 2026-06-15
Scope: release and watcher GitHub operations after #3697 / PR #3702

## Summary

#3697 made the primary issue/PR workflow transport operational through Rust and
octocrab. This inventory records the remaining release and watcher surfaces so
they are not hidden inside a successful octocrab transport claim.

The current state is mostly safe but incomplete:

- Release ceremony tag operations still use `git`, which is expected.
- GitHub Release draft/publish operations fail closed in the shell helper.
- Issue watcher and PR janitor are skill contracts, not native Rust/octocrab
  execution surfaces.
- Native GitHub Release draft/publish and watcher metadata support is routed to
  follow-up issue #3718.
- The watcher skill contract now forbids `gh` as an operational backend for
  covered ADL-owned workflow state.

## Inventory

| Surface | Current GitHub behavior | Status | Required routing |
| --- | --- | --- | --- |
| `adl/tools/release_ceremony.sh --create-tag` | Local git tag creation only. | local-only utility | No octocrab work required. |
| `adl/tools/release_ceremony.sh --push-tag` | Uses `git push origin <tag>`. | local git remote operation | No `gh` dependency; keep as git operation unless release policy changes. |
| `adl/tools/release_ceremony.sh --draft-release` | Fails closed with a message that GitHub draft release creation must be implemented through Rust/octocrab before use. | explicit fail-closed gap | Follow-up issue #3718: implement native GitHub Release draft creation before using this flag. |
| `adl/tools/release_ceremony.sh --publish-release` | Fails closed with a message that GitHub release publication must be implemented through Rust/octocrab before use. | explicit fail-closed gap | Follow-up issue #3718: implement native GitHub Release publication before using this flag. |
| `adl/tools/test_release_ceremony.sh` | Uses a local fake `gh` fixture to test historical release behavior. | test fixture only | Keep isolated from operational claims; future release implementation should add octocrab-backed tests. |
| `adl/tools/skills/issue-watcher/SKILL.md` | Requires repo-native GitHub metadata surfaces for covered ADL-owned workflow state and forbids `gh` fallback for covered paths. | skill contract tightened; native watcher implementation still pending | Follow-up issue #3718: implement native watcher metadata support where the skill needs live GitHub state. |
| `adl/tools/skills/pr-janitor/SKILL.md` | Describes PR monitoring policy and can inspect GitHub state; native workflow paths now rely on #3697 for covered operations. | partially covered through C-SDLC PR path | Keep PR workflow claims limited to covered issue/PR operations. |
| `adl/tools/attach_pr_janitor.sh` | Starts a Codex janitor process with a structured skill prompt; no direct GitHub API or `gh` call. | launcher / local process utility | No octocrab transport claim; watcher/janitor implementation remains skill-mediated. |

## Checklist Disposition

The checklist item “Remaining GitHub release or watcher gaps are routed to
explicit follow-up work rather than hidden inside successful workflow claims” is
now satisfied for #3712 because the gaps are explicitly classified above and
native implementation work is routed to #3718.

This issue does not implement GitHub Release draft/publish support. It records
that those flags are fail-closed and must remain non-claims until a native
Rust/octocrab release implementation exists through #3718 or a later split from
that issue.

## Validation

Focused validation for this issue should prove:

- release draft/publish flags still fail closed from shell;
- issue-watcher contract tests still pass;
- no direct operational `gh` command is introduced for release/watcher paths;
- the integration checklist points to this explicit follow-up route.

## Non-claims

- This is not release approval.
- This does not publish a GitHub Release.
- This does not implement a native release API client.
- This does not turn issue-watcher or PR-janitor into long-running daemons.

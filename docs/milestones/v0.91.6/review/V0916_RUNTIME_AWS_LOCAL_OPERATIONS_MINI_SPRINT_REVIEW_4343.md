# v0.91.6 Runtime AWS / Local Operations Mini-Sprint Review

Issue: `#4343`
Date: 2026-06-20
Status: retained sprint review packet

## Scope

This packet reviews the Runtime AWS / local operations mini-sprint umbrella
after its child issue set closed.

Child issues:

| Issue | Surface | State | Primary evidence |
| --- | --- | --- | --- |
| `#4284` | Wuji dynamic IP Route 53 updater | closed | `infra/ddns/`, issue closure, DDNS validation hooks in `adl/src/cli/pr_cmd/finish_support.rs` |
| `#4330` | Wuji DDNS client and `launchd` automation | closed | `infra/ddns/client/`, issue closure, DDNS client validation hooks in `adl/src/cli/pr_cmd/finish_support.rs` |
| `#4318` | `nessus.local` SSM managed-node enrollment | closed | `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_PROOF_4318.md` |
| `#4319` | `opticon.local` SSM managed-node enrollment | closed | `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_PROOF_4319.md`; `docs/tooling/QNAP_QTS_SSM_ONBOARDING.md` |
| `#4320` | Codex login preparation for `nessus.local` | closed | sanitized issue closeout comment on `#4320` |
| `#4321` | Codex login preparation for `opticon.local` | closed | sanitized issue closeout comment on `#4321` |

## Findings

No retained sprint-blocking findings remain for the umbrella scope.

The child issue set is closed, and the tracked evidence is sufficient for a
mini-sprint closeout claim that DDNS, host enrollment, and access preparation
were coordinated as one bounded operations tranche.

## Review Result

Result: `pass_with_residual_risk`

The sprint may close with explicit limits:

- DDNS proof is represented by the tracked `infra/ddns/` implementation,
  associated client install surfaces, issue closure truth, and finish validation
  hooks. This packet does not add a separate DDNS live-proof artifact.
- `nessus.local` and `opticon.local` SSM proof packets record bounded managed
  node registration, read-only status commands, CloudWatch log proof, and
  host-specific runbooks.
- Codex login preparation for both hosts is recorded through sanitized issue
  comments rather than credential-bearing tracked artifacts.

## Authority Boundaries

The sprint preserves the following boundaries:

- AWS SSM and DDNS are operations-plane surfaces.
- AWS does not become authority for polis state, identity, governance, memory,
  scheduler decisions, provider selection, or model contents.
- SSM command surfaces are bounded status/proof paths, not provider execution
  or scheduler authority.
- DDNS keeps `wuji.agent-logic.ai` current but does not create broad fleet DNS
  automation.
- Host login preparation does not publish passwords, private keys, recovery
  material, private host paths, SCR private material, storage listings, model
  contents, AWS account identifiers, hosted zone identifiers, bearer tokens, or
  activation material.

## Validation And Evidence

Retained proof surfaces:

- `infra/ddns/README.md`
- `infra/ddns/lambda/handler.py`
- `infra/ddns/tests/test_handler.py`
- `infra/ddns/client/wuji_ddns_update.sh`
- `infra/ddns/client/install_wuji_ddns_launchd.sh`
- `infra/ddns/client/com.agentlogic.wuji-ddns.plist`
- `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_OPERATIONS_BRIDGE_4109.md`
- `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_PROOF_4113.md`
- `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_PROOF_4318.md`
- `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_PROOF_4319.md`
- `docs/tooling/QNAP_QTS_SSM_ONBOARDING.md`

Focused sprint-review validation for this packet:

- Live GitHub child issue state check for `#4284`, `#4330`, `#4318`, `#4319`,
  `#4320`, and `#4321`.
- Live GitHub issue-comment check for `#4320` and `#4321` to verify the
  sanitized access-preparation closeout comments exist without copying
  credential material into tracked docs.
- Repository scan for retained DDNS, SSM, and access-preparation evidence.
- `git diff --check`.
- Structured SOR validation for issue `#4343`.

## Residual Risk

- DDNS live-operation proof is not retained as a standalone review packet under
  `docs/milestones/v0.91.6/review/`; reviewers should consume the issue/PR
  closure truth and `infra/ddns/` validation surfaces for that slice.
- The SSM proofs depend on host-local and AWS-local operational state that is
  intentionally not fully reproduced in public tracked artifacts.
- QTS onboarding remains host-specific; the `opticon.local` proof should not be
  generalized to all NAS or Linux hosts without a follow-on installer/runbook
  proof.
- Credentials and host login details remain privately controlled by the
  operator, so tracked review can verify custody boundaries but not reproduce
  login setup from public artifacts.

## Follow-Up Routing

No follow-up blocks sprint closure.

Later work, if needed, should be routed separately:

- reusable multi-host DDNS rollout;
- fleet-scale SSM onboarding;
- non-AWS cloud or edge-node managed-node proof;
- host metrics or CloudWatch Agent rollout;
- scheduler integration for operations tasks;
- provider-health collection through local ADL-owned commands.

## Non-Claims

This packet does not claim:

- live fleet automation;
- Session Manager interactive shell as an approved workflow;
- AWS authority over polis state;
- provider/model execution through AWS SSM;
- scheduler-owned remote execution;
- SCR archive or storage inventory export;
- publication of credentials, tokens, account ids, hosted zone ids, activation
  material, private keys, passwords, or private host inventories.

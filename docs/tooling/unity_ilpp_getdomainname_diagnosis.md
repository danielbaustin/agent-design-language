# Unity ILPP `GetDomainName` diagnosis

Issue: #5332

## Result

The Unity 6000.5.1f1 Observatory batch loop was isolated to the execution
sandbox, not to the project, mutable Unity state, or the ILPP socket path.

Inside the restricted command lane, macOS `getdomainname(2)` returns `EPERM`.
Unity's .NET ILPP trigger initializes `System.Net.CookieContainer`, which calls
that host-identity API before opening its gRPC connection. The trigger then
repeats `GetDomainName: -1` while the ILPP runner is already listening.

The wrapper now probes this capability before staging or launching Unity and
returns `sandbox_host_identity_denied` immediately. The fix is to run the same
issue-bound wrapper through an operator-approved host execution lane. No broad
process scan, hostname mutation, asset mutation, or Unity binary replacement is
required.

The diagnostic matrix runs all three declared cells by default and records the
Unity version, canonical project, single changed variable, first and last
semantic progress, signature counts, terminal classifier, exit status, and
issue-local log references. Its run root is canonicalized before use so path
traversal and symlink escapes are rejected. Large staged Unity state remains on
FastWork; the small matrix logs remain under the issue worktree `.adl`
directory.

## Focused Evidence

The current restricted-lane matrix establishes one precondition consistently:

| Cell | Observed result |
| --- | --- |
| isolated mutable state | `sandbox_host_identity_denied` before staging |
| host HOME only | `sandbox_host_identity_denied` before staging |
| system temp only | `sandbox_host_identity_denied` before staging |

Because the capability check intentionally runs before staging, these three
restricted cells do not claim differentiated Unity or ILPP outcomes. The
approved-host isolated run completed ILPP, created the staged project, and ran
the existing validator.

The approved-host run crossed ILPP and exposed the canonical project's separate
third-party asset boundary. The operator-provisioned project at
`/Volumes/FastWork/adl-unity-observatory/operator-provisioned-5332/unity-observatory`
was then reconstructed from the licensed Unity Asset Store cache, rebuilt with
`UnityObservatoryFlagshipStageBuilder.EnsureFlagshipStage`, and validated with
`UnityObservatoryBatchValidator.ValidateScene`.

The wrapper treats a zero Unity exit plus the validator success marker as the
terminal proof. The validator itself compares the generated title, packet
reference, artifact root, report reference, and evidence level against the
environment supplied by the wrapper; it does not echo those values into the
Unity log.

Retained issue-local logs record:

- `ADL flagship observatory stage validation passed`
- `prefabInstances=43; gameObjects=79; cameras=4; lights=7`
- `Unity Observatory batch validation passed for the shell and flagship environment`

The imported 5.6G asset payload and generated 4.0G Unity `Library` remain on
FastWork only. They are not Git payloads and do not change the #4745 licensing
or redistribution policy.

## Reproduction

Run deterministic proof first:

```sh
bash adl/tools/test_v0916_unity_observatory_local_runtime_consumption_unit.sh
bash adl/tools/test_select_validation_lanes.sh
```

Run the complete diagnostic matrix:

```sh
bash adl/tools/run_v0918_unity_ilpp_diagnostic_matrix.sh
```

Run one explicit cell while investigating a changed condition:

```sh
bash adl/tools/run_v0918_unity_ilpp_diagnostic_matrix.sh host_home
```

If it reports `sandbox_host_identity_denied`, rerun that exact command through
the operator-approved host lane. A successful host-identity probe removes the
ILPP blocker; it does not by itself prove the scene, runtime shell, or imported
asset state.

## Non-Claims

- This diagnosis does not modify Unity-MCP endpoint or project binding.
- It does not commit or redistribute third-party Unity assets.
- It does not redefine the Observatory validator or scene semantics.
- Only the retained full validator success proves the reconstructed local
  shell-plus-flagship project.

## Tooling Follow-Ups Captured On #5332

- The stable `.adl/bin/csdlc-v2` installation omitted `csdlc-merge`; #4741
  closeout required an existing repository-built FastWork binary.
- The stable `.adl/bin/adl` provenance did not match the current source tree;
  this diagnostic used the repository binary only with the explicit
  test-owner-binary override and did not rebuild or replace it.
- The connected GitHub write integration returned `403 Resource not accessible
  by integration` for both follow-on issue creation and issue comments. No raw
  `gh` fallback was used. These defects remain attached to #5332 evidence until
  a typed issue-writing route can split them safely.

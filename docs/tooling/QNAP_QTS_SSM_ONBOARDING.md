# QNAP QTS SSM Onboarding Notes

This guide captures the QNAP/QTS-specific AWS Systems Manager onboarding path
proven during `#4319` on `opticon.local`.

It exists because QTS does not behave like a normal Linux host for SSM worker
installation or shell-based Run Command execution.

## Scope

This guide covers:
- hybrid-node enrollment for a QTS host
- the worker-binary repair needed when the root filesystem is too small
- the QTS-safe direct-command execution model for `AWS-RunShellScript`

This guide does not cover:
- fleet rollout
- Session Manager shell use
- CloudWatch Agent rollout
- storage export or archive export

## Proven host shape

The proven host characteristics were:
- OS: `QTS 5.0.1`
- arch: `aarch64`
- root filesystem was small enough that copying the full worker payload into
  `/usr/bin` produced truncated files
- `/bin/sh -lc ...` invoked the QTS console management menu instead of acting
  like a normal shell proof surface

## Required high-level approach

1. Create the hybrid activation in the intended AWS account.
2. Download the Linux ARM64 SSM RPM.
3. Extract the worker payload into a persistent path under
   `/share/Public/ssm-stage/usr/bin`.
4. Install the core agent under `/opt/aws/ssm/amazon-ssm-agent`.
5. Register the host directly with `amazon-ssm-agent -register`.
6. Replace `/usr/bin/ssm-*` executables with symlinks to the staged binaries
   under `/share/Public/ssm-stage/usr/bin`.
7. Start the core agent with BusyBox `setsid`.
8. Use direct executable/script paths for Run Command proofs.

## Why the staged symlink repair is required

QTS may have too little free space on `/usr` for the full worker payload.

If the worker binaries are copied directly into `/usr/bin`, partial copies can
leave corrupted executables such as:
- truncated `ssm-document-worker`
- zero-byte `ssm-session-logger`
- missing or unusable `ssm-session-worker`

The proven repair is:
- keep the extracted full binaries under `/share/Public/ssm-stage/usr/bin`
- symlink `/usr/bin/ssm-agent-worker`
- symlink `/usr/bin/ssm-document-worker`
- symlink `/usr/bin/ssm-session-worker`
- symlink `/usr/bin/ssm-session-logger`
- symlink `/usr/bin/ssm-cli`

This avoids exhausting the small root filesystem while preserving the execution
paths the agent expects.

## Why direct command execution is required

On the proven QTS host, generic shell-wrapped probes such as:

```sh
/bin/sh -lc "echo 1"
```

did not behave like normal shell commands. They invoked the QTS console
management UI and produced tty-related stderr.

For deterministic `AWS-RunShellScript` proof on QTS:
- use direct executable paths like `/bin/echo 1`, or
- use a direct script path like `/share/Public/adl-4319-polis-status-for-ssm-qts.sh`

Do not assume `/bin/sh -lc` is a safe wrapper on QTS.

## Minimal host-side repair checklist

1. Ensure the staged worker payload exists under `/share/Public/ssm-stage/usr/bin`.
2. Remove broken copies from `/usr/bin`.
3. Replace them with symlinks to the staged binaries.
4. Clear stale IPC state under `/var/lib/amazon/ssm/ipc`.
5. Start the agent with BusyBox `setsid`.
6. Verify managed-node identity with `ssm-cli get-instance-information`.
7. Verify a trivial direct Run Command such as `/bin/echo 1`.
8. Only then run the tracked bounded proof wrapper.

## Durable cautions

- Use the intended AWS profile/account explicitly when creating activations and
  validating managed-node state.
- Do not expose activation codes or IDs in tracked artifacts.
- Treat QTS shell behavior as host-specific and potentially surprising.
- Prefer direct script/executable paths over shell wrappers for repeatable
  automation on QTS.

## Related proof

The full bounded proof packet for the proven host is:
- `docs/milestones/v0.91.6/review/security/LOCAL_POLIS_SSM_PROOF_4319.md`

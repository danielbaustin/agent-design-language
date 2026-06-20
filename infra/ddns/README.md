# Wuji Dynamic DNS

This directory contains the Phase 1 Terraform-managed DDNS slice for
`wuji.agent-logic.ai`.

## Tracked Source Layout

- `lambda/handler.py`: Lambda Function URL handler.
- `tests/test_handler.py`: focused local proof for auth, idempotence, UPSERT, and
  sanitized logging behavior.
- `client/wuji_ddns_update.sh`: Wuji-side client that calls the Lambda URL over
  HTTPS without AWS credentials.
- `client/com.agentlogic.wuji-ddns.plist`: `launchd` template for five-minute
  execution and reboot survival.
- `*.tf`: Terraform package for the DDNS infrastructure.

## Local-Only Terraform Artifacts

Keep these paths untracked:

- `infra/ddns/.terraform/`
- `infra/ddns/tfplan`
- `infra/ddns/build/`

The tracked package keeps local Lambda zip output under `infra/ddns/build/` so
the Terraform surface stays self-contained without turning build byproducts into
review artifacts.

## Bootstrap Boundary

This slice now creates the bearer token via Terraform so the DDNS service can
be stood up from a single apply.

- If `ddns_bearer_token` is unset, Terraform generates a strong token and
  writes it into the SecureString parameter.
- If `ddns_bearer_token` is supplied through a secure local variable source,
  Terraform writes that exact value instead.

The token is still secret material and therefore lives in Terraform state. Use
secure state custody before broad operational rollout.

The current provider-compatible Function URL permission bridge also requires
local `aws` CLI and `jq` availability at apply time because Terraform must use
the tracked `local-exec` compatibility shim until the provider surface can own
`invoked_via_function_url` directly.

## Required Operator Inputs

Before Terraform apply, provide:

- `route53_hosted_zone_id`
- `allowed_hostnames` if different from the default
- optionally `ddns_bearer_token` if you want to supply the token instead of
  letting Terraform generate one

## Validation

Run the focused local proof set:

```sh
python3 -m unittest infra/ddns/tests/test_handler.py
terraform -chdir=infra/ddns fmt -check
terraform -chdir=infra/ddns init -backend=false
terraform -chdir=infra/ddns validate
```

If you want a local plan after supplying real variable values:

```sh
terraform -chdir=infra/ddns plan -out=tfplan
```

## Apply Flow

1. Run Terraform validation.
2. Run `terraform -chdir=infra/ddns plan -out=tfplan`.
3. Review the plan.
4. Apply with `terraform -chdir=infra/ddns apply -auto-approve tfplan`.
   This compatibility path currently expects `aws` and `jq` to be installed on
   the operator machine because the apply uses a bounded `local-exec` bridge for
   the missing Function URL invoke-via-URL permission surface.
5. Retrieve the token for Wuji from Terraform output and write it to the local
   token file.
6. Install the Wuji client script and filled-in plist.
7. Load the plist with `launchctl bootstrap` or `launchctl kickstart`.

If you are using local state during bootstrap, capture the token immediately
after apply and then move to a secure backend or other secure state custody.

## Wuji Client Install

Preferred path:

```sh
infra/ddns/client/install_wuji_ddns_launchd.sh
```

This installer:

- reads `lambda_function_url` and `ddns_bearer_token` from Terraform output
- falls back to the live AWS Function URL and SSM SecureString when Terraform
  CLI access or state custody lives elsewhere, as long as the local DDNS
  package checkout is present
- installs `wuji_ddns_update.sh` to `~/.local/bin/wuji_ddns_update.sh`
- writes the token to `~/.config/wuji-ddns/token` with `0600` permissions
- renders the launch agent into
  `~/Library/LaunchAgents/com.agentlogic.wuji-ddns.plist`
- loads and kickstarts the launch agent for the current user
- runs one immediate updater proof after installation

Useful options:

```sh
infra/ddns/client/install_wuji_ddns_launchd.sh --no-load
infra/ddns/client/install_wuji_ddns_launchd.sh --terraform-dir /path/to/infra/ddns
infra/ddns/client/install_wuji_ddns_launchd.sh --aws-region us-west-2
```

When using the AWS fallback path, the local checkout must still include
`infra/ddns/client/wuji_ddns_update.sh` and
`infra/ddns/client/com.agentlogic.wuji-ddns.plist` because the installer copies
those repo-managed client artifacts onto Wuji.

Manual fallback:

1. Copy `client/wuji_ddns_update.sh` to a stable local path on Wuji.
2. Write the Terraform-managed token to a local file with restrictive
   permissions, for example:

```sh
mkdir -p ~/.config/wuji-ddns
terraform -chdir=infra/ddns output -raw ddns_bearer_token > ~/.config/wuji-ddns/token
chmod 600 ~/.config/wuji-ddns/token
```

This command requires access to the same Terraform state that created the
token. If local state is only temporary, export the token before removing or
relocating that state.

3. Replace the placeholders in `client/com.agentlogic.wuji-ddns.plist`:
   - `__WUJI_DDNS_SCRIPT_PATH__`
   - `__WUJI_DDNS_FUNCTION_URL__`
   - `__WUJI_DDNS_TOKEN_FILE__`
   - `__WUJI_DDNS_LOG_DIR__`
4. Install the plist under `~/Library/LaunchAgents/`.
5. Start it:

```sh
launchctl bootout gui/"$(id -u)" ~/Library/LaunchAgents/com.agentlogic.wuji-ddns.plist 2>/dev/null || true
launchctl bootstrap gui/"$(id -u)" ~/Library/LaunchAgents/com.agentlogic.wuji-ddns.plist
launchctl kickstart -k gui/"$(id -u)"/com.agentlogic.wuji-ddns
```

## Failure Behavior

- ISP address changes: the next scheduled run updates Route 53.
- Lambda failure: DNS stays unchanged and the next scheduled run retries.
- Route 53 failure: Lambda returns failure and CloudWatch logs capture the
  error.
- Wuji offline: DNS remains on the last successful IP until Wuji returns.

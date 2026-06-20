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
5. Retrieve the token for Wuji from Terraform output and write it to the local
   token file.
6. Install the Wuji client script and filled-in plist.
7. Load the plist with `launchctl bootstrap` or `launchctl kickstart`.

If you are using local state during bootstrap, capture the token immediately
after apply and then move to a secure backend or other secure state custody.

## Wuji Client Install

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

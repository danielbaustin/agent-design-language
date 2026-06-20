#!/bin/sh
set -eu

usage() {
  cat <<'EOF'
Usage: install_wuji_ddns_launchd.sh [--terraform-dir <dir>] [--hostname <hostname>] [--aws-region <region>] [--lambda-function-name <name>] [--token-parameter-name <name>] [--no-load]

Installs the Wuji DDNS updater script, token file, and launchd plist for the
current user by reading Terraform outputs from the DDNS stack.
EOF
}

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname "$0")" && pwd)
REPO_ROOT=$(CDPATH= cd -- "$SCRIPT_DIR/../../.." && pwd)
TERRAFORM_DIR="$REPO_ROOT/infra/ddns"
HOSTNAME_VALUE="wuji.agent-logic.ai"
AWS_REGION="us-west-2"
LAMBDA_FUNCTION_NAME="wuji-agent-logic-ddns-updater"
TOKEN_PARAMETER_NAME=""
LOAD_AGENT=1

while [ "$#" -gt 0 ]; do
  case "$1" in
    --terraform-dir)
      shift
      [ "$#" -gt 0 ] || {
        echo "--terraform-dir requires a value" >&2
        exit 2
      }
      TERRAFORM_DIR="$1"
      ;;
    --hostname)
      shift
      [ "$#" -gt 0 ] || {
        echo "--hostname requires a value" >&2
        exit 2
      }
      HOSTNAME_VALUE="$1"
      ;;
    --aws-region)
      shift
      [ "$#" -gt 0 ] || {
        echo "--aws-region requires a value" >&2
        exit 2
      }
      AWS_REGION="$1"
      ;;
    --lambda-function-name)
      shift
      [ "$#" -gt 0 ] || {
        echo "--lambda-function-name requires a value" >&2
        exit 2
      }
      LAMBDA_FUNCTION_NAME="$1"
      ;;
    --token-parameter-name)
      shift
      [ "$#" -gt 0 ] || {
        echo "--token-parameter-name requires a value" >&2
        exit 2
      }
      TOKEN_PARAMETER_NAME="$1"
      ;;
    --no-load)
      LOAD_AGENT=0
      ;;
    --help|-h)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 2
      ;;
  esac
  shift
done

terraform_output_raw() {
  [ -d "$TERRAFORM_DIR" ] || return 1
  command -v terraform >/dev/null 2>&1 || return 1
  terraform -chdir="$TERRAFORM_DIR" output -raw "$1" 2>/dev/null || return 1
}

terraform_output_value() {
  tf_value="$(terraform_output_raw "$1" || true)"
  case "$tf_value" in
    ""|"null")
      return 1
      ;;
    *)
      printf '%s\n' "$tf_value"
      return 0
      ;;
  esac
}

resolve_function_url() {
  if tf_value="$(terraform_output_value lambda_function_url)"; then
    printf '%s\n' "$tf_value"
    return 0
  fi

  command -v aws >/dev/null 2>&1 || {
    echo "aws CLI is required when Terraform outputs are unavailable" >&2
    exit 1
  }
  aws --region "$AWS_REGION" lambda get-function-url-config \
    --function-name "$LAMBDA_FUNCTION_NAME" \
    --query FunctionUrl \
    --output text
}

resolve_token_parameter_name() {
  if [ -n "$TOKEN_PARAMETER_NAME" ]; then
    printf '%s\n' "$TOKEN_PARAMETER_NAME"
    return 0
  fi

  if tf_value="$(terraform_output_value token_parameter_name)"; then
    printf '%s\n' "$tf_value"
    return 0
  fi

  command -v aws >/dev/null 2>&1 || {
    echo "aws CLI is required when Terraform outputs are unavailable" >&2
    exit 1
  }
  aws --region "$AWS_REGION" lambda get-function-configuration \
    --function-name "$LAMBDA_FUNCTION_NAME" \
    --query 'Environment.Variables.DDNS_TOKEN_PARAMETER' \
    --output text
}

write_token_file() {
  token_tmp="$(mktemp "$(dirname "$TOKEN_FILE")/token.tmp.XXXXXX")"
  cleanup_token_tmp() {
    rm -f "$token_tmp"
  }
  trap cleanup_token_tmp EXIT INT TERM

  if tf_value="$(terraform_output_value ddns_bearer_token)"; then
    printf '%s' "$tf_value" > "$token_tmp"
    mv "$token_tmp" "$TOKEN_FILE"
    trap - EXIT INT TERM
    return 0
  fi

  command -v aws >/dev/null 2>&1 || {
    echo "aws CLI is required when Terraform outputs are unavailable" >&2
    exit 1
  }

  parameter_name="$(resolve_token_parameter_name)"
  aws --region "$AWS_REGION" ssm get-parameter \
    --with-decryption \
    --name "$parameter_name" \
    --query Parameter.Value \
    --output text > "$token_tmp"
  [ -s "$token_tmp" ] || {
    echo "Retrieved empty DDNS token for $parameter_name" >&2
    exit 1
  }
  mv "$token_tmp" "$TOKEN_FILE"
  trap - EXIT INT TERM
}

UPDATER_SOURCE="$REPO_ROOT/infra/ddns/client/wuji_ddns_update.sh"
PLIST_TEMPLATE="$REPO_ROOT/infra/ddns/client/com.agentlogic.wuji-ddns.plist"

[ -f "$UPDATER_SOURCE" ] || {
  echo "Updater source not found: $UPDATER_SOURCE" >&2
  exit 1
}

[ -f "$PLIST_TEMPLATE" ] || {
  echo "launchd plist template not found: $PLIST_TEMPLATE" >&2
  exit 1
}

CONFIG_DIR="$HOME/.config/wuji-ddns"
BIN_DIR="$HOME/.local/bin"
STATE_DIR="$HOME/.local/state/wuji-ddns"
PLIST_DIR="$HOME/Library/LaunchAgents"
TOKEN_FILE="$CONFIG_DIR/token"
INSTALL_SCRIPT_PATH="$BIN_DIR/wuji_ddns_update.sh"
PLIST_PATH="$PLIST_DIR/com.agentlogic.wuji-ddns.plist"

mkdir -p "$CONFIG_DIR" "$BIN_DIR" "$STATE_DIR" "$PLIST_DIR"

FUNCTION_URL=$(resolve_function_url)
write_token_file
chmod 600 "$TOKEN_FILE"

install -m 755 "$UPDATER_SOURCE" "$INSTALL_SCRIPT_PATH"

env \
  PLIST_TEMPLATE="$PLIST_TEMPLATE" \
  PLIST_PATH="$PLIST_PATH" \
  INSTALL_SCRIPT_PATH="$INSTALL_SCRIPT_PATH" \
  FUNCTION_URL="$FUNCTION_URL" \
  TOKEN_FILE="$TOKEN_FILE" \
  STATE_DIR="$STATE_DIR" \
  python3 <<'PY'
import os

template_path = os.environ["PLIST_TEMPLATE"]
output_path = os.environ["PLIST_PATH"]
replacements = {
    "__WUJI_DDNS_SCRIPT_PATH__": os.environ["INSTALL_SCRIPT_PATH"],
    "__WUJI_DDNS_FUNCTION_URL__": os.environ["FUNCTION_URL"],
    "__WUJI_DDNS_TOKEN_FILE__": os.environ["TOKEN_FILE"],
    "__WUJI_DDNS_LOG_DIR__": os.environ["STATE_DIR"],
}

with open(template_path, "r", encoding="utf-8") as handle:
    rendered = handle.read()

for needle, replacement in replacements.items():
    rendered = rendered.replace(needle, replacement)

with open(output_path, "w", encoding="utf-8") as handle:
    handle.write(rendered)
PY

if [ "$LOAD_AGENT" -eq 1 ]; then
  launchctl bootout "gui/$(id -u)" "$PLIST_PATH" 2>/dev/null || true
  launchctl bootstrap "gui/$(id -u)" "$PLIST_PATH"
  launchctl kickstart -k "gui/$(id -u)/com.agentlogic.wuji-ddns"
fi

DDNS_FUNCTION_URL="$FUNCTION_URL" \
DDNS_HOSTNAME="$HOSTNAME_VALUE" \
DDNS_TOKEN_FILE="$TOKEN_FILE" \
"$INSTALL_SCRIPT_PATH"

cat <<EOF
Installed Wuji DDNS updater.
  script: $INSTALL_SCRIPT_PATH
  token: $TOKEN_FILE
  plist: $PLIST_PATH
  log dir: $STATE_DIR
  hostname: $HOSTNAME_VALUE
  function url: $FUNCTION_URL
EOF

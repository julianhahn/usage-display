#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ENV_FILE="$ROOT_DIR/secrets/build.env"
TARGET="xtensa-esp32s3-none-elf"

if [[ ! -f "$ENV_FILE" ]]; then
  echo "Missing $ENV_FILE. Copy secrets/build.env.example and fill in local values." >&2
  exit 1
fi

set -a
# shellcheck disable=SC1091
source "$ENV_FILE"
set +a

if [[ -z "${WIFI_SSID:-}" || -z "${WIFI_PASSWORD:-}" ]]; then
  echo "WIFI_SSID and WIFI_PASSWORD must both be set in $ENV_FILE." >&2
  exit 1
fi

# ChatGPT credentials are preferably loaded directly from the local Codex auth
# file. They are held only in this process and are never printed or written.
if [[ -z "${CHATGPT_ACCESS_TOKEN:-}" || -z "${CHATGPT_ACCOUNT_ID:-}" ]]; then
  CODEX_AUTH_FILE="${CODEX_HOME:-$HOME/.codex}/auth.json"
  if [[ ! -f "$CODEX_AUTH_FILE" ]]; then
    echo "Missing ChatGPT credentials and Codex auth file: $CODEX_AUTH_FILE" >&2
    exit 1
  fi

  mapfile -t CHATGPT_CREDENTIALS < <(python3 - "$CODEX_AUTH_FILE" <<'PY'
import json
import sys

with open(sys.argv[1], encoding="utf-8") as auth_file:
    data = json.load(auth_file)
tokens = data.get("tokens", data)
access_token = tokens.get("access_token")
account_id = tokens.get("account_id") or data.get("account_id")
if not access_token or not account_id:
    raise SystemExit("Codex auth file has no access_token and account_id")
print(access_token)
print(account_id)
PY
  )
  CHATGPT_ACCESS_TOKEN="${CHATGPT_CREDENTIALS[0]}"
  CHATGPT_ACCOUNT_ID="${CHATGPT_CREDENTIALS[1]}"
  export CHATGPT_ACCESS_TOKEN CHATGPT_ACCOUNT_ID
fi

if [[ -z "${CHATGPT_ACCESS_TOKEN:-}" || -z "${CHATGPT_ACCOUNT_ID:-}" ]]; then
  echo "CHATGPT_ACCESS_TOKEN and CHATGPT_ACCOUNT_ID must be available for the firmware build." >&2
  exit 1
fi

# Load the host Cargo installation before selecting the ESP32 toolchain.
if [[ -f "$HOME/.cargo/env" ]]; then
  # shellcheck disable=SC1091
  source "$HOME/.cargo/env"
fi

# espup installs the ESP32 toolchain environment separately from the host Rust toolchain.
if [[ -f "$HOME/export-esp.sh" ]]; then
  # shellcheck disable=SC1091
  source "$HOME/export-esp.sh"
fi

cd "$ROOT_DIR"
exec cargo +esp build --target "$TARGET"

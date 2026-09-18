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

if [[ -z "${CHATGPT_ACCESS_TOKEN:-}" || -z "${CHATGPT_ACCOUNT_ID:-}" ]]; then
  echo "CHATGPT_ACCESS_TOKEN and CHATGPT_ACCOUNT_ID must both be set in $ENV_FILE." >&2
  exit 1
fi

cd "$ROOT_DIR"
exec cargo build --target "$TARGET"

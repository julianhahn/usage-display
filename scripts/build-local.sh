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

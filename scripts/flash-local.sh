#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ESPFLASH_BIN="${ESPFLASH_BIN:-$HOME/.cargo/bin/espflash}"
PORT="${ESPFLASH_PORT:-/dev/ttyUSB0}"
ELF="$ROOT_DIR/target/xtensa-esp32s3-none-elf/debug/usage-display"
BOOTLOADER="$ROOT_DIR/tools/bootloaders/esp32s3-bootloader.bin"

if [[ ! -x "$ESPFLASH_BIN" ]]; then
  echo "Missing espflash at $ESPFLASH_BIN. Install espflash 4.6 or newer." >&2
  exit 1
fi

version="$($ESPFLASH_BIN --version | awk '{print $2}')"
required="$(cat "$ROOT_DIR/tools/espflash-version.txt")"
if [[ "$version" != "$required" ]]; then
  echo "espflash $version is not the project version; install espflash $required or set ESPFLASH_BIN." >&2
  exit 1
fi

"$ROOT_DIR/scripts/build-local.sh"

if [[ ! -f "$ELF" ]]; then
  echo "Missing firmware ELF: $ELF" >&2
  exit 1
fi
if [[ ! -f "$BOOTLOADER" ]]; then
  echo "Missing project bootloader: $BOOTLOADER" >&2
  exit 1
fi

command_string="$(printf '%q ' "$ESPFLASH_BIN" flash --chip esp32s3 --port "$PORT" --non-interactive --bootloader "$BOOTLOADER" "$ELF")"
echo "Flashing $ELF to $PORT with espflash $version"
exec sg dialout -c "$command_string"

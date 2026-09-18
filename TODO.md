# Usage Display TODO

## Current goal

Read the ChatGPT subscription weekly usage from the Heltec HTIT-WB32LAF WiFi LoRa 32 V3.2 and show the remaining percentage on the OLED.

## Confirmed facts

- The board is a Heltec WiFi LoRa 32 V3.2.
- The radio variant is HTIT-WB32LAF.
- Linux sees the board as `/dev/ttyUSB0` through a CP2102 bridge.
- The ChatGPT usage endpoint works from Linux.
- The current Business account returns one weekly window.
- The local probe is `scripts/check_usage.py`.
- The first implementation will try direct board access.
- A Linux Helper is the fallback, not the first implementation.
- Build-time environment variables are acceptable for this private home test.

## Architecture and learning

- [x] Define the System Context in LikeC4.
- [x] Define the first container model.
- [x] Decide that the API boundary uses an adapter pattern.
- [x] Define `UsageSnapshot` as the project-owned usage model.
- [x] Document the working ChatGPT usage request.
- [x] Add the direct board request flow to the LikeC4 model.
- [x] Decide the exact `UsageSnapshot` fields (signed off: remaining percentage, reset time, and fresh/stale/unavailable status).
- [ ] Confirm the board display controller and I2C pins from the V3.2 schematic.
- [x] Choose the Rust ESP32-S3 framework and packages (`esp-hal` stack signed off).
- [x] Sign off the proposed program design before implementation. Signed-off sources: `docs/architecture/program-design.md` and `docs/architecture/source/program-design.likec4`; view: `docs/architecture/view/program-design.html`.

## Direct board proof

- [x] Create the Rust project skeleton.
- [x] Configure the ESP32-S3 build target (`xtensa-esp32s3-none-elf`, `build-std = ["core"]`).
- [ ] Build and flash a minimal firmware image.
  - Blocker: the skeleton still uses the default `std` binary entry point. The ESP32-S3 target is `no_std` and needs a minimal `no_std`/panic-handler firmware entry before it can build.
- [ ] Connect the board to Wi-Fi.
- [ ] Read `CHATGPT_ACCESS_TOKEN` and `CHATGPT_ACCOUNT_ID` at build time.
- [ ] Send an HTTPS GET request to `/backend-api/wham/usage`.
- [ ] Parse the weekly usage window.
- [ ] Convert `used_percent` to `remaining_percent`.
- [ ] Print the result through the serial connection.
- [ ] Show the result on the OLED.
- [ ] Refresh every five minutes.
- [ ] Keep the last good value when a request fails.
- [ ] Show a clear error state when no value exists.

## Credential rule for the first test

Credentials may be supplied through local build environment variables. They must never be committed or printed.

```text
CHATGPT_ACCESS_TOKEN=... CHATGPT_ACCOUNT_ID=... cargo build
```

The resulting firmware contains the test token. This is acceptable for the private home prototype. The token must be revoked after a test if needed.

## Fallback

Only if direct board access fails:

- [ ] Build a Linux Usage Helper.
- [ ] Let the Helper use Codex-managed authentication.
- [ ] Expose a small local JSON endpoint for the board.
- [ ] Change the board adapter from ChatGPT to the local Helper.

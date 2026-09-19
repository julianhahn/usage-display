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
- [x] Confirm the V3.2 OLED wiring and compatible driver. SDA 17, SCL 18, reset 21, active-low display power 36; Heltec's driver uses SSD1306 commands at `0x3c`, 128×64. The schematic does not name the controller silicon.
- [x] Choose the Rust ESP32-S3 framework and packages (`esp-hal` stack signed off).
- [x] Sign off the proposed program design before implementation. Signed-off sources: `docs/architecture/program-design.md` and `docs/architecture/source/program-design.likec4`; view: `docs/architecture/view/program-design.html`.

## Direct board proof

- [x] Create the Rust project skeleton.
- [x] Configure the ESP32-S3 build target (`xtensa-esp32s3-none-elf`, `build-std = ["core"]`).
- [x] Build and flash a minimal firmware image.
  - [x] Build the minimal `no_std` firmware for the ESP32-S3 target.
  - [x] Flash it to the board.
- [x] Connect the board to Wi-Fi.
- [x] Run Embassy Net with DHCPv4 and print the acquired IPv4 configuration over serial.
- [x] Prove direct HTTPS access from the ESP32 to ChatGPT. The board received HTTP 200 and parsed a 604800-second weekly window.
- [x] Read `CHATGPT_ACCESS_TOKEN` and `CHATGPT_ACCOUNT_ID` at build time. The local build script sources them from `~/.codex/auth.json` when they are not in `secrets/build.env`.
- [x] Send an HTTPS GET request to `/backend-api/wham/usage`.
- [x] Parse the weekly usage window.
- [x] Enable TLS certificate verification. The board fetched NTP time, checked the certificate chain, hostname, and dates, and received HTTP 200. An unrelated root was rejected with X.509 error `0x2700`. The working firmware was restored and succeeded again.
- [x] Fetch NTP time at boot before TLS. Skip HTTPS if the ten-second time lookup fails. Ordinary NTP is unauthenticated, as approved for this prototype.
- [ ] Finish the `UsageSnapshot` boundary. A tested percentage conversion now drives the OLED, but the complete snapshot model is not implemented.
- [x] Print the result through the serial connection.
- [x] Show the result on the OLED. The firmware wrote `93% remaining` from 7% weekly usage (`.logs/oled-serial.log`), and Julian confirmed the screen works.
- [ ] Refresh every five minutes.
- [ ] Keep the last good value when a request fails.
- [ ] Show a clear error state when no value exists.

## Credential rule for the first test

Credentials may be supplied through local build environment variables. They must never be committed or printed.

```text
CHATGPT_ACCESS_TOKEN=... CHATGPT_ACCOUNT_ID=... cargo build
```

The resulting firmware contains the test token. This is acceptable for the private home prototype. The token must be revoked after a test if needed.

## TLS and snapshot checkpoint

The current source and flashed firmware use verified TLS through `mbedtls-rs`, after fetching time from `time.cloudflare.com`. The latest hardware test received HTTP 200 and a 604800-second weekly window with 7% used, then successfully wrote 93% remaining to the OLED (`.logs/oled-serial.log`).

Reqwless uses `default-features = false` and the `mbedtls-rs` feature. Platform-independent TLS preserves the existing ESP stack. GTS Root R4 is stored in `certs/gts-root-r4.der`. NTP time is not authenticated; this is an accepted limitation of the private home prototype.

The approved `UsageSnapshot` boundary remains:

```text
remaining_percent
reset_at
status: fresh | stale | unavailable
```

No token or full response may be printed.

## Latest hardware proof

The minimal firmware flashed successfully to `/dev/ttyUSB0` with the project-local `espflash 4.4.0` path and the checked-in ESP-IDF 5.5.1 bootloader. The probe identified an ESP32-S3 revision v0.2 with 8 MB flash. The Wi-Fi firmware acquired IPv4 `192.168.178.70/24` by DHCP.

## Current checkpoint

- Verified HTTPS is working on the board. The TLS and NTP changes are not committed yet.
- The board rejected an unrelated ISRG Root X1 trust anchor with error `0x2700` and no HTTP success (`.logs/tls-wrong-root-serial.log`). That temporary source change was removed. The final firmware again succeeded with GTS Root R4 (`.logs/ntp-final-serial.log`).
- Six host tests cover NTP packet validation, unrelated replies, invalid server status, date bounds, and the 2036 timestamp wrap. Run: `rustc +stable --edition 2024 --test src/parse_ntp_response.rs -o /tmp/usage-display-ntp-tests && /tmp/usage-display-ntp-tests`.
- Time sync runs once per boot. The TLS clock then advances with elapsed board time. NTP has a ten-second timeout; HTTPS has a thirty-second timeout. Time lookup failure skips HTTPS. This does not yet implement repeated refreshes or clock resynchronization.
- CMake 4.4.3 and Espressif Clang `esp-20.1.1_20250829` live under `~/.local/opt/usage-display-build-tools/`; `scripts/build-local.sh` loads their optional `env.sh`. Global shell settings and the ESP Rust/Wi-Fi stack are unchanged.
- `mbedtls-rs` uses `embassy-time` and `hook-wall-clock`, not its `esp32s3` feature, which requires an incompatible older `esp-hal`.
- LikeC4 sources, architecture specifications, and rendered views include NTP.
- Julian chose OLED output before a separate snapshot task. `OledPresenter` now handles power, reset, initialization, progress text, percentage output, and no-data text. Missing/nonweekly windows and usage above 100 are not displayed as a valid percentage.
- Three additional host tests cover remaining-percentage conversion: `rustc +stable --edition 2024 --test src/remaining_percent.rs -o /tmp/usage-display-percentage-tests && /tmp/usage-display-percentage-tests`.
- The latest firmware is flashed, the display acknowledged all writes, and Julian confirmed the visible result works. A complete `UsageSnapshot`, five-minute refresh, and stale-value handling remain open.

## Physical enclosure and power

Plan: build a small wooden desktop enclosure for the Heltec board.

- [x] Extract the main board measurements from the reference photos: about 75 mm long, 30 mm wide, and about 20 mm high including the pin headers. Re-measure the bare board before cutting wood.
- [ ] Confirm the rear clip connector location and leave clearance for it. The connector is not visible in these front and side photos.
- [ ] Leave openings for the USB-C port on the short left edge, the built-in PRG and RST buttons on the front, and the OLED.
- [ ] Add a rechargeable battery and decide where its connector and charging path sit.
- [ ] Add one accessible power button to turn the device on and off.
- [ ] Reserve a second accessible button for later firmware control.
- [ ] Make the enclosure slope upward toward the user. The display will sit on a table about one arm's length away, viewed from slightly above at chest height.
- [ ] Start with an internal envelope of at least 80 × 35 × 25 mm. Add extra space for the battery, switch, second button, wiring, wood thickness, and the slanted mounting angle.
- [ ] Prototype the wooden case with removable panels so the board and battery remain serviceable.
- [ ] Check ventilation, cable access, button reach, and battery safety before the final build.

The case should be a small angled box, not a flat board cover. The display faces the user; the USB-C port and board buttons remain reachable for setup and recovery.

## Fallback

Only if direct board access fails:

- [ ] Build a Linux Usage Helper.
- [ ] Let the Helper use Codex-managed authentication.
- [ ] Expose a small local JSON endpoint for the board.
- [ ] Change the board adapter from ChatGPT to the local Helper.

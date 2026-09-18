# Rust stack proposal for the direct board proof

**Status: PROPOSED — human sign-off required**

This proposal covers the first direct-board proof for the Heltec WiFi LoRa 32 V3.2 (`HTIT-WB32LAF`). It does not create the Rust project or choose dependency versions yet.

## Recommendation

Use the bare-metal `esp-hal` ecosystem rather than ESP-IDF:

- `esp-hal` for ESP32-S3 peripherals and clocks
- the current Espressif Wi-Fi crate for the selected `esp-hal` release (`esp-wifi` or its successor, if renamed)
- `embedded-tls` for TLS
- `reqwless` for a small embedded HTTP client
- `serde` and `serde-json-core` for the small usage response
- `ssd1306` for the OLED controller
- `embedded-graphics` for text and simple rendering

The exact compatible versions must be pinned together when implementation starts. The package names above describe responsibilities, not a promise to use the latest release blindly.

## Why `esp-hal`

`esp-hal` is the direct Rust hardware abstraction layer for the ESP32-S3. It exposes GPIO, I2C, timers, clocks, and the interfaces needed by the application without requiring a Linux-like operating system.

This fits the learning goal:

```text
application
  → esp-hal and embedded drivers
  → ESP32-S3 hardware
```

It also keeps the hardware boundary visible. The Heltec board is not a separate processor. We configure its board-specific connections ourselves:

- OLED SDA: GPIO17
- OLED SCL: GPIO18
- OLED reset: GPIO21
- OLED address: `0x3C`

## Why not ESP-IDF first

`esp-idf-hal` would provide access to Espressif's ESP-IDF framework and its mature networking stack. That is a valid alternative, especially when an application needs large existing ESP-IDF components.

It is not the first choice here because the first proof needs a small, understandable firmware path. `esp-hal` lets us learn the microcontroller, Wi-Fi, TLS, and display boundaries directly. We can revisit ESP-IDF if TLS or Wi-Fi support blocks the direct proof.

## Network and HTTPS

`esp-hal` alone does not provide Wi-Fi or HTTPS.

The layers are:

```text
Usage API Adapter
  → HTTP client (`reqwless`)
  → TLS (`embedded-tls`)
  → TCP/IP and Wi-Fi (`esp-wifi` or current successor)
  → ESP32-S3 radio
```

The proof sends:

```text
GET https://chatgpt.com/backend-api/wham/usage
```

with credentials supplied through local build-time environment variables. The token is intentionally not part of the repository. It will still be embedded in the private test firmware, which is acceptable for this home prototype.

The adapter will parse only the fields needed by the signed-off model:

- `rate_limit.primary_window` or `secondary_window`
- `used_percent`
- `limit_window_seconds`
- `reset_at`

The Business account currently returns one 7-day window. The parser must identify the weekly window from `limit_window_seconds`, not from the field name alone.

## Display

Use the generic `ssd1306` driver with `embedded-graphics`:

```text
OLED Presenter
  → ssd1306
  → embedded-hal I2C traits
  → esp-hal I2C GPIO17/GPIO18
  → OLED at 0x3C
```

`ssd1306` knows the controller protocol. It does not know the Heltec pin wiring. The board configuration supplies the pins and address.

The reset pin GPIO21 is kept in the board setup because the Heltec wiring exposes it. Whether the driver needs a reset action must be confirmed during the minimal display test.

## Runtime shape

The first firmware should have these responsibilities:

```text
startup
  → initialise clocks and peripherals
  → initialise Wi-Fi
  → initialise OLED
  → run refresh loop

refresh loop
  → connect or reuse Wi-Fi
  → request usage
  → adapt response to UsageSnapshot
  → render fresh, stale, or unavailable state
  → wait five minutes
```

No board-specific application logic should be placed inside the display driver or the HTTP transport.

## Sign-off questions

1. Do you approve `esp-hal` as the first framework?
2. Do you approve the embedded stack of Wi-Fi crate + `embedded-tls` + `reqwless` + `serde-json-core`?
3. Do you approve `ssd1306` + `embedded-graphics` for the OLED?
4. Should the implementation pin the current compatible crate versions after this proposal is signed off?

Until these answers are approved, this remains a proposal and implementation must not start.

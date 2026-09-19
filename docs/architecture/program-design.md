# Program design proposal

**Status: SIGNED OFF — approved by the human owner for implementation**

Decision: implement the direct-board-first design. The Linux Helper remains a fallback only if the board cannot complete the request reliably.

## Goal

The firmware should connect the Heltec HTIT-WB32LAF WiFi LoRa 32 V3.2 to Wi-Fi, request the ChatGPT subscription usage value directly, and show the remaining weekly percentage on the onboard OLED.

The first design is direct-board-first. A Linux Helper is only a fallback if the board cannot complete the request reliably.

## Responsibilities

```text
Application coordinator
├── Startup and configuration
├── Wi-Fi manager
├── TLS clock
├── Usage API adapter
├── Usage state
├── OLED presenter
└── Refresh scheduler
```

### Application coordinator

Owns the top-level lifecycle. It initializes hardware, runs the refresh cycle, updates state, and asks the presenter to render the current state.

### Startup and configuration

Initializes the ESP32-S3 peripherals and the board pins. Build-time environment values provide the Wi-Fi credentials, ChatGPT access token, and ChatGPT account ID for the private lab test. These values are never logged.

### Wi-Fi manager

Connects to the configured network and reports whether the connection is ready. It owns reconnect attempts. Other parts do not manipulate Wi-Fi details directly.

### TLS clock

After Wi-Fi connects, fetch time from `time.cloudflare.com` using NTP on UDP port 123. Allow ten seconds for DNS and the reply. Check the sender, request nonce, server status, and supported date range. A failed lookup skips HTTPS.

Register the successful time with `mbedtls-rs`. Add elapsed board time when checking certificate dates. The current one-request proof synchronizes once per boot.

Ordinary NTP is not authenticated. The human owner approved this trade-off for the private home prototype. A network attacker could change the reported time; the nonce only rejects replies unrelated to this request.

### Usage API adapter

Builds the HTTPS request to:

```text
GET https://chatgpt.com/backend-api/wham/usage
```

It adds the bearer token and `ChatGPT-Account-Id` headers. It parses the response shape and converts the weekly window's `used_percent` into the project model's `remaining_percent`.

No OpenAI-specific JSON types leave this boundary.

TLS requires the GTS Root R4 trust anchor, the `chatgpt.com` hostname, and valid certificate dates. There is no fallback that disables verification. The request timeout is thirty seconds.

### Usage state

Stores the latest successful `UsageSnapshot` and the current status:

```text
UsageSnapshot {
    remaining_percent
    reset_at
    status: fresh | stale | unavailable
}
```

A successful response replaces the previous snapshot. A failed refresh keeps the last value and changes its status to `stale`. If no successful value exists, the status is `unavailable`.

### OLED presenter

Renders only the project-owned state. It knows the SSD1306-compatible display and its board connection:

- I2C address: `0x3C`
- SDA: GPIO17
- SCL: GPIO18
- reset: GPIO21
- display power: GPIO36, low = on
- resolution: 128×64

The [V3.2 schematic](https://resource.heltec.cn/download/WiFi_LoRa_32_V3/WiFi_LoRa_32_V3.2_Schematic_Diagram.pdf) and [V3.2 pin map](https://resource.heltec.cn/download/WiFi_LoRa_32_V3/Wi-Fi_LoRa32_V3.2_Pinmap.png) identify the wiring. The schematic labels the panel only as `0.96_OLED`; [Heltec's driver](https://github.com/HelTecAutomation/Heltec_ESP32/blob/6ab5cf916e01b0930eaa2245323c728f849705c4/src/heltec.cpp) uses SSD1306 commands, address `0x3c`, and 128×64 geometry.

The first screen shows `Connecting...`, then `Checking time...` and `Reading usage...`. A valid weekly result shows the remaining percentage and `Reset in 2d 14h`. The reset countdown uses network time plus elapsed device time and redraws every minute. Partial hours round up. A missing reset timestamp shows `Reset unknown`; an elapsed window shows `Reset due`. Usage is still fetched only at startup. Missing weekly data shows `No weekly data`; a failed time lookup or request shows `No data`. Refresh and stale-value handling remain separate work.

The presenter takes a percentage or a status string. It does not know the ChatGPT response format.

### Refresh scheduler

Starts one refresh at boot and schedules another refresh every five minutes. It must not start overlapping requests. A failed refresh does not stop the scheduler.

## Data flow

```text
Startup
  → Wi-Fi manager
  → NTP time lookup and TLS clock
  → Usage API adapter
  → HTTPS request to ChatGPT
  → JSON response
  → normalized UsageSnapshot
  → Usage state
  → OLED presenter
  → five-minute delay
  → next refresh
```

The adapter is the translation boundary:

```text
ChatGPT response
  → Usage API adapter
  → UsageSnapshot
  → OLED presenter
```

## State machine

```text
           boot
             ↓
        ConnectingWifi
         ↙          ↘
   WifiUnavailable   Ready
         │             │
         │             ↓
         │       FetchingUsage
         │        ↙          ↘
         │  RequestFailed   UsageReceived
         │       │              │
         └───────┘              ↓
                         ParsingUsage
                          ↙          ↘
                   InvalidResponse  ValidResponse
                          │              │
                          ↓              ↓
                    Unavailable       Fresh
                          │              │
                          └──────┬───────┘
                                 ↓
                              Display
                                 ↓
                            Wait five minutes
                                 ↓
                         ConnectingWifi / Ready
```

`Ready` requires a successful NTP time lookup at boot. Without it, the usage request is skipped.

If a previous successful snapshot exists, `WifiUnavailable`, `RequestFailed`, and `InvalidResponse` render that value as `stale`. Without a previous snapshot, they render `unavailable`.

## Error handling

| Failure | State behavior | Display behavior | Next action |
| --- | --- | --- | --- |
| Wi-Fi cannot connect | Keep previous snapshot, or `unavailable` | Last value with stale marker, or `No data` | Retry on the next cycle |
| NTP lookup fails | No usage request; unavailable at boot | `No data` | Retry time lookup before a later request |
| HTTPS connection fails | Keep previous snapshot as `stale` | Last value with error marker | Retry on the next cycle |
| HTTP non-success status | Keep previous snapshot as `stale` | Last value with error marker | Retry on the next cycle |
| JSON is malformed | Keep previous snapshot as `stale` | Last value with error marker | Retry on the next cycle |
| Weekly window is missing | Set `unavailable` unless an old value exists | `No weekly data` or stale value | Retry on the next cycle |
| Token is missing at build time | Refuse normal usage request | `Setup needed` | Requires a new local build |

Errors should be represented by small internal status values. They should not expose tokens or full response bodies through serial output or the display.

## Hardware and software boundaries

```text
Application coordinator
  ↓
Usage API adapter / OLED presenter
  ↓
esp-hal peripherals and compatible drivers
  ↓
ESP32-S3 / I2C OLED / Wi-Fi radio
```

The approved Rust stack is:

- `esp-hal` for ESP32-S3 peripherals
- a compatible ESP32-S3 Wi-Fi crate
- `mbedtls-rs` in platform-independent mode for verified HTTPS
- `reqwless` for embedded HTTP
- `serde-json-core` for bounded JSON parsing
- `ssd1306` for the display controller
- `embedded-graphics` for rendering

Exact crate versions and target configuration are implementation details and must not change the responsibilities above.

## Credentials

For the private lab test, build-time environment variables are accepted:

```text
CHATGPT_ACCESS_TOKEN=... CHATGPT_ACCOUNT_ID=... cargo build
```

The source code and Git history must not contain credentials. The resulting firmware does contain the test token. This is accepted for the trusted home prototype. Token renewal and stronger device storage are not part of the first implementation slice.

## Testing seams

- Test the NTP reply parser on the host, including invalid replies and the 2036 timestamp wrap.
- On the board, verify that the correct trust root succeeds and an unrelated root fails.
- Test the response parser with recorded, redacted JSON fixtures.
- Test conversion from `used_percent` to `remaining_percent` without hardware.
- Test missing weekly-window behavior.
- Test fresh-to-stale state transitions without network access.
- Test the OLED presenter with a fake display interface.
- Keep Wi-Fi and HTTPS behind interfaces so the coordinator can be tested with fakes.
- Use the real board only for pin, Wi-Fi, HTTPS, and display integration checks.

## First implementation slice

Implement only the smallest vertical proof:

1. Initialize the ESP32-S3 and serial output.
2. Connect to Wi-Fi.
3. Send one HTTPS request using build-time credentials.
4. Parse the weekly window.
5. Print only the normalized `remaining_percent` and `reset_at` over serial.

Do not implement the five-minute scheduler, stale rendering, or OLED presentation in this slice. The purpose is to prove that the board can reach the endpoint and parse the real response before adding display behavior.

## Sign-off question

Can you sign off this program design for the first implementation slice, with direct board access as the primary path and the Linux Helper kept as a fallback?

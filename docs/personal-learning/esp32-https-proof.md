# ESP32 HTTPS proof

## Result

The Heltec board made a direct HTTPS request to ChatGPT successfully.

```text
GET https://chatgpt.com/backend-api/wham/usage
HTTP status: 200
Body bytes: 1358
Weekly window: 604800 seconds
Weekly used percent: 1
```

The board also reported the weekly reset timestamp without printing the token or the response body.

## Rust pieces used

- `esp-radio` for Wi-Fi.
- `embassy-net` for DHCPv4, DNS, and TCP.
- `reqwless` from the pinned upstream revision for HTTP.
- `embedded-tls` through `reqwless` for TLS 1.3.
- `serde-json-core` for allocation-free response parsing.

The upstream revision is used because the crates.io `reqwless 0.14.0` release selected an incompatible `embedded-tls 0.18` and DER combination. The pinned revision uses `embedded-tls 0.19` and builds with the current ESP Rust stack.

## Credential flow

`scripts/build-local.sh` first reads `secrets/build.env`. If the ChatGPT values are absent, it reads `~/.codex/auth.json` in a short-lived Python process and exports the values only to the current build process.

The token is embedded into this private prototype firmware image. It is never printed or committed. This is acceptable for the trusted home prototype. It is not the final production credential design.

## Current TLS limitation

The proof uses `reqwless::client::TlsVerify::None`. Traffic is encrypted, but the server certificate is not verified in this first proof.

A first certificate-verification attempt used `embedded-tls` with a pinned CA certificate. The firmware built and flashed, but the board stalled during the TLS handshake before producing a response. That attempt was not accepted as verification.

The next attempt should switch reqwless to its `mbedtls-rs` backend. This is the Espressif-oriented path and supports hardware-oriented TLS handling. It must be built, flashed, and confirmed by serial output before this document can claim verified TLS.

## UsageSnapshot checkpoint

The approved boundary is:

```text
UsageSnapshot {
  remaining_percent
  reset_at
  status: fresh | stale | unavailable
}
```

The existing board proof only exposes the raw weekly `used_percent`. Converting it to `remaining_percent` and preserving stale values is still pending on-device verification. No token or full API response may be printed.

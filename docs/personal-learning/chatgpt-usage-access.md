# ChatGPT subscription usage access

## Confirmed result

The ChatGPT subscription usage value can be read from Linux without opening the ChatGPT website.

The working request is:

```text
GET https://chatgpt.com/backend-api/wham/usage
```

It uses the ChatGPT OAuth credentials created by Codex:

```text
~/.codex/auth.json
```

The request needs:

```text
Authorization: Bearer <access_token>
ChatGPT-Account-Id: <account_id>
```

The token and account ID must never be committed, printed, or stored in the repository.

## Current account result

The connected account reported:

- plan: `self_serve_business_prolite`
- access allowed: yes
- limit reached: no
- one weekly window: 604800 seconds
- current remaining value during the test: 99%

The endpoint returns `used_percent`. The application calculates:

```text
remaining_percent = 100 - used_percent
```

## What this means for the project

The data source works. We do not need browser automation for the first implementation.

The first direct-board experiment can use the same HTTP request from the ESP32. The main open issue is credential provisioning and renewal. A short-lived token may be used only for a controlled lab test. It is not a final security design.

The endpoint is an internal ChatGPT endpoint, not a public OpenAI API. It may change without notice.

## Reproducible local test

Run:

```text
python3 scripts/check_usage.py
```

The script reads local Codex credentials and prints only normalized, non-secret usage data.

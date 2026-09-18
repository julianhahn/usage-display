#!/usr/bin/env python3
"""Read ChatGPT subscription usage through the local Codex OAuth session."""

import json
import os
import sys
import urllib.error
import urllib.request
from pathlib import Path

USAGE_URL = "https://chatgpt.com/backend-api/wham/usage"
AUTH_PATH = Path(os.environ.get("CODEX_HOME", Path.home() / ".codex")) / "auth.json"


def load_credentials():
    data = json.loads(AUTH_PATH.read_text())
    tokens = data.get("tokens", data)
    access_token = tokens.get("access_token")
    account_id = tokens.get("account_id") or data.get("account_id")
    if not access_token or not account_id:
        raise RuntimeError("auth.json has no access_token and account_id")
    return access_token, account_id


def main():
    access_token, account_id = load_credentials()
    request = urllib.request.Request(
        USAGE_URL,
        headers={
            "Accept": "application/json",
            "Authorization": f"Bearer {access_token}",
            "ChatGPT-Account-Id": account_id,
            "User-Agent": "usage-display-local-test/0.1",
        },
    )

    try:
        with urllib.request.urlopen(request, timeout=20) as response:
            payload = json.load(response)
    except urllib.error.HTTPError as error:
        raise RuntimeError(f"usage request failed with HTTP {error.code}") from error
    except urllib.error.URLError as error:
        raise RuntimeError(f"usage request failed: {error.reason}") from error

    print(json.dumps(normalize(payload), indent=2))


def normalize(payload):
    rate_limit = payload.get("rate_limit") or {}
    windows = []
    for name in ("primary_window", "secondary_window"):
        window = rate_limit.get(name)
        if not isinstance(window, dict):
            continue
        used = window.get("used_percent")
        windows.append(
            {
                "name": name,
                "used_percent": used,
                "remaining_percent": None if used is None else 100 - used,
                "window_seconds": window.get("limit_window_seconds"),
                "reset_at": window.get("reset_at"),
            }
        )

    return {
        "plan_type": payload.get("plan_type"),
        "allowed": rate_limit.get("allowed"),
        "limit_reached": rate_limit.get("limit_reached"),
        "windows": windows,
    }


if __name__ == "__main__":
    try:
        main()
    except (OSError, json.JSONDecodeError, RuntimeError) as error:
        print(f"Error: {error}", file=sys.stderr)
        sys.exit(1)

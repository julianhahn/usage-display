# UsageSnapshot proposal

**Status: PROPOSED — human sign-off required**

`UsageSnapshot` is the project-owned model produced by the Usage API Adapter. It keeps ChatGPT response details out of the refresh loop and OLED code.

## Fields

| Field | Meaning |
| --- | --- |
| `remaining_percent` | Weekly usage remaining, from `0` to `100`. The adapter calculates this as `100 - used_percent`. |
| `reset_at` | Unix timestamp for the end of the weekly usage window. |
| `status` | Whether the value is usable: `fresh`, `stale`, or `unavailable`. |
| `error` | Optional short, non-secret error category when the latest refresh failed. |

The model deliberately does not include the raw ChatGPT response, token, account ID, plan details, or absolute usage counts.

## States

### `fresh`

A successful request returned a valid weekly window. `remaining_percent` and `reset_at` are current.

### `stale`

A previous valid snapshot exists, but the latest refresh failed or returned invalid data. Keep the previous `remaining_percent` and `reset_at`, and set `error` to a small category such as `wifi_unavailable`, `http_error`, or `invalid_response`.

### `unavailable`

No valid snapshot exists yet. The OLED cannot show a usage value and should show a clear no-data state. An error category may explain why.

## Example

```text
UsageSnapshot {
  remaining_percent: 99,
  reset_at: 1790341276,
  status: fresh,
  error: none,
}
```

## Sign-off question

Please confirm whether this minimal model is the architecture we should implement. In particular, confirm that the weekly percentage and reset time are enough for V1 and that `fresh` / `stale` / `unavailable` are the right states.

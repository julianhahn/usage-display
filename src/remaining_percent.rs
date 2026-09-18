/// Only a valid seven-day usage window can become a remaining percentage.
pub fn remaining_percent(used: Option<u32>, window_seconds: Option<u32>) -> Option<u8> {
    if window_seconds != Some(604_800) {
        return None;
    }
    u8::try_from(100_u32.checked_sub(used?)?).ok()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_weekly_usage() {
        for (used, remaining) in [(0, 100), (5, 95), (100, 0)] {
            assert_eq!(
                remaining_percent(Some(used), Some(604_800)),
                Some(remaining)
            );
        }
    }

    #[test]
    fn rejects_missing_or_invalid_usage() {
        assert_eq!(remaining_percent(None, Some(604_800)), None);
        assert_eq!(remaining_percent(Some(101), Some(604_800)), None);
        assert_eq!(remaining_percent(Some(u32::MAX), Some(604_800)), None);
    }

    #[test]
    fn rejects_missing_or_nonweekly_window() {
        assert_eq!(remaining_percent(Some(5), None), None);
        assert_eq!(remaining_percent(Some(5), Some(18_000)), None);
    }
}

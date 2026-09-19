/// Whole days and hours until reset. None means the reset time is unknown.
/// Round up partial hours so an upcoming reset never looks overdue.
pub fn reset_countdown(reset_at: Option<u64>, now: u64) -> Option<(u64, u64)> {
    let seconds = reset_at?.saturating_sub(now);
    let hours = seconds / 3600 + u64::from(seconds % 3600 != 0);
    Some((hours / 24, hours % 24))
}

#[cfg(test)]
mod tests {
    use super::reset_countdown;

    #[test]
    fn shows_days_and_hours() {
        assert_eq!(reset_countdown(Some(100 + 62 * 3600), 100), Some((2, 14)));
    }

    #[test]
    fn rounds_up_partial_hours() {
        assert_eq!(reset_countdown(Some(101), 100), Some((0, 1)));
        assert_eq!(reset_countdown(Some(86400), 1), Some((1, 0)));
    }

    #[test]
    fn handles_missing_and_elapsed_resets() {
        assert_eq!(reset_countdown(None, 100), None);
        assert_eq!(reset_countdown(Some(100), 100), Some((0, 0)));
        assert_eq!(reset_countdown(Some(99), 100), Some((0, 0)));
    }
}

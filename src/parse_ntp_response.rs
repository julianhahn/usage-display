/// Validate an SNTP server reply and return whole Unix seconds.
/// The nonce links the reply to our request; it does not authenticate the server.
pub fn parse_ntp_response(packet: &[u8], nonce: &[u8; 8]) -> Option<u64> {
    if packet.len() < 48 {
        return None;
    }
    let version = (packet[0] >> 3) & 7;
    if packet[0] >> 6 == 3
        || !(3..=4).contains(&version)
        || packet[0] & 7 != 4
        || !(1..=15).contains(&packet[1])
        || packet[24..32] != nonce[..]
        || packet[40..48] == [0; 8]
    {
        return None;
    }
    let seconds = u32::from_be_bytes(packet[40..44].try_into().ok()?) as u64;
    const UNIX_OFFSET: u64 = 2_208_988_800;
    // NTP's 32-bit seconds wrap in February 2036. Support the current and next era.
    let seconds = if seconds < UNIX_OFFSET {
        seconds + (1_u64 << 32)
    } else {
        seconds
    };
    let unix = seconds - UNIX_OFFSET;
    // Reject dates outside this prototype's supported range: 2020 through 2099.
    (1_577_836_800..4_102_444_800)
        .contains(&unix)
        .then_some(unix)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NONCE: [u8; 8] = [1, 2, 3, 4, 5, 6, 7, 8];

    fn reply(unix: u64) -> [u8; 48] {
        let mut packet = [0; 48];
        packet[0] = 0x24; // NTPv4 server response
        packet[1] = 2;
        packet[24..32].copy_from_slice(&NONCE);
        packet[40..44].copy_from_slice(&((unix + 2_208_988_800) as u32).to_be_bytes());
        packet
    }

    #[test]
    fn accepts_current_time() {
        assert_eq!(
            parse_ntp_response(&reply(1_789_776_000), &NONCE),
            Some(1_789_776_000)
        );
    }

    #[test]
    fn accepts_time_after_2036_wrap() {
        assert_eq!(
            parse_ntp_response(&reply(2_208_988_800), &NONCE),
            Some(2_208_988_800)
        );
    }

    #[test]
    fn rejects_short_or_unrelated_reply() {
        assert_eq!(parse_ntp_response(&[0; 47], &NONCE), None);
        assert_eq!(parse_ntp_response(&reply(1_789_776_000), &[9; 8]), None);
    }

    #[test]
    fn rejects_unsynced_server_wrong_mode_or_version() {
        for header in [0xe4, 0x23, 0x14] {
            let mut packet = reply(1_789_776_000);
            packet[0] = header;
            assert_eq!(parse_ntp_response(&packet, &NONCE), None);
        }
    }

    #[test]
    fn rejects_kiss_of_death_and_invalid_stratum() {
        for stratum in [0, 16, 255] {
            let mut packet = reply(1_789_776_000);
            packet[1] = stratum;
            assert_eq!(parse_ntp_response(&packet, &NONCE), None);
        }
    }

    #[test]
    fn rejects_missing_or_out_of_range_time() {
        let mut packet = reply(1_789_776_000);
        packet[40..48].fill(0);
        assert_eq!(parse_ntp_response(&packet, &NONCE), None);
        assert_eq!(parse_ntp_response(&reply(1_546_300_800), &NONCE), None);
        assert_eq!(parse_ntp_response(&reply(4_102_444_800), &NONCE), None);
    }
}

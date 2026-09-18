use embassy_net::{
    Stack,
    dns::DnsQueryType,
    udp::{PacketMetadata, UdpSocket},
};
use embassy_time::{Duration, Instant, with_timeout};
use mbedtls_rs::sys::{
    hook::wall_clock::{MbedtlsWallClock, hook_wall_clock},
    tm,
};
use static_cell::StaticCell;

use crate::parse_ntp_response::parse_ntp_response;

const NTP_SERVER: &str = "time.cloudflare.com";
static CLOCK: StaticCell<NtpClock> = StaticCell::new();

#[derive(Debug)]
pub enum ClockError {
    Timeout,
    Dns,
    Socket,
    Response,
    Entropy,
}

struct NtpClock {
    unix_seconds: u64,
    captured_at: Instant,
}

impl MbedtlsWallClock for NtpClock {
    fn instant(&self) -> Option<tm> {
        let seconds = self
            .unix_seconds
            .checked_add(self.captured_at.elapsed().as_secs())?;
        let date = time::OffsetDateTime::from_unix_timestamp(i64::try_from(seconds).ok()?).ok()?;
        Some(tm {
            tm_sec: i32::from(date.second()),
            tm_min: i32::from(date.minute()),
            tm_hour: i32::from(date.hour()),
            tm_mday: i32::from(date.day()),
            tm_mon: date.month() as i32 - 1,
            tm_year: date.year() - 1900,
            tm_wday: date.weekday().number_days_from_sunday() as i32,
            tm_yday: i32::from(date.ordinal()) - 1,
            tm_isdst: 0,
        })
    }
}

/// Set the TLS calendar clock once, after Wi-Fi connects and before any TLS call.
/// Ordinary NTP is unauthenticated: this prototype trusts the network's time reply.
pub async fn synchronize_clock(stack: Stack<'static>) -> Result<u64, ClockError> {
    with_timeout(Duration::from_secs(10), async {
        let addresses = stack
            .dns_query(NTP_SERVER, DnsQueryType::A)
            .await
            .map_err(|_| ClockError::Dns)?;
        let address = *addresses.first().ok_or(ClockError::Dns)?;
        let mut rx_meta = [PacketMetadata::EMPTY; 1];
        let mut tx_meta = [PacketMetadata::EMPTY; 1];
        let mut rx = [0; 512];
        let mut tx = [0; 48];
        let mut socket = UdpSocket::new(stack, &mut rx_meta, &mut rx, &mut tx_meta, &mut tx);
        socket.bind(0).map_err(|_| ClockError::Socket)?;
        let mut packet = [0; 48];
        packet[0] = 0x23; // NTPv4 client
        let mut nonce = [0; 8];
        esp_hal::rng::Trng::try_new()
            .map_err(|_| ClockError::Entropy)?
            .read(&mut nonce);
        packet[40..48].copy_from_slice(&nonce);
        socket
            .send_to(&packet, (address, 123))
            .await
            .map_err(|_| ClockError::Socket)?;
        let mut response = [0; 512];
        let (size, peer) = socket
            .recv_from(&mut response)
            .await
            .map_err(|_| ClockError::Socket)?;
        if peer.endpoint.addr != address || peer.endpoint.port != 123 {
            return Err(ClockError::Response);
        }
        let unix_seconds =
            parse_ntp_response(&response[..size], &nonce).ok_or(ClockError::Response)?;
        let clock = CLOCK.init(NtpClock {
            unix_seconds,
            captured_at: Instant::now(),
        });
        // SAFETY: registered before TLS starts; immutable storage lives for the entire program.
        unsafe {
            hook_wall_clock(Some(clock));
        }
        Ok(unix_seconds)
    })
    .await
    .map_err(|_| ClockError::Timeout)?
}

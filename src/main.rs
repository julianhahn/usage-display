#![no_std]
#![no_main]

#[path = "OledPresenter.rs"]
mod oled_presenter;
mod parse_ntp_response;
mod remaining_percent;
mod reset_countdown;
mod synchronize_clock;

use core::{fmt::Write as _, panic::PanicInfo};

use embassy_executor::Spawner;
use embassy_net::{
    Config as NetConfig, Runner, StackResources,
    dns::DnsSocket,
    tcp::client::{TcpClient, TcpClientState},
};
use embassy_time::{Duration, Timer};
use embedded_io_async::Read;
use esp_hal::{
    clock::CpuClock,
    gpio::{Level, Output, OutputConfig},
    i2c::master::{Config as I2cConfig, I2c},
    timer::timg::TimerGroup,
};
use esp_println::println;
use esp_radio::wifi::{
    AuthenticationMethodConfig, Config, ControllerConfig, Interface, WifiController,
    sta::StationConfig,
};
use heapless::String;
use reqwless::{
    client::{HttpClient, TlsConfig},
    headers::ContentType,
    request::{Method, RequestBuilder},
    response::Status,
};
use serde::Deserialize;
use static_cell::StaticCell;

esp_bootloader_esp_idf::esp_app_desc!();

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");
const CHATGPT_ACCESS_TOKEN: &str = env!("CHATGPT_ACCESS_TOKEN");
const CHATGPT_ACCOUNT_ID: &str = env!("CHATGPT_ACCOUNT_ID");
const USAGE_URL: &str = "https://chatgpt.com/backend-api/wham/usage";

static NET_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();
static TCP_STATE: StaticCell<TcpClientState<1, 16_384, 16_384>> = StaticCell::new();

#[derive(Deserialize)]
struct UsagePayload {
    rate_limit: Option<RateLimit>,
}

#[derive(Deserialize)]
struct RateLimit {
    primary_window: Option<UsageWindow>,
    secondary_window: Option<UsageWindow>,
}

#[derive(Deserialize)]
struct UsageWindow {
    used_percent: Option<u32>,
    limit_window_seconds: Option<u32>,
    reset_at: Option<u64>,
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[esp_hal::main]
async fn main(spawner: Spawner) -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 140 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);

    let i2c = I2c::new(peripherals.I2C0, I2cConfig::default())
        .unwrap()
        .with_sda(peripherals.GPIO17)
        .with_scl(peripherals.GPIO18);
    let power = Output::new(peripherals.GPIO36, Level::Low, OutputConfig::default());
    let reset = Output::new(peripherals.GPIO21, Level::High, OutputConfig::default());
    let mut oled = match oled_presenter::OledPresenter::new(i2c, power, reset).await {
        Ok(display) => {
            println!("usage-display: OLED initialized at 0x3c");
            Some(display)
        }
        Err(error) => {
            println!("usage-display: OLED initialization failed: {:?}", error);
            None
        }
    };

    let station_config = Config::Station(
        StationConfig::default()
            .with_ssid(WIFI_SSID.try_into().unwrap())
            .with_authentication(AuthenticationMethodConfig::Wpa2Personal(
                WIFI_PASSWORD.try_into().unwrap(),
            )),
    );

    println!("usage-display: starting Wi-Fi");
    let wifi_interface = Interface::station();
    let controller = WifiController::new(
        peripherals.WIFI,
        ControllerConfig::default().with_initial_config(station_config),
    )
    .unwrap();

    let (stack, runner) = embassy_net::new(
        wifi_interface,
        NetConfig::dhcpv4(Default::default()),
        NET_RESOURCES.init(StackResources::new()),
        0x5eed_2026,
    );

    spawner.spawn(connection(controller).unwrap());
    spawner.spawn(net_task(runner).unwrap());

    stack.wait_config_up().await;
    if let Some(config) = stack.config_v4() {
        println!("usage-display: DHCP acquired IPv4 config: {:?}", config);
    } else {
        println!("usage-display: DHCP finished without IPv4 config");
    }

    if let Some(display) = oled.as_mut() {
        if let Err(error) = display.show_status("Checking time...") {
            println!("usage-display: OLED write failed: {:?}", error);
        }
    }
    println!("usage-display: fetching time via NTP");
    let mut clock = None;
    let result = match synchronize_clock::synchronize_clock(stack).await {
        Ok(unix_seconds) => {
            clock = Some((unix_seconds, embassy_time::Instant::now()));
            println!(
                "usage-display: NTP clock ready unix_seconds={}",
                unix_seconds
            );
            if let Some(display) = oled.as_mut() {
                if let Err(error) = display.show_status("Reading usage...") {
                    println!("usage-display: OLED write failed: {:?}", error);
                }
            }
            println!("usage-display: starting HTTPS usage request with certificate verification");
            embassy_time::with_timeout(Duration::from_secs(30), fetch_usage(stack))
                .await
                .unwrap_or(Err(UsageRequestError::Timeout))
        }
        Err(error) => Err(UsageRequestError::Clock(error)),
    };
    let mut displayed_usage = None;
    match result {
        Ok(snapshot) => {
            println!(
                "usage-display: ChatGPT HTTPS status=200 body_bytes={} weekly_used_percent={:?} weekly_window_seconds={:?} weekly_reset_at={:?}",
                snapshot.body_bytes,
                snapshot.weekly_used_percent,
                snapshot.weekly_window_seconds,
                snapshot.weekly_reset_at,
            );
            let remaining = remaining_percent::remaining_percent(
                snapshot.weekly_used_percent,
                snapshot.weekly_window_seconds,
            );
            displayed_usage = remaining.map(|value| (value, snapshot.weekly_reset_at));
            if let Some(display) = oled.as_mut() {
                let now = clock
                    .map(|(unix, instant)| unix + instant.elapsed().as_secs())
                    .unwrap_or(0);
                let result = match remaining {
                    Some(value) => display.show_remaining(value, snapshot.weekly_reset_at, now),
                    None => display.show_status("No weekly data"),
                };
                match result {
                    Ok(()) => println!(
                        "usage-display: OLED write complete remaining_percent={:?}",
                        remaining
                    ),
                    Err(error) => println!("usage-display: OLED write failed: {:?}", error),
                }
            }
        }
        Err(error) => {
            println!("usage-display: usage request failed: {:?}", error);
            if let Some(display) = oled.as_mut() {
                if let Err(error) = display.show_status("No data") {
                    println!("usage-display: OLED write failed: {:?}", error);
                }
            }
        }
    }

    loop {
        Timer::after(Duration::from_secs(60)).await;
        if let (Some(display), Some((remaining, reset_at)), Some((unix, instant))) =
            (oled.as_mut(), displayed_usage, clock)
        {
            if let Err(error) =
                display.show_remaining(remaining, reset_at, unix + instant.elapsed().as_secs())
            {
                println!("usage-display: OLED write failed: {:?}", error);
            }
        }
    }
}

struct UsageResult {
    body_bytes: usize,
    weekly_used_percent: Option<u32>,
    weekly_window_seconds: Option<u32>,
    weekly_reset_at: Option<u64>,
}

#[derive(Debug)]
enum UsageRequestError {
    Clock(synchronize_clock::ClockError),
    TlsSetup,
    Timeout,
    BuildHeader,
    Http(reqwless::Error),
    HttpStatus(u16),
    BodyRead,
    Json,
}

async fn fetch_usage(stack: embassy_net::Stack<'static>) -> Result<UsageResult, UsageRequestError> {
    let tcp_state = TCP_STATE.init(TcpClientState::new());
    let tcp_client = TcpClient::new(stack, tcp_state);
    let dns_socket = DnsSocket::new(stack);

    // Wi-Fi keeps the hardware entropy source enabled during this request.
    static TLS_RNG: StaticCell<esp_hal::rng::Trng> = StaticCell::new();
    let rng = TLS_RNG.init(esp_hal::rng::Trng::try_new().map_err(|_| UsageRequestError::TlsSetup)?);
    let tls_context = mbedtls_rs::Tls::new(rng).map_err(|_| UsageRequestError::TlsSetup)?;
    let ca = mbedtls_rs::Certificate::new_no_copy(include_bytes!("../certs/gts-root-r4.der"))
        .map_err(|_| UsageRequestError::TlsSetup)?;
    let tls = TlsConfig::new(
        mbedtls_rs::TlsVersion::Tls1_2,
        ca,
        None,
        tls_context.reference(),
    );
    let mut client = HttpClient::new_with_tls(&tcp_client, &dns_socket, tls);

    let mut authorization = String::<2304>::new();
    write!(authorization, "Bearer {CHATGPT_ACCESS_TOKEN}")
        .map_err(|_| UsageRequestError::BuildHeader)?;
    let headers = [
        ("Authorization", authorization.as_str()),
        ("ChatGPT-Account-Id", CHATGPT_ACCOUNT_ID),
        ("User-Agent", "usage-display-esp32/0.1"),
    ];

    let mut request = client
        .request(Method::GET, USAGE_URL)
        .await
        .map_err(UsageRequestError::Http)?
        .headers(&headers)
        .accept(ContentType::ApplicationJson);

    let mut header_buffer = [0u8; 4096];
    let mut response = request
        .send(&mut header_buffer)
        .await
        .map_err(UsageRequestError::Http)?;
    let status = response.status;
    let mut body = response.body().reader();
    let mut body_buffer = [0u8; 16_384];
    let mut body_bytes = 0;

    while body_bytes < body_buffer.len() {
        let read = body
            .read(&mut body_buffer[body_bytes..])
            .await
            .map_err(|_| UsageRequestError::BodyRead)?;
        if read == 0 {
            break;
        }
        body_bytes += read;
    }

    if status != Status::Ok {
        return Err(UsageRequestError::HttpStatus(status.0));
    }

    let (payload, _) = serde_json_core::from_slice::<UsagePayload>(&body_buffer[..body_bytes])
        .map_err(|_| UsageRequestError::Json)?;
    let weekly = payload.rate_limit.and_then(|rate_limit| {
        rate_limit
            .secondary_window
            .filter(|window| window.limit_window_seconds == Some(604_800))
            .or_else(|| {
                rate_limit
                    .primary_window
                    .filter(|window| window.limit_window_seconds == Some(604_800))
            })
    });

    Ok(UsageResult {
        body_bytes,
        weekly_used_percent: weekly.as_ref().and_then(|window| window.used_percent),
        weekly_window_seconds: weekly
            .as_ref()
            .and_then(|window| window.limit_window_seconds),
        weekly_reset_at: weekly.as_ref().and_then(|window| window.reset_at),
    })
}

#[embassy_executor::task]
async fn net_task(mut runner: Runner<'static, Interface>) {
    runner.run().await;
}

#[embassy_executor::task]
async fn connection(mut controller: WifiController<'static>) {
    loop {
        println!("usage-display: connecting to Wi-Fi");

        match controller.connect_async().await {
            Ok(info) => {
                println!("usage-display: Wi-Fi connected to {:?}", info);
                controller.wait_for_disconnect_async().await.ok();
                println!("usage-display: Wi-Fi disconnected");
            }
            Err(error) => {
                println!("usage-display: Wi-Fi connection failed: {:?}", error);
            }
        }

        Timer::after(Duration::from_secs(5)).await;
    }
}

#![no_std]
#![no_main]

use core::panic::PanicInfo;

use embassy_executor::Spawner;
use embassy_net::{Config as NetConfig, Runner, StackResources};
use embassy_time::{Duration, Timer};
use esp_hal::{clock::CpuClock, timer::timg::TimerGroup};
use esp_radio::wifi::{
    AuthenticationMethodConfig, Config, ControllerConfig, Interface, WifiController,
    sta::StationConfig,
};
use esp_println::println;
use static_cell::StaticCell;

esp_bootloader_esp_idf::esp_app_desc!();

const WIFI_SSID: &str = env!("WIFI_SSID");
const WIFI_PASSWORD: &str = env!("WIFI_PASSWORD");

static NET_RESOURCES: StaticCell<StackResources<3>> = StaticCell::new();

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[esp_hal::main]
async fn main(spawner: Spawner) -> ! {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    esp_alloc::heap_allocator!(size: 100 * 1024);

    let timg0 = TimerGroup::new(peripherals.TIMG0);
    esp_rtos::start(timg0.timer0, peripherals.FROM_CPU_INTR0);

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
    println!("usage-display: Wi-Fi configured");

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

    loop {
        Timer::after(Duration::from_secs(60)).await;
    }
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

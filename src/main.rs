use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::{cpu::Core, peripherals::Peripherals, task::thread::ThreadSpawnConfiguration};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    http::server::EspHttpServer,
    log::EspLogger,
    nvs::EspDefaultNvsPartition,
    timer::EspTaskTimerService,
    wifi::{AsyncWifi, EspWifi},
};
use futures::executor::block_on;
use lazy_static::lazy_static;
use log::info;
use parking_lot::Mutex;
use smart_leds::SmartLedsWrite;
use std::{str, thread::sleep, time::Duration};
use types::Pixel;
use ws2812_esp32_rmt_driver::Ws2812Esp32Rmt;

const SSID: &str = match option_env!("YOUR_SSID") {
    Some(v) => v,
    None => "YOUR_SSID",
};
const PASSWORD: &str = match option_env!("YOUR_PASSWORD") {
    Some(v) => v,
    None => "YOUR_PASSWORD",
};

const STACK_SIZE: usize = 0x4000;

///! Ignore this
const _FRONTEND_URL: &str = "http://192.168.194.11:3000";

const NUM_LEDS: usize = 100;

lazy_static! {
    pub static ref NEO_PIXELS: Mutex<Ws2812Esp32Rmt<'static>> = Mutex::new(
        Ws2812Esp32Rmt::new(
            Peripherals::take().unwrap().rmt.channel0,
            Peripherals::take().unwrap().pins.gpio17
        )
        .unwrap()
    );
    pub static ref LED_DATA: Mutex<[Pixel; NUM_LEDS]> = Mutex::new(generate_full_array(0, 0, 0));
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();
    EspLogger::initialize_default();

    write_led_data()?;

    ThreadSpawnConfiguration {
        name: Some(b"LED-THREAD\0"),
        stack_size: STACK_SIZE,
        priority: 1,
        ///! I tried putting that to 24 and 1 and both times had flickering.
        pin_to_core: Some(Core::Core1),
        ..Default::default()
    }
    .set()?;

    std::thread::spawn(|| -> anyhow::Result<()> {
        loop {
            let data = *LED_DATA.lock();
            unsafe { LED_DATA.force_unlock() };
            *LED_DATA.lock() = data.map(|mut pixel| {
                pixel.rainbow_tick();
                pixel
            });

            write_led_data()?;

            sleep(Duration::from_millis(50));
        }
    });

    ThreadSpawnConfiguration::default().set()?;

    let peripherals = Peripherals::take().unwrap();
    let sys_loop = EspSystemEventLoop::take()?;
    let nvs = EspDefaultNvsPartition::take()?;
    let timer_service = EspTaskTimerService::new()?;

    let mut wifi = AsyncWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), Some(nvs))?,
        sys_loop,
        timer_service,
    )?;

    block_on(connect_wifi(&mut wifi))?;

    let _server = create_server()?;

    prevent_return()
}

fn prevent_return() -> ! {
    loop {
        sleep(Duration::new(10, 0))
    }
}

fn write_led_data() -> anyhow::Result<()> {
    NEO_PIXELS.lock().write(LED_DATA.lock().iter().cloned())?;
    Ok(())
}

fn generate_full_array(red: u8, green: u8, blue: u8) -> [Pixel; NUM_LEDS] {
    [Pixel {
        r: red,
        g: green,
        b: blue,
    }; NUM_LEDS]
}

async fn connect_wifi(wifi: &mut AsyncWifi<EspWifi<'static>>) -> anyhow::Result<()> {
    let wifi_configuration: Configuration = Configuration::Client(ClientConfiguration {
        ssid: SSID.into(),
        bssid: None,
        auth_method: AuthMethod::WPA2Personal,
        password: PASSWORD.into(),
        channel: None,
    });

    wifi.set_configuration(&wifi_configuration)?;

    wifi.start().await?;
    info!("Wifi started");

    wifi.connect().await?;
    info!("Wifi connected");

    wifi.wait_netif_up().await?;
    info!("Wifi netif up");

    Ok(())
}

fn create_server() -> anyhow::Result<EspHttpServer<'static>> {
    let server_configuration = esp_idf_svc::http::server::Configuration {
        stack_size: STACK_SIZE,
        max_sessions: 8,
        max_open_sockets: 4,
        uri_match_wildcard: true,
        ..Default::default()
    };

    Ok(EspHttpServer::new(&server_configuration)?)
}

// Dependencies
use esp_idf_hal::peripherals::Peripherals;
mod config;
mod index;
use config::credentials;
use embedded_svc::http::Method;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::server::{Configuration as HttpServerConfig, EspHttpServer};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use index::index_html;
use std::{thread::sleep, time::Duration};

fn main() {
    server()
}
// Server
fn server() {
    esp_idf_sys::link_patches();

    // Configure Wifi
    let peripherals = Peripherals::take().unwrap();
    let sysloop = EspSystemEventLoop::take().unwrap();
    let nvs = EspDefaultNvsPartition::take().unwrap();

    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sysloop.clone(), Some(nvs)).unwrap(),
        sysloop,
    )
    .unwrap();

    wifi.set_configuration(&Configuration::Client(ClientConfiguration {
        ssid: credentials().name.into(),
        bssid: None,
        auth_method: AuthMethod::None,
        password: credentials().pass.into(),
        channel: None,
    }))
    .unwrap();

    // Start Wifi
    wifi.start().unwrap();

    // Connect Wifi
    wifi.connect().unwrap();

    // Wait until the network interface is up
    wifi.wait_netif_up().unwrap();

    // HTTP Configuration
    // Create HTTP Server Connection control
    let mut httpserver = EspHttpServer::new(&HttpServerConfig::default()).unwrap();
    httpserver
        .fn_handler("/", Method::Get, |request| {
            let html = index_html();
            let arg = request.uri();
            println!("{:?}", get_args(arg));
            let mut response = request.into_ok_response().unwrap();
            response.write(html.as_bytes()).unwrap();
            Ok(())
        })
        .unwrap();

    loop {
        sleep(Duration::from_millis(1000));
    }
}

fn get_args(url: &str) -> Vec<u64> {
    let variables: Vec<u64> = url
        .split("&")
        .flat_map(|x| x.split("=").nth(1))
        .map(|num| num.parse().unwrap_or(0))
        .collect();
    variables
}

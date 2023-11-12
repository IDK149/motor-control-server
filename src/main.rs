mod index;
use embedded_svc::http::Method;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::server::{Configuration as HttpServerConfig, EspHttpServer};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
use index::index_html;
use std::{thread::sleep, time::Duration};

fn main() {
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
        ssid: "WIFI GRATIS".into(),
        bssid: None,
        auth_method: AuthMethod::None,
        password: "CamiAna1".into(),
        channel: None,
    }))
    .unwrap();

    // Start Wifi
    wifi.start().unwrap();

    // Connect Wifi
    wifi.connect().unwrap();

    // Wait until the network interface is up
    wifi.wait_netif_up().unwrap();

    // Get the local IP address
    println!("Server IP address {{number}}",);

    // HTTP Configuration
    // Create HTTP Server Connection Handle
    let mut httpserver = EspHttpServer::new(&HttpServerConfig::default()).unwrap();

    // Define Server Request Handler Behaviour on Get for Root URL
    httpserver
        .fn_handler("/", Method::Get, |request| {
            // Retrieve html String
            let html = index_html();
            let uri = request.uri();
            println!("{:?}", variables_format(uri));
            let mut response = request.into_ok_response().unwrap();
            // Return Requested Object (Index Page)
            response.write(html.as_bytes()).unwrap();
            Ok(())
        })
        .unwrap();

    // Loop to Avoid Program Termination
    loop {
        sleep(Duration::from_millis(1000));
    }
}

fn variables_format(url: &str) -> Vec<u64> {
    let variables: Vec<u64> = url
        .split("&")
        .flat_map(|x| x.split("=").nth(1))
        .filter_map(|num| num.parse().ok())
        .collect();
    variables
}

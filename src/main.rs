use embedded_svc::http::Method;
use embedded_svc::wifi::{AuthMethod, ClientConfiguration, Configuration};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_svc::eventloop::EspSystemEventLoop;
use esp_idf_svc::http::server::{Configuration as HttpServerConfig, EspHttpServer};
use esp_idf_svc::nvs::EspDefaultNvsPartition;
use esp_idf_svc::wifi::{BlockingWifi, EspWifi};
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
    println!("Server IP address");

    // HTTP Configuration
    // Create HTTP Server Connection Handle
    let mut httpserver = EspHttpServer::new(&HttpServerConfig::default()).unwrap();

    // Define Server Request Handler Behaviour on Get for Root URL
    httpserver
        .fn_handler("/", Method::Get, |request| {
            // Retrieve html String
            let html = index_html();
            let uri = request.uri();
            println!("{}", uri);
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

fn index_html() -> String {
    format!(
        r#"
<!DOCTYPE html>
<html lang="en">

<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Document</title>
    <style>
* {{
    margin: 0;
    padding: 0;
}}

.container {{
    color: white;
    min-height: 100vh;
    display: flex;
    justify-content: center;
    align-items: center;
    background-color: #2d3250;
}}

.subcontainer {{
    font-size: 1.5rem;
    border-radius: 20px;
    width: 50vw;
    height: 60vh;
    background-color: #424769;
    display: flex;
    flex-direction: column;
    justify-content: space-evenly;
    align-items: center;
}}

.formulario {{
    width: 100%;
    height: 40vh;
    color: #f9b17a;
    display: flex;
    flex-direction: column;
    justify-content: space-around;
    align-items: center;
    font-size: 4rem;
}}

.options {{
    width: 100%;
    display: flex;
    justify-content: space-around;
}}

p {{
    text-align: center;
}}

input {{
    text-align: center;
    color: white;
    background-color: transparent;
    border: none;
    border-bottom: 1px solid #676f9d;
    outline: none;
}}

.send {{
    width: 100px;
    height: 30px;
    overflow: hidden;
    border-radius: 5px;
    border: 1px solid black;
    color: black;
    background-color: #f9b17a;
}}
    </style>
</head>
<body>
	<div class="container">
		<div class="subcontainer">
			<div class="title">
				<h1>Pong-Master</h1>
			</div>
			<form class="formulario">
				<div class="options">
					<div class="velocity">
						<p>V</p>
						<input type="text" name="VelocidadValue">
					</div>
					<div class="angle">
						<p>θ</p>
						<input type="text" name="angleValue">
					</div>
				</div>
				<input type="submit" value="Enviar" class="send">
			</form>
		</div>
	</div>
</body>


</html>
        "#
    )
}

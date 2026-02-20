use anyhow::{bail, Context};
use esp_idf_svc::{
    eventloop::EspSystemEventLoop,
    hal::{delay::FreeRtos, gpio::PinDriver, peripherals::Peripherals},
    nvs::EspDefaultNvsPartition,
    wifi::{
        AccessPointConfiguration, AuthMethod, BlockingWifi, ClientConfiguration, Configuration,
        EspWifi,
    },
};
use log::info;
use std::{collections::HashSet, net::UdpSocket};

const BLOCKLIST: &str = include_str!("blacklist.txt");
const SSID: &str = "Nexus One";
const PASSWORD: &str = "aaaaaaaa";
const PORT: u16 = 53;
// std::result::Result::Ok;

// fn init_wifi() -> anyhow::Result<BlockingWifi<EspWifi<'static>>> {
//     let peripherals = Peripherals::take()?;

//     let sys_loop = EspSystemEventLoop::take()?;
//     let mut wifi = BlockingWifi::wrap(
//         EspWifi::new(peripherals.modem, sys_loop.clone(), None)?,
//         sys_loop,
//     )?;

//     wifi.set_configuration(&Configuration::Client(ClientConfiguration {
//         ssid: SSID.try_into().unwrap(),
//         password: PASSWORD.try_into().unwrap(),
//         auth_method: AuthMethod::WPA2Personal,
//         ..Default::default()
//     }))?;

//     wifi.start()?;
//     wifi.connect()?;
//     wifi.wait_netif_up()?;
//     Ok(wifi)
// }
fn init_wifi() -> anyhow::Result<BlockingWifi<EspWifi<'static>>> {
    let peripherals = Peripherals::take()?;
    let sys_loop = EspSystemEventLoop::take()?;
    let mut wifi = BlockingWifi::wrap(
        EspWifi::new(peripherals.modem, sys_loop.clone(), None)?,
        sys_loop,
    )?;

    wifi.set_configuration(&Configuration::AccessPoint(AccessPointConfiguration {
        ssid: "esp-sinker".try_into().unwrap(),
        password: "12345678".try_into().unwrap(),
        auth_method: AuthMethod::WPA2Personal,
        ssid_hidden: false,
        ..Default::default()
    }))?;

    wifi.start()?;
    wifi.wait_netif_up()?;
    Ok(wifi)
}

// fn led_debug() -> anyhow::Result<()> {
//     let peripherals = Peripherals::take()?;
//     let mut led = PinDriver::output(peripherals.pins.gpio2)?;
//     println!("LED initialized, entering loop");
//     loop {
//         led.set_high()?;
//         FreeRtos::delay_ms(100);
//         led.set_low()?;
//         FreeRtos::delay_ms(100);
//     }
// }
fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();
    let _nvs = EspDefaultNvsPartition::take()?;
    esp_idf_svc::log::EspLogger::initialize_default();
    let wifi = init_wifi().expect("error in init wifi");
    let ip_info = wifi.wifi().sta_netif().get_ip_info()?;
    println!("Connected with IP: {}", ip_info.ip);

    let socket = UdpSocket::bind(("0.0.0.0", PORT))?;
    println!("DNS server is listening on port {}", PORT);
    let mut buffer = [0u8; 512];
    // led_debug().expect("error in led debug");
    loop {
        let (size, src) = socket.recv_from(&mut buffer)?;
        
        match dns_parser::Packet::parse(&buffer[..size]) {
            Ok(packet) => {
                for question in packet.questions{
                    println!("Received query for: {}", question.qname);
                }
            }
            Err(e) => {
                println!("Failed to parse DNS packet: {}", e);
                continue;
            }
        }
        
        println!("Received {} bytes from {}", size, src);
        // std::thread::sleep(core::time::Duration::from_secs(5));
    }
    // Ok(())
}

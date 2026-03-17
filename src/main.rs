use anyhow::{Error, Result};
use dns_parser::Builder;
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
use std::{collections::HashSet, net::UdpSocket, sync::Arc, thread};

const BLOCKLIST: &str = include_str!("blacklist.txt");
const SSID: &str = "Nexus One";
const PASSWORD: &str = "aaaaaaaa";
const IP: &str = "0.0.0.0";
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

fn build_sinkhole_response(query: &[u8]) -> Vec<u8> {
    let mut response = Vec::new();

    response.push(query[0]);
    response.push(query[1]);

    response.push(0x81);
    response.push(0x80);

    response.push(query[4]);
    response.push(query[5]);

    response.push(0x00);
    response.push(0x01);

    response.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

    response.extend_from_slice(&query[12..]);

    response.extend_from_slice(&[0xC0, 0x0C]);

    response.extend_from_slice(&[0x00, 0x01]);

    response.extend_from_slice(&[0x00, 0x01]);

    response.extend_from_slice(&[0x00, 0x00, 0x00, 0x3C]);

    response.extend_from_slice(&[0x00, 0x04]);

    response.extend_from_slice(&[0x00, 0x00, 0x00, 0x00]);

    response
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
    let blocklist: HashSet<String> = BLOCKLIST
        .lines()
        .map(|line| line.trim().to_string())
        .collect();
    let socket = Arc::new(UdpSocket::bind((IP, PORT))?);
    println!("DNS server is listening on port {}", PORT);
    // let socket = Arc::new(socket);
    // led_debug().expect("error in led debug");

    //mspc create a comunication channel so tx sends and rx receives
    let (tx, rx) = std::sync::mpsc::channel::<(Vec<u8>, std::net::SocketAddr)>();

    let resolver = Arc::clone(&socket);
    let listener = Arc::clone(&socket);

    //listner: receive packet through tx
    thread::spawn(move || {
        let mut buffer = [0u8; 512];
        loop {
            let (size, src) = listener.recv_from(&mut buffer).unwrap();
            tx.send((buffer[..size].to_vec(), src)).unwrap();
        }
    });

    //resolver
    thread::spawn(move || {
        while let Ok((bytes, src)) = rx.recv() {
            match dns_parser::Packet::parse(&bytes) {
                Ok(packet) => {
                    for question in packet.questions {
                        let domain = question.qname.to_string();
                        if blocklist.contains(&domain) {
                            let response = build_sinkhole_response(&bytes);
                            resolver.send_to(&response, src).unwrap();
                            println!("BLOCKED: {}", question.qname);
                        } else {
                            println!("ALLOWED: {}", question.qname);
                        }
                    }
                }
                Err(e) => {
                    println!("Failed to parse DNS packet: {}", e);
                }
            }
        }
    });
    //prevent the code from quitting
    loop {
        thread::sleep(core::time::Duration::from_secs(1));
    }
    // Ok(())
}

// Scan for IPv4 addresses on the network

use std::net::{IpAddr, Ipv4Addr, TcpStream};
//use pnet::packet::ip;
//use std::process::Command;
use rayon::prelude::*;
use ping::ping;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use dns_lookup::lookup_addr;

pub fn scan_port(nb1: u8, nb2: u8, port: u16) {

    let mutex = Arc::new(Mutex::new(Vec::new()));

    (0..=255).into_par_iter().for_each(|octet3| {
        (0..=255).into_par_iter().for_each(|octet4| {
            let ip = IpAddr::V4(Ipv4Addr::new(nb1, nb2, octet3, octet4));
            let addr = format!("{}.{}.{}.{}:{}", nb1, nb2, octet3, octet4, port);
            match TcpStream::connect(addr) {
                Ok(_) => {
                    match lookup_addr(&ip) {
                        Ok(hostname) => {
                            let res = format!("{} ({})", ip, hostname);
                            let mut result = mutex.lock().unwrap();
                            result.push(res);
                        },
                        Err(_) => {},
                    }
                },
                Err(_) => {},
            }
        })
    });

    let result = mutex.lock().unwrap();
    println!("Port: {}", port);
    for ip in result.iter() {
        println!("\tIP: {}", ip);
    }
}

pub fn ipv4(nb1: u8, nb2: u8) {

    let mutex = Arc::new(Mutex::new(Vec::new()));
  
    (1..=255).into_par_iter().for_each(|octet4| {
        let ip = IpAddr::V4(Ipv4Addr::new(nb1, nb2, 1, octet4));
        match ping(ip, Some(Duration::from_millis(500)), None, None, None, None) {
            Ok(_) => {
                match lookup_addr(&ip) {
                    Ok(hostname) => {
                        let res = format!("{} ({})", ip, hostname);
                        let mut result = mutex.lock().unwrap();
                        result.push(res);
                    },
                    Err(_) => {},
                }
            },
            Err(_) => {},
        }
    });

    let result = mutex.lock().unwrap();
    for ip in result.iter() {
        println!("IP: {}", ip);
    }
}

// fn ping_attempt(ip_address: Ipv4Addr) {

//     let output = Command::new("ping")
//     .arg("-c")
//     .arg("1")
//     .arg(ip_address.to_string())
//     .output()
//     .expect("Failed to execute command");

//     let output_str = String::from_utf8_lossy(&output.stdout);

//     if output_str.contains("perte 0%") { // Changer la chaîne de caractères ?
//         println!("{} is reachable", ip_address);
//     }
// }
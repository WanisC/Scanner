// Scan addresses for specific port on the network

use std::net::{IpAddr, Ipv4Addr, TcpStream};
use rayon::prelude::*;
use std::sync::{Arc, Mutex};
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
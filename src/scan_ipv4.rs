// Scan for IPv4 addresses on the network

use std::{
    net::{IpAddr, Ipv4Addr}, 
    sync::{Arc, Mutex}, 
    time::Duration, 
    thread,
    fs,
    io::Write,
    collections::HashMap
};
use rayon::prelude::*;
use ping::ping;
use dns_lookup::lookup_addr;
use serde::Serialize;
use chrono::Local;

#[derive(Serialize)]
pub struct ScanResult {
    success: Vec<String>,
    failure: HashMap<String, Vec<String>>,
}

pub fn ipv4(octets: &[u8]) {
    let scan_range = match octets.len() {
        1 => {
            let o1 = octets[0];
            (0..=255u8).flat_map(move |o2|
                (0..=255u8).flat_map(move |o3|
                    (1..=254u8).map(move |o4|
                        Ipv4Addr::new(o1, o2, o3, o4)
                    )
                )
            ).collect::<Vec<_>>()
        }
        2 => {
            let (o1, o2) = (octets[0], octets[1]);
            (0..=255u8).flat_map(move |o3|
                (1..=254u8).map(move |o4|
                    Ipv4Addr::new(o1, o2, o3, o4)
                )
            ).collect::<Vec<_>>()
        }
        3 => {
            let (o1, o2, o3) = (octets[0], octets[1], octets[2]);
            (1..=254u8)
                .map(move |o4| Ipv4Addr::new(o1, o2, o3, o4))
                .collect::<Vec<_>>()
        }
        4 => {
            vec![Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3])]
        }
        _ => unreachable!(),
    };

    let output = Arc::new(Mutex::new(ScanResult {
        success: Vec::new(),
        failure: HashMap::new(),
    }));
    let max_threads_pings = 50;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(max_threads_pings)
        .build()
        .unwrap();
  
    pool.install(|| {
        scan_range.into_par_iter().for_each(|ip_v4| {
            let ip = IpAddr::V4(ip_v4);
            thread::sleep(Duration::from_millis(20));
             match ping(ip, Some(Duration::from_millis(500)), None, None, None, None) {
                Ok(_) => {
                    let res = match lookup_addr(&ip) {
                        Ok(hostname) => format!("{:<15} ({})", ip, hostname),
                        Err(_) => format!("{}", ip),
                    };
                    let output = Arc::clone(&output);
                    let mut output = output.lock().unwrap();
                    output.success.push(res);
                }
                Err(_) => {
                    let output = Arc::clone(&output);
                    let mut output = output.lock().unwrap();
                    let entry = output.failure.entry("Unreachable".to_string()).or_default();
                    entry.push(ip.to_string());
                }
            }
        });
    });

    let mut result = output.lock().unwrap();
    result.success.sort();
    for v in result.failure.values_mut() {
        v.sort();
    }
    let json = serde_json::to_string_pretty(&*result).expect("JSON serialization error");

    fs::create_dir_all("logs").expect("Unable to create 'logs' folder");

    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
    let filename = format!("logs/IPv4_{}.json", timestamp);
    let mut file = fs::File::create(filename).expect("Unable to create the JSON file");
    file.write_all(json.as_bytes()).expect("Error writing to JSON file");
}
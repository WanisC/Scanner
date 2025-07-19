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
pub struct SuccessEntry {
    ip: String,
    
    #[serde(skip_serializing_if = "Option::is_none")]
    hostname: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    ipv6: Option<String>,
}
#[derive(Serialize)]
pub struct ScanResult {
    success: Vec<SuccessEntry>,
    failure: HashMap<String, Vec<String>>,
}

impl ScanResult {
    pub fn new() -> Self {
        ScanResult {
            success: Vec::new(),
            failure: HashMap::new(),
        }
    }

    pub fn sort_by_ip(&mut self) {
        self.success.sort_by_key(|entry| {
            entry.ip.parse::<Ipv4Addr>().unwrap_or(Ipv4Addr::new(0, 0, 0, 0))
        });

        for ips in self.failure.values_mut() {
            ips.sort_by_key(|ip| ip.parse::<Ipv4Addr>().unwrap_or(Ipv4Addr::new(0, 0, 0, 0)));
        }
    }

    pub fn to_json_file(&self) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self)
            .expect("JSON serialization error");

        fs::create_dir_all("logs")?;

        let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");
        let filename = format!("logs/IPv4_{}.json", timestamp);
        let mut file = fs::File::create(filename)?;
        file.write_all(json.as_bytes())?;

        Ok(())
    }

    pub fn display(&mut self) {
        for entry in &self.success {
            match &entry.hostname {
                Some(host) => println!("\t{:<15} ({})", entry.ip, host),
                None => println!("\t{:<15} (no hostname)", entry.ip),
            }
        }
        println!("[-] Scan completed without log generation.");
        println!("[-] {} accessible hosts", self.success.len());
        println!(
            "[-] {} unreachable hosts",
            self.failure.values().map(|v| v.len()).sum::<usize>()
        );
    }
}

fn generate_ips(octets: &[u8]) -> Vec<Ipv4Addr> {
    match octets.len() {
        1 => {
            let o1 = octets[0];
            (0..=255).flat_map(move |o2| {
                (0..=255).flat_map(move |o3| {
                    (1..=254).map(move |o4| Ipv4Addr::new(o1, o2, o3, o4))
                })
            }).collect()
        }
        2 => {
            let (o1, o2) = (octets[0], octets[1]);
            (0..=255).flat_map(move |o3| {
                (1..=254).map(move |o4| Ipv4Addr::new(o1, o2, o3, o4))
            }).collect()
        }
        3 => {
            let (o1, o2, o3) = (octets[0], octets[1], octets[2]);
            (1..=254).map(move |o4| Ipv4Addr::new(o1, o2, o3, o4)).collect()
        }
        4 => vec![Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3])],
        _ => unreachable!("Invalid number of octets"),
    }
}

pub fn ipv4(octets: &[u8], with_log: &str) {

    let log_enabled = match with_log {
        "1" => true,
        "0" => false,
        _ => {
            eprintln!("Invalid ‘log’ value: use \"0\" or \"1\"");
            false
        }
    };

    let scan_range = generate_ips(&octets);

    let output = Arc::new(Mutex::new(ScanResult::new()));
    let max_threads_pings = 50;

    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(max_threads_pings)
        .build()
        .unwrap();
  
    pool.install(|| {
        scan_range.into_par_iter().for_each(|ip_v4| {
            let ip = IpAddr::V4(ip_v4);

            let mut local_success = Vec::new();
            let mut local_failure: HashMap<String, Vec<String>> = HashMap::new();

            thread::sleep(Duration::from_millis(20));

            match ping(ip, Some(Duration::from_millis(500)), None, None, None, None) {
                Ok(_) => {
                    let hostname = lookup_addr(&ip).ok(); 
                    local_success.push(SuccessEntry { ip: ip.to_string(), hostname: hostname, ipv6: None});
                }
                Err(_) => {
                    local_failure.entry("Unreachable".to_string()).or_default().push(ip.to_string());
                }
            }

            let mut output = output.lock().unwrap();
            output.success.extend(local_success);
            for (k, v) in local_failure {
                output.failure.entry(k).or_default().extend(v);
            }
        });
    });

    let mut result = output.lock().unwrap();
    result.sort_by_ip();
    if log_enabled {
        result.to_json_file().expect("Failed to create JSON log");  
    } else {
        result.display();
    }
}
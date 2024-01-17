// Scan for IPv4 addresses on the network

use std::net::Ipv4Addr;
use std::process::Command;
use std::str;
use rayon::prelude::*;

pub fn scan(nb1: u8, nb2: u8) {
    println!("Scanning local network {}.{}...", nb1, nb2);

    // Create a vector of all possible IP addresses for the last two octets
    let ips: Vec<Ipv4Addr> = (0..255)
        .flat_map(|nb3| (0..255).map(move |nb4| Ipv4Addr::new(nb1, nb2, nb3, nb4)))
        .collect();

    ips.par_iter()
        .for_each(|ip| {
            let output = Command::new("ping")
                .arg("-c 1")
                .arg("-a")
                .arg(ip.to_string())
                .output();
            match output {
                Ok(output) => {
                    if output.status.success() {
                        let str_output = str::from_utf8(&output.stdout).expect("Invalid UTF-8");

                        if let Some(hostname) = extract_host(str_output) {
                            println!("{} : {}", ip, hostname);
                        } else {
                            println!("{}", ip);
                        }
                    }
                }
                Err(_) => {}
            }
        });
}

fn extract_host(ping_output: &str) -> Option<&str> {
    let hostname = ping_output
        .lines()
        .find_map(|line| {
            if line.starts_with("PING") {
                let start = line.find('(')? + 1;
                let end = line.find(')')?;
                Some(&line[start..end])
            } else {
                None
            }
        });

    hostname
}
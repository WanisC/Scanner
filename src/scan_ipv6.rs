// Scan for IPv6 addresses on the network

use std::net::{IpAddr, Ipv4Addr};
use rayon::prelude::*;
use ping::ping;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use dns_lookup::lookup_addr;

fn collect_ipv4(nb1: u8, nb2: u8) -> Vec<Ipv4Addr> {
    // Lock the mutex to store the results
    let mutex = Arc::new(Mutex::new(Vec::new()));
  
    // Scan the network by pinging each IP address (nb1.nb2.1.1-255)
    (1..=255).into_par_iter().for_each(|octet4| {
        let ipv4 = Ipv4Addr::new(nb1, nb2, 1, octet4);
        let ip = IpAddr::V4(ipv4);
        // Ping the IP address
        if ping(ip, Some(Duration::from_millis(500)), None, None, None, None).is_ok() {
            let mutex = Arc::clone(&mutex);
            let mut mutex = mutex.lock().unwrap();
            mutex.push(ipv4);
        }
    });

    // Return the results
    let result = mutex.lock().unwrap(); 
    result.clone()
}

pub fn ipv6(nb1: u8, nb2: u8) {
    // Vector of all IPv4 addresses
    let all_ipv4 = collect_ipv4(nb1, nb2);

    // Lock the mutex to store the results
    let output = Arc::new(Mutex::new(Vec::new()));
    
    all_ipv4.into_par_iter().for_each(|ip| { 
        let hex_ipv4 = format!("{:02x}{:02x}{:02x}{:02x}", ip.octets()[0], ip.octets()[1], ip.octets()[2], ip.octets()[3]);

        let ip_6to4 = format!("2002:{}::1", hex_ipv4); // 6to4 address format

        let ip_addrv4 = IpAddr::V4(ip);

        // Looking for the hostname
        if let IpAddr::V4(ipv4) = ip_addrv4 {
            let length = ipv4.to_string().len();
            let space = " ".repeat(15 - length);
            let res = match lookup_addr(&ip_addrv4) {
                Ok(hostname) => format!("ipV4: {}{}6to4: {} ({})", ipv4, space, ip_6to4, hostname),
                Err(_) => format!("ipV4: {}{}6to4: {}", ipv4, space, ip_6to4),
            };
            let output = Arc::clone(&output);
            let mut output = output.lock().unwrap();
            output.push(res);
        }
    });

    // Let's build a string with the results
    let final_result = output.lock().unwrap().join("\n");
    println!("{}", final_result);
}
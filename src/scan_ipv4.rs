// Scan for IPv4 addresses on the network

use std::net::{IpAddr, Ipv4Addr};
use rayon::prelude::*;
use ping::ping;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use dns_lookup::lookup_addr;

pub fn ipv4(nb1: u8, nb2: u8) {

    // Lock the mutex to store the results
    let output = Arc::new(Mutex::new(Vec::new()));
  
    // Scan the network by pinging each IP address (nb1.nb2.1.1-255)
    (1..=255).into_par_iter().for_each(|octet4| {
        let ip = IpAddr::V4(Ipv4Addr::new(nb1, nb2, 1, octet4));
        // Ping the IP address
        if ping(ip, Some(Duration::from_millis(500)), None, None, None, None).is_ok() {
            // Get the hostname (via reverse DNS lookup)
            let res = match lookup_addr(&ip) {
                Ok(hostname) => {
                    let length = ip.to_string().len();
                    let space = " ".repeat(15 - length);
                    format!("{}{}({})", ip, space, hostname)
                },
                Err(_) => format!("{}", ip),
            };
            let output = Arc::clone(&output);
            let mut output = output.lock().unwrap();
            output.push(res);
        }
    });

    // Let's build a string with the results
    let string = Arc::new(Mutex::new(String::new()));
    // By iterating over the vector of results (with parallelism)
    output.lock().unwrap().clone()
        .into_par_iter().for_each(|ip| {
            let string = Arc::clone(&string);
            let mut string = string.lock().unwrap(); 
            *string += format!("{}\n", ip).as_str();
        });
    println!("{}", string.lock().unwrap());
}
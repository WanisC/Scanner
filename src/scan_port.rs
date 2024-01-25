// Scan addresses for specific port on the network

use std::net::{IpAddr, Ipv4Addr, TcpStream, SocketAddr};
use rayon::prelude::*;
use tokio::time::{timeout, Duration};
use std::sync::{Arc, Mutex};
// use dns_lookup::lookup_addr;

// Parallelism
pub fn port_paral(nb1: u8, nb2: u8, nb3: u8, nb4: u8, port: Vec<u16>) {

    let start_port = port[0];
    let end_port = port[1];
    if start_port > end_port {
        panic!("Invalid port range");
    }
    let ipv4 = IpAddr::V4(Ipv4Addr::new(nb1, nb2, nb3, nb4));
    let output = Arc::new(Mutex::new(Vec::new()));

    (start_port..=end_port).into_par_iter().for_each(|port| {
        let ipv4 = ipv4.to_string();
        let ipv4 = ipv4.as_str();

        let stream = TcpStream::connect((ipv4, port));

        let curr_port = match stream {
            Ok(_) => {
                format!("Port {} is open", port)
            },
            Err(_) => {
                format!("0")
            }
        };

        if curr_port != 0.to_string() {
            let output = Arc::clone(&output);
            let mut output = output.lock().unwrap();
            output.push(curr_port);
        }
    });

    let final_result = output.lock().unwrap().join("\n");
    println!("{}", final_result);

}

// Asynchrone (problème ici sur les ports)
pub async fn port_async(nb1: u8, nb2: u8, nb3: u8, nb4: u8, port: Vec<u16>) {

    let start_port = port[0];
    let end_port = port[1];
    if start_port > end_port {
        panic!("Invalid port range");
    }
    let ipv4 = IpAddr::V4(Ipv4Addr::new(nb1, nb2, nb3, nb4));

    for port in start_port..=end_port {
        let ipv4 = ipv4.to_string();
        let ipv4 = ipv4.as_str();

        if test_connection(ipv4, port).await {
            println!("Port {} is open", port);
        }
    }
}

async fn test_connection(ip: &str, port: u16) -> bool {

    let addr: SocketAddr = format!("{}:{}", ip, port).parse().expect("Unable to convert");

    timeout(Duration::from_secs(5), tokio::net::TcpStream::connect(addr)).await.is_ok()
}

// Both
pub async fn parallel_port_scan(ipv4_address: &str, ports: Vec<u16>) {
    let start_port = ports[0];
    let end_port = ports[1];

    let mut output = Vec::new();

    let vec = (start_port..=end_port).collect::<Vec<u16>>();

    let futures = vec.clone().into_par_iter().map(|port| {
        test_connection(ipv4_address, port)
    }).collect::<Vec<_>>();

    for (i, future) in futures.into_iter().enumerate() {
        match future.await {
            true => {
                // println!("Port {} is open", vec[i]);
                let msg = format!("Port {} is open", vec[i]);
                output.push(msg);
            }
            false => {}
        }
    }

    let final_result = output.join("\n");
    println!("{}", final_result);
}


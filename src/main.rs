mod scan_ipv4;
mod scan_ipv6;
mod scan_port;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    // Port scanning
    ScanPort {
        #[clap(long_help = "Fist pair of bytes", required = true)]
        octet1: u8,

        #[clap(long_help = "Second pair of bytes", required = true)]
        octet2: u8,

        #[clap(long_help = "Third pair of bytes", required = true)]
        octet3: u8,

        #[clap(long_help = "Last pair of bytes", required = true)]
        octet4: u8,

        #[clap(short = 'p', long = "port", long_help = "Port range to scan", required = true, value_delimiter = '/')] 
        port: Vec<u16>,

        #[clap(short = 't', long = "type", long_help = "Parallelism or Asynchrone", required = true)]  
        type_scan: String,
    },

    // IPv4 scanning
    ScanIpv4 {
        #[clap(long_help = "Fist pair of bytes", required = true)]
        octet1: u8,

        #[clap(long_help = "Second pair of bytes", required = true)]
        octet2: u8,
    },

    // IPv6 scanning (incoming)
    ScanIpv6 {
        #[clap(long_help = "Fist pair of bytes", required = true)]
        octet1: u8,

        #[clap(long_help = "Second pair of bytes", required = true)]
        octet2: u8,
    },
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        // Scan for IPv4 addresses on the network with their hostname for a specific port
        Commands::ScanPort { octet1, octet2, octet3, octet4, port, type_scan } => {
            if type_scan.to_lowercase() == "async" {
                println!("Scanning ports {} to {} on {}.{}.{}.{} with asynchrone", port[0], port[1], octet1, octet2, octet3, octet4);
                scan_port::port_async(*octet1, *octet2, *octet3, *octet4, port.clone()).await;
                return Ok(());
            } else if type_scan.to_lowercase() == "paral" {
                println!("Scanning ports {} to {} on {}.{}.{}.{} with parallelism", port[0], port[1], octet1, octet2, octet3, octet4);
                scan_port::port_paral(*octet1, *octet2, *octet3, *octet4, port.clone());
                return Ok(());
            } else if type_scan.to_lowercase() == "both" {
                println!("Scanning ports {} to {} on {}.{}.{}.{} with both", port[0], port[1], octet1, octet2, octet3, octet4);
                let ip_str = format!("{}.{}.{}.{}", octet1, octet2, octet3, octet4);
                let ip_str = ip_str.as_str();
                scan_port::parallel_port_scan(ip_str, port.clone()).await;
                return Ok(()); 
            } else {
                println!("Invalid type of scan");
                return Ok(());
            }
        },

        // List all IPv4 addresses on the network with their hostname
        Commands::ScanIpv4 { octet1, octet2 } => {
            println!("Scanning local network with octet1={}, octet2={}", octet1, octet2);
            scan_ipv4::ipv4(*octet1, *octet2);
        },

        // List all IPv4/IPv6 addresses on the network with their hostname
        Commands::ScanIpv6 { octet1, octet2 } => {
            println!("Scanning local network with octet1={}, octet2={} for IPv6", octet1, octet2);
            scan_ipv6::ipv6(*octet1, *octet2);
        },
    }
    Ok(())
}
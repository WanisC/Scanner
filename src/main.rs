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

    ScanIpv4 {
        #[clap(long_help = "IPv4 address", required = true)]
        ipv4_adr: String,
    },

    ScanIpv6 {
        #[clap(long_help = "Fist pair of bytes", required = true)]
        octet1: u8,

        #[clap(long_help = "Second pair of bytes", required = true)]
        octet2: u8,
    },

    ScanNet {},
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ScanPort { octet1, octet2, octet3, octet4, port, type_scan } => {
            if type_scan.to_lowercase() == "async" {
                println!("Scanning ports {} to {} on {}.{}.{}.{} with asynchrone", port[0], port[1], octet1, octet2, octet3, octet4);
                scan_port::port_async(*octet1, *octet2, *octet3, *octet4, port.clone()).await;
                return Ok(());
            } else if type_scan.to_lowercase() == "paral" {
                println!("Scanning ports {} to {} on {}.{}.{}.{} with parallelism", port[0], port[1], octet1, octet2, octet3, octet4);
                scan_port::port_paral(*octet1, *octet2, *octet3, *octet4, port.clone()).await;
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

        Commands::ScanIpv4 { ipv4_adr } => {
            println!("[-] Local network scan of {:?}", ipv4_adr);
            let octets_result: Result<Vec<u8>, _> = ipv4_adr
                .split('.')
                .map(|s| s.parse::<u8>())
                .collect();
            let octets = match octets_result {
                Ok(octs) if (1..=4).contains(&octs.len()) => octs,
                _ => {
                    eprintln!("Invalid IP address, must be 1 to 4 bytes between 0 and 255");
                    std::process::exit(1);
                }
            };
            scan_ipv4::ipv4(&octets);
            println!("[-] Results are stored in a JSON file");
        },

        Commands::ScanIpv6 { octet1, octet2 } => {
            println!("Local network scan of... with octet1={}, octet2={} for IPv6", octet1, octet2);
            scan_ipv6::ipv6(*octet1, *octet2);
        },

        Commands::ScanNet {} => {},
    }
    Ok(())
}
mod scan_ipv4;
mod scan_ipv6;
mod scan_port;
mod full_scan;

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

        #[clap(short = 'p', long_help = "Port to scan", required = true)] 
        port: u16,       
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

    // Full network scanning (incoming)
    FullScan {},
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::ScanPort { octet1, octet2, port } => {
            println!("Scanning with octet1={}, octet2={}, port={}", octet1, octet2, port);
            scan_port::port(*octet1, *octet2, *port);
        },

        Commands::ScanIpv4 { octet1, octet2 } => {
            println!("Scanning local network with octet1={}, octet2={}", octet1, octet2);
            scan_ipv4::ipv4(*octet1, *octet2);
        },

        Commands::ScanIpv6 { octet1, octet2 } => {
            println!("Scanning local network with octet1={}, octet2={} for IPv6", octet1, octet2);
            scan_ipv6::ipv6(*octet1, *octet2);
        },

        Commands::FullScan {} => {
            println!("Scanning local network");
            full_scan::full();
        },
    }
    Ok(())
}
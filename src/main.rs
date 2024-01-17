mod scan_ipv4;

use clap::Parser;

#[derive(Parser)]
#[clap(author, version, about)]
struct Opts {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Parser)]
enum Commands {
    ScanIpv4 {
        #[clap(long_help = "Vector of bytes")]
        bytes: Vec<u8>,        
    }
}

fn main() -> std::io::Result<()> {
    let opts = Opts::parse();

    match &opts.command {
        Commands::ScanIpv4 { bytes } => {
            // Check that the vector has exactly two elements
            if bytes.len() != 2 {
                println!("The vector must have exactly two elements");
                return Ok(());
            }
            scan_ipv4::scan(bytes[0], bytes[1]);
        }
    }

    Ok(())
}
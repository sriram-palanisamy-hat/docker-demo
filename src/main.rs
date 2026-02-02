mod container;
mod network;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Network {
        #[command(subcommand)]
        command: NetworkCmd,
    },
    Run {
        #[arg(long)]
        net: String,
        #[arg(long)]
        ip: String,
    },
}

#[derive(Subcommand)]
enum NetworkCmd {
    Create {
        #[arg(long)]
        name: String,
        #[arg(long)]
        subnet: String,
        #[arg(long)]
        gateway: String,
    },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Network { command } => match command {
            NetworkCmd::Create { name, subnet, gateway } => {
                network::create_bridge(&name, &subnet, &gateway)?;
            }
        },
        Commands::Run { net, ip } => {
            container::run_container(&net, &ip)?;
        }
    }

    Ok(())
}

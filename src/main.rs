use clap::Parser;
use std::process;
mod client;
mod server;

#[derive(Parser)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}
#[derive(Parser)]
enum Command {
    Server { port: u16 },
    Client { server_address: String },
}

#[tokio::main]
async fn main() {
    let args = Cli::parse();

    match args.command {
        Command::Server { port } => {
            if let Err(e) = server::start_server(port).await {
                eprintln!("Error starting server: {}", e);
                process::exit(1);
            }
        }
        Command::Client { server_address } => {
            if let Err(e) = client::start_client(&server_address).await {
                eprintln!("Error starting client: {}", e);
                process::exit(1);
            }
        }
    }
}

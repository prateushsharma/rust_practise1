mod server;
mod client;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: cargo run -- <server|client>");
        std::process::exit(1);
    }

    match args[1].as_str() {
        "server" => server::start_server().await,
        "client" => client::start_client().await,
        _ => {
            eprintln!("Invalid argument: use 'server' or 'client'");
            std::process::exit(1);
        }
    }
}

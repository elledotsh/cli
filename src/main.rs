use std::error::Error;

use ::clap::{Args, Parser, Subcommand};

mod elle;

#[derive(Parser)]
#[clap(name = "elle", version = env!("CARGO_PKG_VERSION"))]
#[command(author, version, about, long_about = None)]
#[command(about = "elle: a tiny metaframework for Laravel")]

pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args)]
struct MakeArgs {
    name: String,
    kind: String,
}
#[derive(Args)]
struct NewArgs {
    name: String,
}
#[derive(Args)]
struct ServeArgs {
    host: Option<String>,
    port: Option<u16>,
}

#[derive(Subcommand)]
enum Commands {
    Make(MakeArgs),
    New(NewArgs),
    Serve(ServeArgs),
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    match args.command {
        Commands::Make(args) => elle::make(&args.name, &args.kind),
        Commands::New(args) => elle::new(&args.name),
        Commands::Serve(args) => elle::serve(
            args.host.unwrap_or("localhost".to_string()),
            args.port.unwrap_or(8000),
        ),
    }
    Ok(())
}

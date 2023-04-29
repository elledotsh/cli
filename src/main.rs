use std::error::Error;

use ::clap::{Args, Parser, Subcommand};

mod elle;

#[derive(Parser)]
#[command(name = "elle", version = env!("CARGO_PKG_VERSION"))]
#[command(about = "elle: a tiny metaframework for Laravel", long_about = None)]

pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Args)]
struct MakeMigrationCreateArgs {
    name: String,
}
#[derive(Args)]
struct MakeModelArgs {
    name: String,
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
    #[command(
        name = "make:migration:create",
        about = "generates a new creation migration"
    )]
    MakeMigrationCreate(MakeMigrationCreateArgs),
    #[command(name = "make:model", about = "generates a new model")]
    MakeModel(MakeModelArgs),
    New(NewArgs),
    Serve(ServeArgs),
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Cli::parse();
    match args.command {
        Commands::MakeMigrationCreate(args) => elle::make_migration_create(&args.name),
        Commands::MakeModel(args) => elle::make_model(&args.name),
        Commands::New(args) => elle::new(&args.name),
        Commands::Serve(args) => elle::serve(
            args.host.unwrap_or("localhost".to_string()),
            args.port.unwrap_or(8000),
        ),
    }
    Ok(())
}

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

mod d01;
mod d03;

#[derive(Subcommand)]
#[command(disable_help_subcommand(true))]
enum Commands {
    D01(d01::Main),
    D03(d03::Main),
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Commands::D01(v) => v.run(),
        Commands::D03(v) => v.run(),
    }
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}

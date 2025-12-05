use clap::{Parser, Subcommand};

/// Advent of Code 2025
#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

macro_rules! solutions {
    ($($M:ident($m:ident :: $entry:ident)),+ $(,)?) => {
        $(mod $m;)+

        #[derive(Subcommand)]
        #[command(disable_help_subcommand(true))]
        enum Commands {
            $($M($m::$entry),)+
        }

        fn main() {
            match Cli::parse().command {
                $(Commands::$M(v) => v.run(),)+
            }
        }
    }
}

pub mod grid;

solutions! {
    D01(d01::Main),
    D02(d02::Main),
    D03(d03::Main),
    D04(d04::Main),
    D05(d05::Main),
    Hello(hello::Main),
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}

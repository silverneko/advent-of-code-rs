use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

macro_rules! gen_main {
    ($($M:ident :: $m:ident),+) => {
        $(mod $m;)+

        #[derive(Subcommand)]
        #[command(disable_help_subcommand(true))]
        enum Commands {
            $($M($m::Main),)+
        }

        fn main() {
            match Cli::parse().command {
                $(Commands::$M(v) => v.run(),)+
            }
        }
    }
}

gen_main! {D01::d01, D02::d02, D03::d03}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert();
}

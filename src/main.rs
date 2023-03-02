mod commands;
use clap::{command, Parser};
use commands::Commands;

#[derive(Parser, Debug)]
#[command(
    bin_name = "pd6",
    about = "A collection of tools to support the creation of the powerd6 system."
)]
struct CliArguments {
    #[arg(
        short = 'p',
        long = "preview",
        help = "This option will describe the steps the tool will take, without actually taking them.",
        default_value = None,
        global = true,
    )]
    preview: Option<bool>,
    #[command(subcommand)]
    command: Commands,
}
pub fn main() {
    let args = CliArguments::parse();

    match args.command {
        Commands::Build(_) => todo!(),
    }
}

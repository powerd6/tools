use tools::errors::ErrorCodes;

mod cli;

pub fn main() -> Result<(), ErrorCodes> {
    cli::run_cli()
}

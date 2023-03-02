use clap::Subcommand;

pub mod build;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Build(build::Command),
}
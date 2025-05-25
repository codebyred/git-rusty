pub mod commands;
use clap::Parser;
use anyhow;
use commands::Args;

fn main() -> anyhow::Result<()> {

    Args::parse().command.run()?;

    Ok(())

}

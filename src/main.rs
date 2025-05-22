#[allow(unused)]
pub mod run;
pub mod config;
pub mod init;
pub mod object;

use config::Args;
use run::run;
use clap::Parser;
use anyhow;

fn main() -> anyhow::Result<()> {

    let args = Args::parse();

    run(args)?;

    Ok(())

}

use std::{env,  path::PathBuf};
use anyhow::Context;

use crate::object;

pub fn run() -> anyhow::Result<()> {
    
    let path = PathBuf::from(env::current_dir().context("getting the curr dir from env")?);
    let hash = object::write_tree(&path)?;

    println!("{}", hex::encode(hash));

    Ok(())
}

/*
    iterate over file & dir in root path
    if file call write obj
    if dir call write tree recursively

*/


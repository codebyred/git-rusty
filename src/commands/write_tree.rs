use std:: path::PathBuf;
use crate::object;

pub fn run() -> anyhow::Result<()> {
    
    let path = PathBuf::from(".");
    object::write_tree(&path)?;

    Ok(())
}

/*
    iterate over file & dir in root path
    if file call write obj
    if dir call write tree recursively

*/


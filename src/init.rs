use std::fs;
use anyhow;

pub fn create_git_dir() -> anyhow::Result<()> {

    fs::create_dir(".git")?;
    fs::create_dir(".git/objects")?;
    fs::create_dir(".git/refs")?;
    fs::write(".git/HEAD", "ref: refs/heads/main\n")?;
    println!("Initialized git directory");    
    Ok(())

}
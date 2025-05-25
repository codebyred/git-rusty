pub mod cat_file;
pub mod hash_object;
pub mod init;
pub mod ls_tree;
pub mod write_tree;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,
        hash: String,
    },
    HashObject {
        #[clap(short = 'w')]
        write: bool,
        file: PathBuf,
    },
    LsTree {
        #[clap(long)]
        name_only: bool,
        hash: String,
    },
    WriteTree 
}

impl Command {
    pub fn run(self) -> anyhow::Result<()> {
        match self {
            Command::Init => {
                init::create_git_dir()?;
            }
            Command::CatFile {pretty_print,hash} => {
                cat_file::run(&hash)?;
            }
            Command::HashObject { write, file } => {
                hash_object::run(&file)?;
            }
            Command::LsTree {
                name_only,
                hash,
            } => {
                ls_tree::run(&hash, name_only)?;
            }
            Command::WriteTree => {

            }
        };
        Ok(())
    }
}

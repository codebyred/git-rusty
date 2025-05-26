pub mod cat_file;
pub mod hash_object;
pub mod init;
pub mod ls_tree;
pub mod write_tree;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use cat_file::CatFile;
use hash_object::HashObject;
use ls_tree::LsTree;

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
                CatFile::new().with_pretty_print(pretty_print).run(&hash)?;
            }
            Command::HashObject { write, file } => {
                HashObject::new().with_write(write).run(&file)?;
            }
            Command::LsTree {
                name_only,
                hash,
            } => {
                LsTree::new().with_name_only(name_only).run(&hash)?;
            }
            Command::WriteTree => {
                write_tree::run()?;
            }
        };
        Ok(())
    }
}

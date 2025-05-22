
use crate::config::*;
use crate::object::{blob, tree};
use crate::init;


pub fn run(args: Args)-> anyhow::Result<()> {

    match args.command {

        Command::Init => {
            init::create_git_dir()?;
        }
        Command::CatFile { pretty_print,object_hash} => {
            blob::read_object(object_hash)?;
        },
        Command::HashObject { write, file } => {
            blob::create_object(file)?;
        },
        Command::LsTree { name_only, tree_object } => {
            tree::read_object(name_only, tree_object)?;
        }

    };

    Ok(())
}




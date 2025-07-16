pub mod cat_file;
pub mod hash_object;
pub mod init;
pub mod ls_tree;
pub mod write_tree;
pub mod commit_tree;
pub mod clone;
use std::path::PathBuf;
use clap::{Parser, Subcommand};
use cat_file::CatFile;
use hash_object::HashObject;
use ls_tree::LsTree;
#[derive(Parser, Debug)]
#[command(version, about = "A clone of Git implemented in Rust", long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Initialize a new, empty Git repository
    Init,

    /// Provide content or type and size information for repository objects
    CatFile {
        /// Pretty-print the contents of the object
        #[clap(short = 'p', help = "Pretty-print the contents of the object")]
        pretty_print: bool,

        /// The hash of the object to display
        hash: String,
    },

    /// Compute object ID and optionally create a blob from a file
    HashObject {
        /// Actually write the object into the database
        #[clap(short = 'w', help = "Write the object into the database")]
        write: bool,

        /// The file to hash
        file: PathBuf,
    },

    /// List the contents of a tree object
    LsTree {
        /// List only names of files (no extra metadata)
        #[clap(long, help = "Show only filenames")]
        name_only: bool,

        /// The hash of the tree object
        hash: String,
    },

    /// Write a tree object from the current index
    WriteTree,

    /// Create a commit object
    CommitTree {
        /// The hash of the tree object
        tree_hash: String,

        /// The parent commit hash
        #[clap(short = 'p', help = "Parent commit hash")]
        parent_hash: Option<String>,

        /// The commit message
        #[clap(short = 'm', help = "Commit message")]
        message: String,
    },

    /// Clone a repository into a new directory
    Clone {
        /// The URL of the repository to clone
        url: String,
    },
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
            },
            Command::CommitTree { message, tree_hash, parent_hash } => {
                commit_tree::run(&message, &tree_hash, parent_hash.as_deref())?;
            },
            Command::Clone { url } => {
                clone::run(&url)?;
            }
        };
        Ok(())
    }
}

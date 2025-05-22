pub mod tree;
pub mod blob;

pub enum Kind {
    Blob,
    Tree,
    Commit
}
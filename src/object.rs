use anyhow::Context;
use flate2::{Compression, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
};

/// Writes a blob object and returns its raw SHA-1 hash (20 bytes)
pub fn write_blob(file: &Path) -> anyhow::Result<[u8; 20]> {
    let content = fs::read(file).context("reading file for blob")?;
    let header = format!("blob {}\0", content.len());
    
    let mut blob = header.into_bytes();
    blob.extend(&content);
    
    let hash = Sha1::digest(&blob);
    write_git_object(&hash, &blob)?;
    
    Ok(hash.into())
}

/// Writes a tree object and returns its raw SHA-1 hash (20 bytes)
pub fn write_tree(path: &Path) -> anyhow::Result<[u8; 20]> {
    let mut entries = BTreeMap::new(); // BTreeMap to auto-sort entries by name

    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let file_name = entry.file_name();
        if file_name == ".git" {
            continue;
        }

        let file_name_bytes = file_name.to_string_lossy().as_bytes().to_vec();
        let entry_path = entry.path();

        let mut entry_data = Vec::new();
        if entry_path.is_file() {
            let hash = write_blob(&entry_path)?;
            entry_data.extend(b"100644 ");
            entry_data.extend(&file_name_bytes);
            entry_data.push(0);
            entry_data.extend(&hash);
        } else if entry_path.is_dir() {
            let hash = write_tree(&entry_path)?;
            entry_data.extend(b"040000 ");
            entry_data.extend(&file_name_bytes);
            entry_data.push(0);
            entry_data.extend(&hash);
        }

        entries.insert(file_name_bytes, entry_data);
    }

    let mut tree_body = Vec::new();
    for (_name, entry_data) in entries {
        tree_body.extend(entry_data);
    }

    let header = format!("tree {}\0", tree_body.len());
    let mut tree = header.into_bytes();
    tree.extend(tree_body);

    let hash = Sha1::digest(&tree);
    write_git_object(&hash, &tree)?;

    Ok(hash.into())
}


/// Writes a git object file from the given hash and content
fn write_git_object(hash: &[u8], content: &[u8]) -> anyhow::Result<()> {
    let hex_hash = hex::encode(hash);
    let folder = format!(".git/objects/{}", &hex_hash[..2]);
    let file = format!("{}/{}", folder, &hex_hash[2..]);

    fs::create_dir_all(&folder).context("creating .git object folder")?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(content).context("compressing git object")?;
    let compressed = encoder.finish().context("finalizing compression")?;

    fs::write(&file, &compressed).context("writing git object")?;
    Ok(())
}

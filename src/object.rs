use anyhow::Context;
use flate2::{Compression, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::{fs, io::Write, path::PathBuf};

use crate::error::GitObjectError;

pub fn write_blob(file: &PathBuf) -> anyhow::Result<String> {
    let file_content = fs::read(file).context("opening input file")?;
    let file_content_size = file_content.len();
    let header = format!("blob {}\0", file_content_size);
    let mut blob_content = header.into_bytes();
    blob_content.extend(file_content);
    let hash_hex = hex::encode(Sha1::digest(&blob_content));
    write_git_object(&hash_hex, &blob_content)?;
    Ok(hash_hex)
}

pub fn write_tree(path: &PathBuf) -> anyhow::Result<String>{
    let mut entries:Vec<u8> = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let entry_path = entry.path();
        let file_name =  entry.file_name();
        if file_name == ".git" {continue;}
        let file_name_bytes = file_name
            .to_string_lossy()
            .as_bytes()
            .to_vec();

        if entry_path.is_file() {
            let blob_hash = write_blob(&entry_path)?;
            entries.extend(b"100644 ");
            entries.extend(file_name_bytes);
            entries.push(0);
            entries.extend(hex::decode(&blob_hash)?);
        }else if entry_path.is_dir() {
            let tree_hex = write_tree(&entry_path)?;
            entries.extend(b"40000");
            entries.extend(file_name_bytes);
            entries.push(0);
            entries.extend(hex::decode(&tree_hex)?);
        }
    }
    let header = format!("tree {}\0", entries.len());
    let mut tree_content = header.into_bytes();
    tree_content.extend(&entries);
    let hash_hex = hex::encode(Sha1::digest(&tree_content));
    write_git_object(&hash_hex, &tree_content)?;
    Ok(hash_hex)
}


fn write_git_object(hash_hex: &str, content:&[u8]) -> anyhow::Result<()> {
    if hash_hex.len() > 20 {
        return Err(GitObjectError::InvalidHashLength(hash_hex.len()).into());
    }
    let folder = format!(".git/objects/{}", &hash_hex[..2]);
    let file = format!("{}/{}", folder, &hash_hex[2..]);
    fs::create_dir_all(&folder).context("creating directory in .git/objects")?;
    let mut z = ZlibEncoder::new(Vec::new(), Compression::default());
    z.write_all(&content).context("compressing content")?;
    let compressed = z.finish().context("finilizing compression")?;
    fs::write(&file, &compressed).context("writing compressed content to file")?;
    Ok(())
}


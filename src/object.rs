use anyhow::{Context, Ok};
use flate2::{Compression, write::ZlibEncoder};
use sha1::{Digest, Sha1};
use std::io::Write;
use std::{collections::BTreeMap, fs, path::Path};

/// Writes a blob object and returns its raw SHA-1 hash (20 bytes)
pub fn create_blob(file: &Path) -> anyhow::Result<[u8; 20]> {
    let blob_content = fs::read(file).context("reading file for blob")?;
    let header = format!("blob {}\0", blob_content.len());

    let hash = write_git_object(&header, &blob_content[..])?;

    Ok(hash)
}

/// Writes a tree object and returns its raw SHA-1 hash (20 bytes)
pub fn create_tree(path: &Path) -> anyhow::Result<[u8; 20]> {
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
            let hash = create_blob(&entry_path)?;
            entry_data.extend(b"100644 ");
            entry_data.extend(&file_name_bytes);
            entry_data.push(0);
            entry_data.extend(&hash);
        } else if entry_path.is_dir() {
            let hash = create_tree(&entry_path)?;
            entry_data.extend(b"040000 ");
            entry_data.extend(&file_name_bytes);
            entry_data.push(0);
            entry_data.extend(&hash);
        }

        entries.insert(file_name_bytes, entry_data);
    }

    let mut tree_content = Vec::new();

    for (_name, entry_data) in entries {
        tree_content.extend(entry_data);
    }

    let header = format!("tree {}\0", tree_content.len());

    let hash = write_git_object(&header, &tree_content[..])?;

    Ok(hash)
}

pub fn create_commit(tree_hash: &str, parent_hash: Option<&str>, message: &str) -> anyhow::Result<[u8; 20]> {
    use std::fmt::Write;

    let mut commit_content = String::new();

    writeln!(&mut commit_content, "tree {tree_hash}")
        .with_context(|| format!("writing tree hash as commit content failed"))?;

    if let Some(parent_hash) = parent_hash {
        writeln!(&mut commit_content, "parent {parent_hash}")
        .with_context(|| format!("writing parent hash as commit content failed"))?;
    }

    writeln!(&mut commit_content, "author   codebyred <nazmulhaqueredoan@gmail.com> 1748355565 +0600")?;
    writeln!(&mut commit_content, "committer codebyred <nazmulhaqueredoan@gmail.com> 1748355565 +0600")?;
    writeln!(&mut commit_content, "")?;
    writeln!(&mut commit_content, "{message}")?;

    let header = format!("commit {}\0", commit_content.len());

    let hash = write_git_object(&header, &commit_content.as_bytes())?;

    Ok(hash)
}

/// Writes a git object file from the given hash and content
fn write_git_object(header: &str, content: &[u8]) -> anyhow::Result<[u8; 20]> {
    let mut object = header.as_bytes().to_vec();
    object.extend(content);

    let hash = Sha1::digest(&object);
    let hex_hash = hex::encode(hash);

    let folder = format!(".git/objects/{}", &hex_hash[..2]);
    let file = format!("{}/{}", folder, &hex_hash[2..]);

    fs::create_dir_all(&folder).context("creating .git object folder")?;

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder
        .write_all(&object[..])
        .context("compressing git object")?;
    let compressed = encoder.finish().context("finalizing compression")?;

    fs::write(&file, &compressed).context("writing git object")?;

    Ok(hash.into())
}

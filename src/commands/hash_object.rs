use std::{fs, io::Write, path::PathBuf};
use anyhow::Context;
use flate2::{Compression, write::ZlibEncoder};
use sha1::{Digest, Sha1};

pub fn run(file: &PathBuf) -> anyhow::Result<()> {
    // read file contents
    let file = fs::read(file).context("opening input file")?;
    let size = file.len();

    // format the file in this way: blob <size>\0content
    let mut blob_vec: Vec<u8> = Vec::new();
    let header = format!("blob {}\0", size);
    blob_vec.extend(header.as_bytes());
    blob_vec.extend(file);

    // generate the sha-1 hash using the formatted content
    let mut hasher = Sha1::new();
    hasher.update(&blob_vec);
    let result = hasher.finalize();
    let hash = hex::encode(result);

    // compress the formatted file
    let mut z = ZlibEncoder::new(Vec::new(), Compression::default());
    z.write_all(&blob_vec[..])
        .context("compressing file content")?;
    let compressed_bytes = z.finish().unwrap();

    // create a dir with 1st letter of hash and a file with rest of the hash
    let folder_path = format!(".git/objects/{}", &hash[..2]);
    fs::create_dir(&folder_path).context("creatting directory in .git/objects")?;
    let file_path = format!("{}/{}", folder_path, &hash[2..]);

    // store the compressed data in the file
    fs::write(file_path, compressed_bytes).context("writing to input file")?;
    println!("{hash}");

    Ok(())
}

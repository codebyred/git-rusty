use std::{ffi::CStr, fs::{self, File}, io::{BufRead, BufReader, Read, Write}, path::PathBuf};
use anyhow::Context;
use flate2::{read::ZlibDecoder, Compression};
use flate2::write::ZlibEncoder;
use sha1::{Sha1, Digest};
use std::io;
use crate::object::Kind;


pub fn read_object(object_hash: String) -> anyhow::Result<()> {

    /*
        use the first two letter of hash for dir and rest for filename in .git/objects
        decompress the file content, decompressed file format: blob <size>\0content
        separate content from header and print the content
        
    */

    let file = File::open(format!(
        ".git/objects/{}/{}",
        &object_hash[..2],
        &object_hash[2..]
    )).context("open in .git/objects")?;
    let decoder: ZlibDecoder<File> = ZlibDecoder::new(file);
    let mut buf_reader = BufReader::new(decoder);
    let mut buf = Vec::new();
    buf_reader.read_until(0, &mut buf).context("reading \0 in header")?;
    let header = CStr::from_bytes_with_nul(&buf)
        .context("converting u8 to c string")?;
    let header = header.to_str().context("converting header from &CStr to &str")?;
    let Some((kind, size)) = header.split_once(" ") else {
        anyhow::bail!("the object file is not valid format 'blob <size>/0content");
    };
    let kind = match kind {
        "blob"=> Kind::Blob,
        _ => anyhow::bail!("undefined kind in object header"),
    };
    let size = size.parse::<usize>().context("header has invalid size")?;
    buf.clear();
    buf.resize(size, 0);
    buf_reader.read_exact(&mut buf).context("reading contents of object file")?;
    let n = buf_reader.read(&mut [0]).context("validating EOF ")?;
    anyhow::ensure!(n == 0, "object file has {n} trailing bytes");
    let mut stdout = io::stdout().lock();
    match kind {
        Kind::Blob => stdout.write_all(&mut buf).context("printing object file content")?,
        _ => anyhow::bail!("I want to break free")
    }

    Ok(())

}

pub fn create_object(file: PathBuf) -> anyhow::Result<()> {

    /*
        Algorithm:
        read file contents
        format the file in this way: blob <size>\0content
        compress the formatted file
        generate the sha-1 hash using the formatted content
        create a dir with 1st letter of hash and a file with rest of the hash
        store the compressed data in the file

     */

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
    let mut z = ZlibEncoder::new(Vec::new(),Compression::default());
    z.write_all(&blob_vec[..]).context("compressing file content")?;
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



use std::{ffi::CStr, fs::File, io::{BufRead, BufReader, Read}};

use anyhow::Context;
use flate2::read::ZlibDecoder;

use super::Kind;



pub fn read_object(name_only: bool, tree_object: String) -> anyhow::Result<()> {
    /*
        find the object file in .git/objects/obj[..2]/obj[2..]
        decompress the file
        check the header, header format: tree <size>\0

    */

    let object_path = format!(".git/objects/{}/{}",&tree_object[..2], &tree_object[2..]);
    let file = File::open(object_path).context("Reading object files in .git/objects/")?;
    let file_decoder = ZlibDecoder::new(file);
    let mut buf_reader = BufReader::new(file_decoder);
    let mut buf = Vec::new();
    buf_reader.read_until(0,&mut buf).context("reading file data as utf8")?;
    let header = CStr::from_bytes_with_nul(&buf[..]).context("coverting u8 to c string")?;
    let header = header.to_str().context("converting c string to str")?;

    let Some((kind, _)) = header.split_once(" ") else {
        anyhow::bail!("the object file is not valid format 'blob <size>/0content");
    };
    let _ = match kind {
        "tree"=> Kind::Tree,
        _ => anyhow::bail!("undefined kind in object header"),
    };

    loop {
        buf.clear();
        let n = buf_reader.read_until(0, &mut buf).context("reading file data as utf8")?;

        if n == 0 {
            break;
        }

        let entry = CStr::from_bytes_with_nul(&buf[..]).context("coverting u8 to c string")?;
        let entry = entry.to_str().context("converting c string to str")?;
        let mut sha = [0u8;20];
        buf_reader.read_exact(&mut sha).context("reading sha byte in tree object file")?;
        let sha = hex::encode(sha);

        if name_only {
            let wiw: Vec<&str> = entry.split(" ").collect();
            println!("{}", wiw[1]);
        }else {
            println!("{} {}", entry, sha);
        }
        
    }

    Ok(())
}
/*
tree object format:
  tree <size>\0 <-- I have read this using bufReader. 
  <mode> <name>\0<20_byte_sha> <-- buf pointer is probably here now, upto '\0' characters are valid utf8
                                   next are binary format not hexadecimal sha                 
  <mode> <name>\0<20_byte_sha>
    <-- then buf pointer is here, read_until will return 0; thus the operation will end
*/
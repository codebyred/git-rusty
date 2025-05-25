use anyhow::Context;
use flate2::read::ZlibDecoder;
use std::{
    ffi::CStr,
    fs::File,
    io::{self, BufRead, BufReader, Read, Write},
};

pub fn run(hash: &str) -> anyhow::Result<()> {
    let file = File::open(format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))
        .with_context(|| "open in .git/objects")?;

    let decoder: ZlibDecoder<File> = ZlibDecoder::new(file);
    let mut buf_reader = BufReader::new(decoder);
    let mut buf = Vec::new();

    buf_reader
        .read_until(0, &mut buf)
        .context("reading \0 in header")?;
    let header = CStr::from_bytes_with_nul(&buf).context("converting u8 to c string")?;
    let header = header
        .to_str()
        .context("converting header from &CStr to &str")?;

    let Some((_, size)) = header.split_once(" ") else {
        anyhow::bail!("the object file is not valid format 'blob <size>/0content");
    };

    let size = size.parse::<usize>().context("header has invalid size")?;
    buf.clear();
    buf.resize(size, 0);
    buf_reader
        .read_exact(&mut buf)
        .context("reading contents of object file")?;
    let n = buf_reader.read(&mut [0]).context("validating EOF ")?;
    anyhow::ensure!(n == 0, "object file has {n} trailing bytes");
    let mut stdout = io::stdout().lock();
    stdout
        .write_all(&mut buf)
        .context("printing object file content")?;

    Ok(())
}

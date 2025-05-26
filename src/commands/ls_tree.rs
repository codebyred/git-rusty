use anyhow::{Context, Ok};
use flate2::read::ZlibDecoder;
use std::{
    ffi::CStr,
    fs::File,
    io::{BufRead, BufReader, Read},
};

#[derive(Debug, Default)]
pub struct LsTree {
    name_only: bool,
}

impl LsTree {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_name_only(mut self, status: bool) -> Self {
        self.name_only = status;
        self
    }

    pub fn run(self, hash: &str) -> anyhow::Result<()> {
        let file = File::open(format!(".git/objects/{}/{}", &hash[..2], &hash[2..]))
            .context("open in .git/objects")?;

        let decoder: ZlibDecoder<File> = ZlibDecoder::new(file);
        let mut buf_reader = BufReader::new(decoder);
        let mut buf = Vec::new();

        buf_reader
            .read_until(0, &mut buf)
            .context("reading \0 in header")?;

        loop {
            buf.clear();
            let n = buf_reader
                .read_until(0, &mut buf)
                .context("reading file data as utf8")?;

            if n == 0 {
                break;
            }

            let entry = CStr::from_bytes_with_nul(&buf[..]).context("coverting u8 to c string")?;
            let entry = entry.to_str().context("converting c string to str")?;
            let mut sha = [0u8; 20];
            buf_reader
                .read_exact(&mut sha)
                .context("reading sha byte in tree object file")?;
            let sha = hex::encode(sha);

            if self.name_only {
                let wiw: Vec<&str> = entry.split(" ").collect();
                println!("{}", wiw[1]);
            } else {
                println!("{} {}", entry, sha);
            }
        }

        Ok(())
    }
}

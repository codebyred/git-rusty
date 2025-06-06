use std::path::PathBuf;
use crate::object;

#[derive(Debug, Default)]
pub struct HashObject {
    write: bool,
}

impl HashObject {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_write(mut self, status: bool) -> Self {
        self.write = status;
        self
    }

    pub fn run(self, file: &PathBuf) -> anyhow::Result<()> {

        let hash = object::create_blob(file)?;
        println!("{}", hex::encode(hash));

        Ok(())
    }
}

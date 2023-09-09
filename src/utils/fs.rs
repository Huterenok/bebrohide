use anyhow::{bail, Context, Result};

use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;

pub fn read_file(path: &str) -> Result<Vec<u8>> {
    fs::read(path).context("Something went wrong while reading file")
}

pub fn write_to_file(path: &str, data: &[u8]) -> Result<()> {
    return OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)?
        .write_all(data)
        .context("Error while writing to file");
}

pub fn rewrite_file(path: &str, data: &[u8]) -> Result<()> {
    return OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path)?
        .write_all(data)
        .context("Error while writing to file");
}

pub fn retrieve_filename(path: &str) -> Result<String> {
    match Path::new(path).file_name() {
        Some(filename) => Ok(filename.to_str().unwrap().to_owned()),
        None => bail!("Cannot find file with {} path", path),
    }
}

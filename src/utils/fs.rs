use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

use std::error::Error;

pub fn read_file(path: &str) -> Result<Vec<u8>, Box<dyn Error>> {
    match OpenOptions::new().read(true).open(path) {
        Ok(file) => {
            let mut reader = BufReader::new(file);
            let mut buffer = Vec::new();
            if reader.read_to_end(&mut buffer).is_err() {
                return Err("Something went wrong while reading file".into());
            };

            Ok(buffer)
        }
        Err(_) => Err(format!("File with {} path was not found", path).into()),
    }
}

pub fn write_to_file(path: &str, data: &[u8]) -> Result<(), Box<dyn Error>> {
    match OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(path)
    {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            match writer.write_all(data) {
                Ok(_) => (),
                Err(_) => return Err("Error while writing to file".into()),
            };
            Ok(())
        }
        Err(_) => Err(format!("File with {} path was not found", path).into()),
    }
}

pub fn rewrite_file(path: &str, data: &[u8]) -> Result<(), Box<dyn Error>> {
    match OpenOptions::new().write(true).truncate(true).open(path) {
        Ok(file) => {
            let mut writer = BufWriter::new(file);
            match writer.write_all(data) {
                Ok(_) => (),
                Err(_) => return Err("Error while writing to file".into()),
            };
            Ok(())
        }
        Err(_) => Err("File cannot be rewritten".into()),
    }
}

pub fn retrieve_filename(path: &str) -> Result<String, Box<dyn Error>> {
    match Path::new(path).file_name() {
        Some(filename) => Ok(filename.to_str().unwrap().to_owned()),
        None => Err(format!("Cannot find file with {} path", path).into()),
    }
}

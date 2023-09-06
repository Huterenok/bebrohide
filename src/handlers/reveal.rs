//cargo run reveal -r assets/bebra4.txt -d assets2
//cargo run reveal -r assets/test.exe -d assets2 -c
use std::error::Error;

use clap::ArgMatches;

use crate::mask::{HiddenMask, VisibleMask, VISIBLE_MASK_SIZE};

use crate::utils::codec::decode;
use crate::utils::fs::{read_file, rewrite_file, write_to_file};
use crate::utils::hash::hash;
use crate::utils::io::read_password;
use crate::utils::serialize::deserialize;

#[derive(Debug)]
struct RevealArgs {
    destination: String,
    hidden: String,
    clear: bool,
}

pub fn run_reveal_command(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let args: RevealArgs = matches.clone().into();

    let data: &[u8] = &read_file(&args.hidden)?;
    let before_visible_length = match data.len().checked_sub(VISIBLE_MASK_SIZE) {
        Some(length) => length,
        None => return Err("File doesn't contain hidden one".into()),
    };
    let visible_mask: VisibleMask = deserialize(&data[before_visible_length..])?;
    match visible_mask.verify() {
        true => (),
        false => return Err("File doesn't contain hidden one".into()),
    };

    let pwd = read_password()?;
    let pwd_hash: &[u8] = &hash(pwd.as_bytes())?;

    let decoded: &[u8] = &decode(
        pwd_hash,
        &visible_mask.nonce,
        &data[data.len() - visible_mask.encoded_hidden_length as usize - VISIBLE_MASK_SIZE
            ..data.len() - VISIBLE_MASK_SIZE],
    )?;

    //TODO: -8
    let hidden_mask: HiddenMask =
        deserialize(&decoded[decoded.len() - visible_mask.hidden_mask_length as usize - 8..])?;
    //TODO: -8
    let hidden_file: &[u8] =
        &decoded[..decoded.len() - visible_mask.hidden_mask_length as usize - 8];

    let path = format!("{}/{}", args.destination, hidden_mask.name);
    write_to_file(&path, hidden_file)?;

    if args.clear {
        rewrite_file(
            &args.hidden,
            //TODO: -8
            &data[..data.len() - decoded.len() - VISIBLE_MASK_SIZE - 16],
        )?;
    }

    Ok(())
}

impl From<ArgMatches> for RevealArgs {
    fn from(matches: ArgMatches) -> Self {
        let destination = matches
            .get_one::<String>("DESTINATION_FOLDER")
            .unwrap()
            .into();
        let hidden = matches.get_one::<String>("HIDDEN_FILE").unwrap().into();
        let clear = matches.get_one::<bool>("CLEAR").cloned().unwrap();

        RevealArgs {
            destination,
            hidden,
            clear,
        }
    }
}

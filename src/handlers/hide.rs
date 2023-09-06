// cargo run hide -s assets/bebra1.txt -r assets/temp.exe -i assets/bebra5.txt
// cargo run hide -s assets/temp.exe -r assets/bebra.txt -i assets/test.exe
// cargo run hide -s assets/temp.exe -r assets/bebra.txt
use clap::ArgMatches;
use std::error::Error;

use crate::mask::{HiddenMask, VisibleMask};

use crate::utils::codec::encode;
use crate::utils::fs::{read_file, write_to_file, retrieve_filename};
use crate::utils::hash::hash;
use crate::utils::io::read_password;
use crate::utils::serialize::serialize;

#[derive(Debug)]
struct HideArgs {
    source: String,
    hidden: String,
    inside: String,
}

pub fn run_hide_command(matches: &ArgMatches) -> Result<(), Box<dyn Error>> {
    let args: HideArgs = matches.clone().into();

    let hidden_file: &[u8] = &read_file(&args.hidden)?;
    let hidden_file_length = hidden_file.len() as u64;

    let pwd = read_password()?;
    let pwd_hash: &[u8] = &hash(pwd.as_bytes())?;

		let	hidden_filename = retrieve_filename(&args.hidden)?;
    let hidden_mask = HiddenMask::new(hidden_filename);
    let hidden_data: &[u8] = &[hidden_file, &serialize(&hidden_mask)?].concat();

    let (encoded, nonce) = encode(&pwd_hash, &hidden_data)?;
    let visible_mask: &[u8] = &serialize(&VisibleMask::new(
        encoded.len() as u64,
        hidden_file_length,
        hidden_mask.size(),
        nonce,
    ))?;

    //TODO
    if args.source == args.inside {
        let data: &[u8] = &[&encoded, visible_mask].concat();
        write_to_file(&args.inside, data)?;
    } else {
        let source_file: &[u8] = &read_file(&args.source)?;

        let data: &[u8] = &[source_file, &encoded, visible_mask].concat();
        write_to_file(&args.inside, data)?;
    }

    Ok(())
}

impl From<ArgMatches> for HideArgs {
    fn from(matches: ArgMatches) -> Self {
        let source = matches.get_one::<String>("SOURCE_FILE").cloned().unwrap();
        let hidden = matches.get_one::<String>("HIDDEN_FILE").cloned().unwrap();
        let inside = matches
            .get_one::<String>("HIDE_INSIDE")
            .unwrap_or(&source)
            .clone();

        HideArgs {
            source: source.into(),
            hidden,
            inside,
        }
    }
}

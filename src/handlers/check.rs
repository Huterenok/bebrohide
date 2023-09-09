// cargo run check -r assets/bebra1.txt
use anyhow::Result;
use clap::ArgMatches;

use crate::mask::{VisibleMask, VISIBLE_MASK_SIZE};

use crate::utils::fs::read_file;
use crate::utils::serialize::deserialize;

#[derive(Debug)]
struct CheckArgs {
    hidden: String,
}

pub fn run_check_command(matches: &ArgMatches) -> Result<()> {
    let args: CheckArgs = matches.clone().into();

    let data = read_file(&args.hidden)?;
    let before_mask_length = match data.len().checked_sub(VISIBLE_MASK_SIZE) {
        Some(length) => length,
        None => {
            println!("false");
            return Ok(());
        }
    };

    let visible_mask: VisibleMask = match deserialize(&data[before_mask_length..]) {
        Ok(mask) => mask,
        Err(_) => {
            println!("false");
            return Ok(());
        }
    };

    println!("{}", visible_mask.verify());

    Ok(())
}

impl From<ArgMatches> for CheckArgs {
    fn from(matches: ArgMatches) -> Self {
        let hidden = matches.get_one::<String>("HIDDEN_FILE").cloned().unwrap();

        CheckArgs { hidden }
    }
}

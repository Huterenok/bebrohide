mod cli;
mod mask;
mod handlers;
mod utils;

use std::fmt::Display;

use handlers::{run_check_command, run_hide_command, run_reveal_command};

fn main() {
    let matches = cli::build_cli().get_matches();

    //Matches command name and redirects flow to handling this command
    match matches.subcommand().unwrap() {
        ("hide", matches) => run_hide_command(matches).unwrap_or_else(handle_error),
        ("reveal", matches) => run_reveal_command(matches).unwrap_or_else(handle_error),
        ("check", matches) => run_check_command(matches).unwrap_or_else(handle_error),
        _ => unreachable!("The cli parser should prevent reaching here"),
    }
}

fn handle_error<E: Display, T>(error: E) -> T{
    eprintln!("{error}");
    ::std::process::exit(1);
}
